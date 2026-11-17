//! Small Windows ABI boundary. Buffers and server handles are released on every path.
use crate::models::Event;
use std::{ffi::c_void, path::Path, sync::mpsc::Sender, time::Duration};
type Handle = *mut c_void;
#[repr(C)]
struct Session {
    id: u32,
    station: *mut u16,
    state: i32,
}
#[link(name = "Wtsapi32")]
unsafe extern "system" {
    fn WTSOpenServerW(name: *const u16) -> Handle;
    fn WTSCloseServer(server: Handle);
    fn WTSEnumerateSessionsW(
        server: Handle,
        reserved: u32,
        version: u32,
        buffer: *mut *mut Session,
        count: *mut u32,
    ) -> i32;
    fn WTSQuerySessionInformationW(
        server: Handle,
        id: u32,
        class: i32,
        buffer: *mut *mut u16,
        bytes: *mut u32,
    ) -> i32;
    fn WTSFreeMemory(buffer: Handle);
    fn WTSSendMessageW(
        server: Handle,
        id: u32,
        title: *const u16,
        title_len: u32,
        text: *const u16,
        text_len: u32,
        style: u32,
        timeout: u32,
        response: *mut u32,
        wait: i32,
    ) -> i32;
}
#[link(name = "Netapi32")]
unsafe extern "system" {
    fn NetGetJoinInformation(server: *const u16, name: *mut *mut u16, status: *mut i32) -> u32;
    fn NetApiBufferFree(buffer: Handle) -> u32;
}
#[link(name = "Shell32")]
unsafe extern "system" {
    fn ShellExecuteW(
        hwnd: Handle,
        op: *const u16,
        file: *const u16,
        args: *const u16,
        dir: *const u16,
        show: i32,
    ) -> isize;
}
#[link(name = "Kernel32")]
unsafe extern "system" {
    fn MoveFileExW(from: *const u16, to: *const u16, flags: u32) -> i32;
}
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
unsafe fn read_wide(p: *const u16) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut n = 0;
    unsafe {
        while *p.add(n) != 0 {
            n += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(p, n))
    }
}
struct Server(Handle);
impl Drop for Server {
    fn drop(&mut self) {
        unsafe { WTSCloseServer(self.0) }
    }
}
struct Buffer(Handle);
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { WTSFreeMemory(self.0) }
    }
}
pub fn replace_file(from: &Path, to: &Path) -> Result<(), String> {
    move_file(from, to, true)
}
pub fn move_new_file(from: &Path, to: &Path) -> Result<(), String> {
    move_file(from, to, false)
}
fn move_file(from: &Path, to: &Path, replace: bool) -> Result<(), String> {
    use std::os::windows::ffi::OsStrExt;
    let a: Vec<_> = from.as_os_str().encode_wide().chain(Some(0)).collect();
    let b: Vec<_> = to.as_os_str().encode_wide().chain(Some(0)).collect();
    if unsafe { MoveFileExW(a.as_ptr(), b.as_ptr(), 8 | u32::from(replace)) } == 0 {
        Err(std::io::Error::last_os_error().to_string())
    } else {
        Ok(())
    }
}
pub fn shell_open(file: &str, args: &str) -> Result<String, String> {
    let f = wide(file);
    let a = wide(args);
    let op = wide("open");
    let r = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            op.as_ptr(),
            f.as_ptr(),
            a.as_ptr(),
            std::ptr::null(),
            1,
        )
    };
    if r <= 32 {
        Err(format!("ShellExecute: код {r}"))
    } else {
        Ok("Процесс запущен; результат удалённой операции не проверен".into())
    }
}
pub fn domain(pc: &str, limit: usize) -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static ACTIVE: AtomicUsize = AtomicUsize::new(0);
    if ACTIVE.fetch_add(1, Ordering::SeqCst) >= limit {
        ACTIVE.fetch_sub(1, Ordering::SeqCst);
        return "Домен: предыдущие запросы ещё выполняются".into();
    }
    let (tx, rx) = std::sync::mpsc::channel();
    let pc = pc.to_owned();
    std::thread::spawn(move || {
        let name = wide(&pc);
        let mut p = std::ptr::null_mut();
        let mut status = 0;
        let result = unsafe { NetGetJoinInformation(name.as_ptr(), &mut p, &mut status) };
        let text = if result != 0 {
            format!(
                "Ошибка домена: {}",
                std::io::Error::from_raw_os_error(result as i32)
            )
        } else {
            let s = unsafe { read_wide(p) };
            match status {
                3 => s,
                2 => format!("Рабочая группа: {s}"),
                1 => "Не присоединён".into(),
                _ => "Неизвестно".into(),
            }
        };
        if !p.is_null() {
            unsafe {
                NetApiBufferFree(p.cast());
            }
        }
        ACTIVE.fetch_sub(1, Ordering::SeqCst);
        let _ = tx.send(text);
    });
    rx.recv_timeout(Duration::from_secs(5))
        .unwrap_or_else(|_| "Домен: нет ответа (5 сек.)".into())
}
pub fn send_message(pc: &str, text: &str, seconds: u32, tx: &Sender<Event>) -> Result<(), String> {
    if !crate::services::valid_target(pc)
        || text.trim().is_empty()
        || text.contains('\0')
        || text.encode_utf16().count() > 2000
        || !(1..=3600).contains(&seconds)
    {
        return Err("Некорректная цель, текст или время сообщения".into());
    }
    let pcw = wide(pc);
    let server = Server(unsafe { WTSOpenServerW(pcw.as_ptr()) });
    if server.0.is_null() {
        return Err(std::io::Error::last_os_error().to_string());
    }
    let mut p = std::ptr::null_mut();
    let mut count = 0;
    if unsafe { WTSEnumerateSessionsW(server.0, 0, 1, &mut p, &mut count) } == 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    let _buffer = Buffer(p.cast());
    let mut recipients = vec![];
    if count > 0 && !p.is_null() {
        for session in unsafe { std::slice::from_raw_parts(p, count as usize) } {
            if !matches!(session.state, 0 | 4) {
                continue;
            }
            let mut user = std::ptr::null_mut();
            let mut bytes = 0;
            if unsafe {
                WTSQuerySessionInformationW(server.0, session.id, 5, &mut user, &mut bytes)
            } == 0
            {
                return Err(std::io::Error::last_os_error().to_string());
            }
            let _user_buffer = Buffer(user.cast());
            let name = unsafe { read_wide(user) };
            if !name.trim().is_empty() {
                recipients.push((session.id, name));
            }
        }
    }
    if recipients.is_empty() {
        return Err("Пользовательские сеансы не найдены".into());
    }
    // Dispatcher joins all sends before Server drops; workers borrow text and the opaque handle.
    let handle = server.0 as usize;
    crate::services::dispatch_sessions(pc, recipients, seconds, tx, |id| {
        let title = wide("Сообщение от службы поддержки");
        let body = wide(text);
        let mut response = 0;
        let ok = unsafe {
            WTSSendMessageW(
                handle as Handle,
                id,
                title.as_ptr(),
                ((title.len() - 1) * 2) as u32,
                body.as_ptr(),
                ((body.len() - 1) * 2) as u32,
                0x41,
                seconds,
                &mut response,
                1,
            )
        };
        if ok == 0 {
            Err(std::io::Error::last_os_error().to_string())
        } else {
            Ok(response)
        }
    });
    Ok(())
}

pub fn ping(target: &str, timeout: u32) -> crate::models::PingReply {
    use crate::models::PingReply;
    use std::net::{IpAddr, ToSocketAddrs};
    use windows_sys::Win32::{NetworkManagement::IpHelper::*, Networking::WinSock::*};
    let ip = match target
        .parse::<IpAddr>()
        .ok()
        .or_else(|| (target, 0).to_socket_addrs().ok()?.next().map(|s| s.ip()))
    {
        Some(ip) => ip,
        None => return PingReply::failure("Ошибка DNS"),
    };
    let payload = [0u8; 32];
    // u64 allocation provides sufficient size and alignment for either reply structure.
    let mut reply = [0u64; 128];
    let handle = unsafe {
        if ip.is_ipv4() {
            IcmpCreateFile()
        } else {
            Icmp6CreateFile()
        }
    };
    if handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
        return PingReply::failure(std::io::Error::last_os_error().to_string());
    }
    let count = unsafe {
        match ip {
            IpAddr::V4(ip) => IcmpSendEcho(
                handle,
                u32::from_ne_bytes(ip.octets()),
                payload.as_ptr().cast(),
                payload.len() as u16,
                std::ptr::null(),
                reply.as_mut_ptr().cast(),
                std::mem::size_of_val(&reply) as u32,
                timeout,
            ),
            IpAddr::V6(ip) => {
                let mut source: SOCKADDR_IN6 = std::mem::zeroed();
                source.sin6_family = AF_INET6;
                let mut destination = source;
                destination.sin6_addr.u.Byte = ip.octets();
                Icmp6SendEcho2(
                    handle,
                    std::ptr::null_mut(),
                    None,
                    std::ptr::null(),
                    &source,
                    &destination,
                    payload.as_ptr().cast(),
                    payload.len() as u16,
                    std::ptr::null(),
                    reply.as_mut_ptr().cast(),
                    std::mem::size_of_val(&reply) as u32,
                    timeout,
                )
            }
        }
    };
    let error = if count == 0 {
        Some(std::io::Error::last_os_error())
    } else {
        None
    };
    let (status, rtt) = unsafe {
        if ip.is_ipv4() {
            let r = &*(reply.as_ptr().cast::<ICMP_ECHO_REPLY>());
            (r.Status, r.RoundTripTime)
        } else {
            let r = &*(reply.as_ptr().cast::<ICMPV6_ECHO_REPLY_LH>());
            (r.Status, r.RoundTripTime)
        }
    };
    unsafe {
        IcmpCloseHandle(handle);
    }
    if let Some(e) = error {
        PingReply::failure(format!("Нет ответа: {e}"))
    } else if status == 0 {
        PingReply::success(rtt)
    } else {
        PingReply::failure(format!("ICMP: код {status}"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn loopback_echo() {
        let reply = super::ping("127.0.0.1", 1000);
        assert!(reply.online);
        assert!(reply.round_trip_ms.is_some());
    }
}

#[link(name = "User32")]
unsafe extern "system" {
    fn MessageBoxW(hwnd: Handle, text: *const u16, title: *const u16, kind: u32) -> i32;
}
pub fn notice(title: &str, text: &str, error: bool) {
    let title = wide(title);
    let text = wide(text);
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            title.as_ptr(),
            if error { 0x10 } else { 0x40 },
        );
    }
}

#[link(name = "User32")]
unsafe extern "system" {
    fn EnumWindows(callback: unsafe extern "system" fn(Handle, isize) -> i32, param: isize) -> i32;
    fn GetWindowThreadProcessId(hwnd: Handle, pid: *mut u32) -> u32;
    fn GetWindowTextW(hwnd: Handle, text: *mut u16, max: i32) -> i32;
    fn EnableWindow(hwnd: Handle, enabled: i32) -> i32;
    fn SetWindowLongPtrW(hwnd: Handle, index: i32, value: isize) -> isize;
}
struct WindowSearch<'a> {
    title: &'a str,
    found: Handle,
}
unsafe extern "system" fn find_owned_window(hwnd: Handle, param: isize) -> i32 {
    // EnumWindows invokes synchronously; search points to a stack value for this call only.
    let search = unsafe { &mut *(param as *mut WindowSearch<'_>) };
    let mut pid = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, &mut pid);
    }
    if pid != std::process::id() {
        return 1;
    }
    let mut title = [0u16; 512];
    let count = unsafe { GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32) };
    if count > 0 && String::from_utf16_lossy(&title[..count as usize]) == search.title {
        search.found = hwnd;
        0
    } else {
        1
    }
}
fn own_window(title: &str) -> Handle {
    let mut search = WindowSearch {
        title,
        found: std::ptr::null_mut(),
    };
    unsafe {
        EnumWindows(
            find_owned_window,
            (&mut search as *mut WindowSearch<'_>) as isize,
        );
    }
    search.found
}
pub fn set_dialog_owner(title: Option<&str>) {
    let parent = own_window("Quick Remote Toolkit");
    if parent.is_null() {
        return;
    }
    if let Some(title) = title {
        let child = own_window(title);
        if !child.is_null() {
            unsafe {
                SetWindowLongPtrW(child, -8, parent as isize);
                EnableWindow(parent, 0);
            }
        }
    } else {
        unsafe {
            EnableWindow(parent, 1);
        }
    }
}

pub fn default_csv_path() -> Option<std::path::PathBuf> {
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::{
        System::Com::CoTaskMemFree,
        UI::Shell::{FOLDERID_Documents, SHGetKnownFolderPath},
    };
    let mut p = std::ptr::null_mut();
    let result =
        unsafe { SHGetKnownFolderPath(&FOLDERID_Documents, 0, std::ptr::null_mut(), &mut p) };
    if result < 0 || p.is_null() {
        return None;
    }
    let mut n = 0;
    unsafe {
        while *p.add(n) != 0 {
            n += 1;
        }
    }
    let path = std::path::PathBuf::from(std::ffi::OsString::from_wide(unsafe {
        std::slice::from_raw_parts(p, n)
    }));
    unsafe {
        CoTaskMemFree(p.cast());
    }
    let csv = path.join("GitHub/Quick-Remote-Toolkit/QuickRemoteToolkit.clients.csv");
    csv.is_file().then_some(csv)
}

#[link(name = "Kernel32")]
unsafe extern "system" {
    fn GetOEMCP() -> u32;
    fn MultiByteToWideChar(
        code_page: u32,
        flags: u32,
        bytes: *const u8,
        len: i32,
        out: *mut u16,
        capacity: i32,
    ) -> i32;
}
fn decode_codepage(bytes: &[u8], codepage: u32) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    let Ok(len) = i32::try_from(bytes.len()) else {
        return "Вывод слишком велик".into();
    };
    let n =
        unsafe { MultiByteToWideChar(codepage, 0, bytes.as_ptr(), len, std::ptr::null_mut(), 0) };
    if n <= 0 {
        return String::from_utf8_lossy(bytes).into_owned();
    }
    let mut wide = vec![0u16; n as usize];
    let n = unsafe { MultiByteToWideChar(codepage, 0, bytes.as_ptr(), len, wide.as_mut_ptr(), n) };
    if n <= 0 {
        String::from_utf8_lossy(bytes).into_owned()
    } else {
        String::from_utf16_lossy(&wide[..n as usize])
    }
}
pub fn decode_oem(bytes: &[u8]) -> String {
    decode_codepage(bytes, unsafe { GetOEMCP() })
}
#[cfg(test)]
mod encoding_tests {
    #[test]
    fn oem_cyrillic_diagnostics() {
        let (bytes, _, _) = encoding_rs::IBM866.encode("Ошибка доступа");
        assert_eq!(super::decode_codepage(&bytes, 866), "Ошибка доступа");
    }
}
