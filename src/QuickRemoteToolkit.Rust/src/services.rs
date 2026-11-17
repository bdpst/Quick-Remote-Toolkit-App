use crate::models::{Client, Event, PingReply, Settings};
use base64::Engine;
use std::os::windows::process::CommandExt;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    sync::mpsc::Sender,
    time::{Duration, Instant},
};

pub fn settings_path() -> PathBuf {
    PathBuf::from(std::env::var_os("APPDATA").unwrap_or_default())
        .join("QuickRemoteToolkitApp")
        .join("settings.json")
}
pub fn load_settings() -> Result<Settings, String> {
    let path = settings_path();
    if path.exists() {
        load_settings_at(&path)
    } else {
        Ok(Settings {
            clients_csv_path: crate::native::default_csv_path()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default(),
            ..Default::default()
        })
    }
}
fn load_settings_at(p: &std::path::Path) -> Result<Settings, String> {
    if !p.exists() {
        return Ok(Settings::default());
    }
    let bytes = fs::read(p).map_err(|e| e.to_string())?;
    let bytes = bytes.strip_prefix(&[0xef, 0xbb, 0xbf]).unwrap_or(&bytes);
    let s: Settings = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    Ok(s)
}
pub fn save_settings(s: &Settings) -> Result<(), String> {
    save_settings_at(s, &settings_path())
}
fn save_settings_at(s: &Settings, p: &std::path::Path) -> Result<(), String> {
    use std::io::Write;
    fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
    let tmp = p.with_extension(format!(
        "{}.{}.tmp",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    ));
    let data = serde_json::to_vec_pretty(s).map_err(|e| e.to_string())?;
    let mut owned = false;
    let result = (|| {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)
            .map_err(|e| e.to_string())?;
        owned = true;
        file.write_all(&data).map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        drop(file);
        let backup = p.with_file_name("settings.before-rust.json");
        if backup.exists() {
            load_settings_at(&backup)
                .map_err(|e| format!("Резервная копия настроек повреждена: {e}"))?;
        } else if p.exists() {
            let original = fs::read(p).map_err(|e| e.to_string())?;
            let bytes = original
                .strip_prefix(&[0xef, 0xbb, 0xbf])
                .unwrap_or(&original);
            serde_json::from_slice::<Settings>(bytes)
                .map_err(|e| format!("Исходные настройки изменены или повреждены: {e}"))?;
            let backup_tmp = tmp.with_extension("backup.tmp");
            let mut own_backup = false;
            let backup_result = (|| {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&backup_tmp)
                    .map_err(|e| e.to_string())?;
                own_backup = true;
                file.write_all(&original).map_err(|e| e.to_string())?;
                file.sync_all().map_err(|e| e.to_string())?;
                drop(file);
                match crate::native::move_new_file(&backup_tmp, &backup) {
                    Ok(()) => {
                        own_backup = false;
                        Ok(())
                    }
                    Err(_) if backup.exists() => load_settings_at(&backup)
                        .map(|_| ())
                        .map_err(|e| e.to_string()),
                    Err(e) => Err(e),
                }
            })();
            if own_backup {
                let _ = fs::remove_file(&backup_tmp);
            }
            backup_result?;
        }
        crate::native::replace_file(&tmp, p)
    })();
    if owned && result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

pub fn parse_csv(bytes: &[u8]) -> Vec<Client> {
    let text = match std::str::from_utf8(bytes) {
        Ok(s) => s.to_owned(),
        Err(_) => encoding_rs::WINDOWS_1251.decode(bytes).0.into_owned(),
    };
    let mut clients = Vec::new();
    for line in text.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let p: Vec<_> = line.split(';').collect();
        if p.len() < 3 {
            continue;
        }
        clients.push(Client {
            number: p[0].trim().parse().unwrap_or(clients.len() as i32 + 1),
            computer: p[1].trim().into(),
            ip: p[2].trim().into(),
            person: p.get(3).unwrap_or(&"").trim().into(),
            domain: "—".into(),
            ..Default::default()
        });
    }
    clients
}
pub fn load_csv(path: &str) -> Result<Vec<Client>, String> {
    fs::read(path)
        .map(|b| parse_csv(&b))
        .map_err(|e| e.to_string())
}
pub fn valid_target(s: &str) -> bool {
    !s.is_empty()
        && s.encode_utf16().count() <= 255
        && !s.starts_with('-')
        && s.chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '-' | '_' | ':'))
}
fn hidden(program: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut c = Command::new(program);
    c.creation_flags(0x08000000);
    c
}
fn launch(program: &str, args: &[String]) -> Result<String, String> {
    Command::new(program)
        .args(args)
        .spawn()
        .map(|_| "Процесс запущен; результат удалённой операции не проверен".into())
        .map_err(|e| e.to_string())
}
fn ps_literal(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}
pub fn powershell(script: &str, timeout: u64) -> Result<String, String> {
    let bytes: Vec<u8> = script.encode_utf16().flat_map(u16::to_le_bytes).collect();
    // Files avoid pipe-buffer deadlocks while waiting for a child. They contain output only, never the command/message.
    let id = format!(
        "qrt-{}-{}",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
    );
    let out = std::env::temp_dir().join(format!("{id}.out"));
    let err = out.with_extension("err");
    let (mut owned_out, mut owned_err) = (false, false);
    let result = (|| {
        let stdout = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&out)
            .map_err(|e| e.to_string())?;
        owned_out = true;
        let stderr = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&err)
            .map_err(|e| e.to_string())?;
        owned_err = true;
        let mut child = hidden(
            PathBuf::from(std::env::var_os("SystemRoot").unwrap_or("C:\\Windows".into()))
                .join("System32/WindowsPowerShell/v1.0/powershell.exe"),
        )
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-EncodedCommand",
            &base64::engine::general_purpose::STANDARD.encode(bytes),
        ])
        .stdout(stdout)
        .stderr(stderr)
        .stdin(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
        let start = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
                break status;
            }
            if start.elapsed() > Duration::from_secs(timeout) {
                let _ = child.kill();
                let _ = child.wait();
                return Err(
                    "Тайм-аут. Результат неизвестен: проверьте состояние на ПК перед повтором."
                        .into(),
                );
            }
            std::thread::sleep(Duration::from_millis(50));
        };
        let output = fs::read_to_string(&out).unwrap_or_default();
        let error = fs::read_to_string(&err).unwrap_or_default();
        if status.success() {
            Ok(output.trim().into())
        } else {
            Err(format!("Код {status}: {} {}", error.trim(), output.trim()))
        }
    })();
    if owned_out {
        let _ = fs::remove_file(out);
    }
    if owned_err {
        let _ = fs::remove_file(err);
    }
    result
}
pub fn valid_member(member: &str) -> bool {
    let member = member.trim();
    let parts: Vec<_> = member.split(char::from(92)).collect();
    parts.len() == 2
        && parts.iter().all(|s| !s.is_empty())
        && !member.chars().any(char::is_control)
        && member.encode_utf16().count() <= 256
}
pub fn administrators(computer: &str, member: &str, add: bool) -> Result<String, String> {
    if !valid_target(computer) || !valid_member(member) {
        return Err("Некорректный ПК или учётная запись".into());
    }
    let cmd = if add {
        "Add-LocalGroupMember"
    } else {
        "Remove-LocalGroupMember"
    };
    powershell(
        &format!(
            "$ErrorActionPreference='Stop'; [Console]::OutputEncoding=[Text.Encoding]::UTF8; try {{ Invoke-Command -ComputerName {} -ArgumentList {} -ErrorAction Stop -ScriptBlock {{ param([string]$Member); $ErrorActionPreference='Stop'; $g=Get-LocalGroup -SID 'S-1-5-32-544' -ErrorAction Stop; {cmd} -Group $g.Name -Member $Member -ErrorAction Stop }}; 'Членство изменено'; exit 0 }} catch {{ [Console]::Error.WriteLine($_.Exception.Message); exit 1 }}",
            ps_literal(computer),
            ps_literal(member)
        ),
        60,
    )
}
pub const ACTIONS: &[&str] = &[
    "Remote Assistance",
    "Ping",
    "Tracert",
    "Open C$",
    "Event Viewer",
    "Computer Management",
    "MSTSC",
    "WinRS cmd",
    "gpupdate",
];
trait ActionAdapter {
    fn launch(&self, program: &str, args: &[String]) -> Result<String, String>;
    fn shell_open(&self, file: &str, args: &str) -> Result<String, String>;
    fn smb(&self, pc: &str, user: &str) -> Result<String, String>;
}
struct SystemActions;
impl ActionAdapter for SystemActions {
    fn launch(&self, program: &str, args: &[String]) -> Result<String, String> {
        launch(program, args)
    }
    fn shell_open(&self, file: &str, args: &str) -> Result<String, String> {
        crate::native::shell_open(file, args)
    }
    fn smb(&self, pc: &str, user: &str) -> Result<String, String> {
        let mut child = hidden("net.exe")
            .args(["use", &format!("\\\\{pc}\\IPC$"), &format!("/user:{user}")])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        let start = Instant::now();
        let smb = loop {
            if let Some(s) = child.try_wait().map_err(|e| e.to_string())? {
                break format!("SMB: {s}");
            }
            if start.elapsed() > Duration::from_secs(5) {
                let _ = child.kill();
                let _ = child.wait();
                break "SMB: тайм-аут".into();
            }
            std::thread::sleep(Duration::from_millis(50));
        };
        use std::io::Read;
        let mut output = Vec::new();
        let mut error = Vec::new();
        if let Some(mut pipe) = child.stdout.take() {
            let _ = pipe.read_to_end(&mut output);
        }
        if let Some(mut pipe) = child.stderr.take() {
            let _ = pipe.read_to_end(&mut error);
        }
        Ok(format!(
            "{smb}; {} {}",
            crate::native::decode_oem(&output).trim(),
            crate::native::decode_oem(&error).trim()
        ))
    }
}
pub fn action(c: &Client, action: usize) -> Result<String, String> {
    action_with(c, action, &SystemActions)
}
fn action_with(c: &Client, action: usize, adapter: &impl ActionAdapter) -> Result<String, String> {
    if !valid_target(if matches!(action, 1 | 2) {
        c.target()
    } else {
        &c.computer
    }) {
        return Err("Некорректное имя ПК/IP: допустимы буквы, цифры, точки, дефисы, подчёркивания и двоеточия".into());
    }
    let pc = &c.computer;
    let cmd = std::env::var("ComSpec").unwrap_or("cmd.exe".into());
    match action {
        0 => adapter.launch("msra.exe", &["/offerra".into(), pc.clone()]),
        1 => adapter.launch(
            &cmd,
            &["/d".into(), "/k".into(), format!("ping {}", c.target())],
        ),
        2 => adapter.launch(
            &cmd,
            &[
                "/d".into(),
                "/c".into(),
                format!("tracert {} & echo. & pause", c.target()),
            ],
        ),
        3 => {
            let user = format!(
                "{}\\{}",
                std::env::var("USERDOMAIN").unwrap_or_default(),
                std::env::var("USERNAME").unwrap_or_default()
            );
            let smb = adapter.smb(pc, &user)?;
            adapter
                .launch("explorer.exe", &[format!("\\\\{pc}\\c$")])
                .map(|s| format!("{s}; {smb}"))
        }
        4 => adapter.shell_open("eventvwr.msc", &format!("/computer:{pc}")),
        5 => adapter.shell_open("compmgmt.msc", &format!("/computer:\\\\{pc}")),
        6 => adapter.launch("mstsc.exe", &[format!("/v:{pc}")]),
        7 => adapter.launch(
            &cmd,
            &["/d".into(), "/k".into(), format!("winrs -r:{pc} cmd")],
        ),
        8 => adapter.launch(
            &cmd,
            &[
                "/d".into(),
                "/c".into(),
                format!("winrs -r:{pc} gpupdate /force & echo. & pause"),
            ],
        ),
        _ => Err("Неизвестная команда".into()),
    }
}
trait ProbeAdapter: Sync {
    fn ping(&self, target: &str, timeout: u32) -> PingReply;
    fn domain(&self, computer: &str, limit: usize) -> String;
}
struct NativeProbe;
impl ProbeAdapter for NativeProbe {
    fn ping(&self, target: &str, timeout: u32) -> PingReply {
        crate::native::ping(target, timeout)
    }
    fn domain(&self, pc: &str, limit: usize) -> String {
        crate::native::domain(pc, limit)
    }
}
pub fn probe(
    clients: Vec<(usize, Client)>,
    settings: Settings,
    tx: Sender<Event>,
    generation: u64,
) {
    probe_with(clients, settings, tx, &NativeProbe, generation);
}
fn probe_with(
    clients: Vec<(usize, Client)>,
    settings: Settings,
    tx: Sender<Event>,
    adapter: &impl ProbeAdapter,
    generation: u64,
) {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    let clients = Arc::new(clients);
    let next = Arc::new(AtomicUsize::new(0));
    let limit = settings.parallel_ping_limit.clamp(1, 128) as usize;
    std::thread::scope(|scope| {
        for _ in 0..limit.min(clients.len()) {
            let clients = clients.clone();
            let next = next.clone();
            let tx = tx.clone();
            scope.spawn(move || {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some((client_index, c)) = clients.get(i) else {
                        break;
                    };
                    let _ = tx.send(Event::ProbeStarted(
                        generation,
                        *client_index,
                        chrono::Local::now().format("%d.%m.%Y %H:%M:%S").to_string(),
                    ));
                    let reply = if !valid_target(c.target()) {
                        PingReply::failure("Некорректная цель")
                    } else if settings.ping_timeout_ms < 0 {
                        PingReply::failure("PingTimeoutMs не может быть отрицательным.")
                    } else {
                        adapter.ping(c.target(), settings.ping_timeout_ms as u32)
                    };
                    let result = reply
                        .round_trip_ms
                        .map(|ms| format!("{ms} ms"))
                        .unwrap_or(reply.diagnostic);
                    let _ = tx.send(Event::Log(c.computer.clone(), "Ping".into(), result));
                    let domain = if reply.online {
                        let _ = tx.send(Event::Probe(
                            generation,
                            *client_index,
                            "Доступен".into(),
                            "Проверка…".into(),
                        ));
                        adapter.domain(&c.computer, limit)
                    } else {
                        String::new()
                    };
                    if domain.starts_with("Ошибка домена:") || domain.starts_with("Домен:")
                    {
                        let _ = tx.send(Event::Log(
                            c.computer.clone(),
                            "Domain".into(),
                            domain.clone(),
                        ));
                    }
                    let label = if reply.online {
                        "Доступен"
                    } else {
                        "Не отвечает"
                    };
                    let domain = if reply.online {
                        if domain.starts_with("Ошибка домена:") {
                            "Ошибка запроса".into()
                        } else if domain.starts_with("Домен:") {
                            "Нет ответа".into()
                        } else if domain == "Рабочая группа: " {
                            "Рабочая группа".into()
                        } else {
                            domain
                        }
                    } else {
                        "Недоступно".into()
                    };
                    let _ = tx.send(Event::Probe(
                        generation,
                        *client_index,
                        label.into(),
                        domain,
                    ));
                }
            });
        }
    });
    let _ = tx.send(Event::ProbeDone(generation));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn csv_encodings() {
        let s = "number;computer;ip;person\n1;PC-TEST;192.0.2.1;Иван\n";
        let (b, _, _) = encoding_rs::WINDOWS_1251.encode(s);
        assert_eq!(parse_csv(s.as_bytes())[0].person, parse_csv(&b)[0].person);
    }
    #[test]
    fn shell_targets() {
        for s in ["pc & whoami", "pc\"", "-n", "pc\nfoo", ""] {
            assert!(!valid_target(s));
        }
        assert!(valid_target("PC-TEST.example"));
        assert!(valid_target("2001:db8::1"));
    }
    #[test]
    fn ps_escaping() {
        assert_eq!(ps_literal("DOMAIN\\O'Neil"), "'DOMAIN\\O''Neil'");
    }
}

#[cfg(test)]
mod parity_tests {
    use super::*;
    #[test]
    fn csv_contract_empty_invalid_zero_and_negative() {
        assert!(parse_csv(b"").is_empty());
        assert!(parse_csv(b"header\nwrong").is_empty());
        let c = parse_csv(
            "number;computer;ip;person\n0; PC-0 ;-;Имя\nbad;PC-2;;Другой\n-2147483648;PC-3;;"
                .as_bytes(),
        );
        assert_eq!(c[0].number, 0);
        assert_eq!(c[1].number, 2);
        assert_eq!(c[2].number, i32::MIN);
        assert_eq!(c[0].target(), "PC-0");
    }
}

pub fn export_logs(logs: &[crate::models::Log], csv: bool) -> String {
    fn quote(s: &str) -> String {
        format!("\"{}\"", s.replace('"', "\"\""))
    }
    let mut out = String::new();
    for l in logs {
        if csv {
            out.push_str(&format!(
                "{};{};{};{}\r\n",
                quote(&l.time),
                quote(&l.target),
                quote(&l.action),
                quote(&l.result)
            ));
        } else {
            out.push_str(&format!(
                "[{}] {} | {} | {}\r\n",
                l.time, l.target, l.action, l.result
            ));
        }
    }
    out
}
#[cfg(test)]
mod file_tests {
    use super::*;
    #[test]
    fn wpf_bom_roundtrip_atomic_backup_and_invalid_input() {
        let dir = std::env::temp_dir().join(format!(
            "qrt-settings-test-{}-{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap()
        ));
        fs::create_dir_all(&dir).unwrap();
        let p = dir.join("settings.json");
        let original=b"\xef\xbb\xbf{\"IsDarkTheme\":true,\"FavoriteComputers\":[\"computer:PC-1\"],\"FutureSetting\":17}";
        fs::write(&p, original).unwrap();
        let mut s = load_settings_at(&p).unwrap();
        assert!(s.is_dark_theme);
        s.show_favorites_only = true;
        save_settings_at(&s, &p).unwrap();
        assert_eq!(
            fs::read(dir.join("settings.before-rust.json")).unwrap(),
            original
        );
        let s = load_settings_at(&p).unwrap();
        assert!(s.show_favorites_only);
        assert_eq!(s.extra["FutureSetting"], 17);
        assert!(serde_json::from_slice::<Settings>(&fs::read(&p).unwrap()).is_ok());
        fs::write(&p, b"{broken").unwrap();
        assert!(load_settings_at(&p).is_err());
        assert!(save_settings_at(&s, &dir).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn export_contract_and_escaping() {
        let logs = vec![crate::models::Log {
            time: "2026-11-16 12:00:00".into(),
            target: "PC-1".into(),
            action: "Test".into(),
            result: "a;\"b\"\nnext".into(),
        }];
        assert!(export_logs(&logs, false).starts_with("[2026-11-16 12:00:00] PC-1 | Test | "));
        assert!(export_logs(&logs, true).contains("\"a;\"\"b\"\"\nnext\""));
        assert!(export_logs(&[], true).is_empty());
    }
}

#[cfg(test)]
mod operation_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct Mock {
        active: AtomicUsize,
        max: AtomicUsize,
        calls: AtomicUsize,
    }
    impl ProbeAdapter for Mock {
        fn ping(&self, target: &str, timeout: u32) -> PingReply {
            assert!(target.starts_with("192.0.2."));
            assert_eq!(timeout, 77);
            let n = self.active.fetch_add(1, Ordering::SeqCst) + 1;
            self.max.fetch_max(n, Ordering::SeqCst);
            std::thread::sleep(Duration::from_millis(3));
            self.active.fetch_sub(1, Ordering::SeqCst);
            self.calls.fetch_add(1, Ordering::SeqCst);
            if target.ends_with(".2") {
                PingReply::failure("Тайм-аут")
            } else {
                PingReply::success(12)
            }
        }
        fn domain(&self, _: &str, _: usize) -> String {
            "Рабочая группа: TEST".into()
        }
    }
    #[test]
    fn limited_workers_and_selected_indices_with_errors() {
        let m = Mock {
            active: AtomicUsize::new(0),
            max: AtomicUsize::new(0),
            calls: AtomicUsize::new(0),
        };
        let clients = (1..=16)
            .map(|i| {
                (
                    i + 10,
                    Client {
                        computer: format!("PC-{i}"),
                        ip: format!("192.0.2.{i}"),
                        ..Default::default()
                    },
                )
            })
            .collect();
        let (tx, rx) = std::sync::mpsc::channel();
        let settings = Settings {
            parallel_ping_limit: 3,
            ping_timeout_ms: 77,
            ..Default::default()
        };
        probe_with(clients, settings, tx, &m, 42);
        assert_eq!(m.calls.load(Ordering::SeqCst), 16);
        assert!(m.max.load(Ordering::SeqCst) <= 3);
        let events: Vec<_> = rx.try_iter().collect();
        assert!(
            events
                .iter()
                .any(|e| matches!(e,Event::Log(_,a,r) if a=="Ping" && r=="12 ms"))
        );
        assert!(
            events
                .iter()
                .any(|e| matches!(e,Event::Probe(42,12,s,d) if s=="Не отвечает"&&d=="Недоступно"))
        );
        assert!(matches!(events.last(), Some(Event::ProbeDone(42))));
        let (tx, rx) = std::sync::mpsc::channel();
        drop(rx);
        probe_with(
            vec![(
                77,
                Client {
                    computer: "PC-1".into(),
                    ip: "192.0.2.1".into(),
                    ..Default::default()
                },
            )],
            Settings {
                ping_timeout_ms: 77,
                ..Default::default()
            },
            tx,
            &m,
            42,
        );
    }
    #[test]
    fn account_validation() {
        for s in [
            "",
            r"user",
            r"\user",
            r"DOMAIN\",
            r"DOMAIN\user\x",
            "DOMAIN\\us\ner",
        ] {
            assert!(!valid_member(s), "{s}");
        }
        assert!(valid_member(r" ДОМЕН\Пользователь "));
    }
}

#[cfg(test)]
mod action_tests {
    use super::*;
    use std::sync::Mutex;
    #[derive(Default)]
    struct Mock {
        calls: Mutex<Vec<(String, Vec<String>)>>,
    }
    impl ActionAdapter for Mock {
        fn launch(&self, p: &str, a: &[String]) -> Result<String, String> {
            self.calls.lock().unwrap().push((p.into(), a.to_vec()));
            Ok("Запущено".into())
        }
        fn shell_open(&self, p: &str, a: &str) -> Result<String, String> {
            self.launch(p, &[a.into()])
        }
        fn smb(&self, pc: &str, _: &str) -> Result<String, String> {
            self.launch("SMB", &[pc.into()])
        }
    }
    #[test]
    fn every_command_uses_selected_target_without_execution() {
        let mock = Mock::default();
        let c = Client {
            computer: "ПК-ТЕСТ".into(),
            ip: "192.0.2.17".into(),
            ..Default::default()
        };
        for i in 0..ACTIONS.len() {
            action_with(&c, i, &mock).unwrap();
        }
        let calls = mock.calls.lock().unwrap();
        assert!(
            calls.iter().any(|(p, a)| p == "msra.exe"
                && a == &vec!["/offerra".to_string(), "ПК-ТЕСТ".to_string()])
        );
        assert!(calls.iter().any(|(p, a)| p == "SMB" && a[0] == "ПК-ТЕСТ"));
        assert!(
            calls
                .iter()
                .any(|(_, a)| a.iter().any(|s| s.contains("tracert 192.0.2.17")))
        );
        drop(calls);
        let computer_only = Client {
            computer: "PC-VALID".into(),
            ip: "bad & unused".into(),
            ..Default::default()
        };
        for action in [0, 3, 4, 5, 6, 7, 8] {
            assert!(action_with(&computer_only, action, &mock).is_ok());
        }
        assert!(action_with(&computer_only, 2, &mock).is_err());
        for pc in ["pc&whoami", "pc%TEMP%", "pc\"", "pc user", "pc|x"] {
            let invalid = Client {
                computer: pc.into(),
                ip: "192.0.2.1".into(),
                ..Default::default()
            };
            assert!(action_with(&invalid, 0, &mock).is_err());
        }
    }
}

pub fn dispatch_sessions(
    pc: &str,
    recipients: Vec<(u32, String)>,
    seconds: u32,
    tx: &Sender<Event>,
    send: impl Fn(u32) -> Result<u32, String> + Sync,
) {
    std::thread::scope(|scope| {
        for (id, user) in recipients {
            let send = &send;
            scope.spawn(move || {
                let recipient = format!("{user} (сеанс {id})");
                let _ = tx.send(Event::Log(
                    pc.into(),
                    "Сообщение".into(),
                    format!("{recipient}: отправка и ожидание ответа, до {seconds} сек."),
                ));
                let result = match send(id) {
                    Ok(1) => "Пользователь нажал ОК.".into(),
                    Ok(2) => "Пользователь закрыл сообщение без подтверждения (Отмена).".into(),
                    Ok(32000) => "Время ожидания истекло.".into(),
                    Ok(n) => format!("Получен ответ Windows: {n}."),
                    Err(e) => format!("Ошибка отправки: {e}"),
                };
                let _ = tx.send(Event::Log(
                    pc.into(),
                    "Сообщение".into(),
                    format!("{recipient}: {result}"),
                ));
            });
        }
    });
}
#[cfg(test)]
mod message_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    #[test]
    fn session_responses_are_parallel_distinct_and_do_not_log_body() {
        let (tx, rx) = std::sync::mpsc::channel();
        let active = AtomicUsize::new(0);
        let max = AtomicUsize::new(0);
        dispatch_sessions(
            "PC-DEMO",
            (1..=5).map(|id| (id, format!("User-{id}"))).collect(),
            300,
            &tx,
            |id| {
                let n = active.fetch_add(1, Ordering::SeqCst) + 1;
                max.fetch_max(n, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(5));
                active.fetch_sub(1, Ordering::SeqCst);
                match id {
                    1 => Ok(1),
                    2 => Ok(2),
                    3 => Ok(32000),
                    4 => Ok(99),
                    _ => Err("код 5".into()),
                }
            },
        );
        drop(tx);
        let logs: Vec<_> = rx
            .iter()
            .filter_map(|e| {
                if let Event::Log(_, _, r) = e {
                    Some(r)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(logs.len(), 10);
        assert!(max.load(Ordering::SeqCst) > 1);
        for text in [
            "Пользователь нажал ОК.",
            "без подтверждения",
            "Время ожидания истекло.",
            "ответ Windows: 99",
            "код 5",
        ] {
            assert!(logs.iter().any(|l| l.contains(text)));
        }
        assert!(logs.iter().all(|l| !l.contains("PRIVATE MESSAGE BODY")));
    }
    #[test]
    fn powershell_timeout_is_unknown_without_remote_effect() {
        let error = powershell("Start-Sleep -Seconds 5", 0).unwrap_err();
        assert!(error.contains("Результат неизвестен"));
    }
}

#[cfg(test)]
mod probe_phase_tests {
    use super::*;
    struct GateProbe(std::sync::Mutex<std::sync::mpsc::Receiver<()>>);
    impl ProbeAdapter for GateProbe {
        fn ping(&self, _: &str, _: u32) -> PingReply {
            PingReply::success(7)
        }
        fn domain(&self, _: &str, _: usize) -> String {
            let _ = self.0.lock().unwrap().recv_timeout(Duration::from_secs(5));
            "WORKGROUP".into()
        }
    }
    #[test]
    fn online_and_rtt_are_visible_before_domain_completes() {
        let (release, gate) = std::sync::mpsc::channel();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::scope(|scope| {
            scope.spawn(move || {
                probe_with(
                    vec![(
                        0,
                        Client {
                            computer: "PC-DEMO".into(),
                            ip: "192.0.2.1".into(),
                            ..Default::default()
                        },
                    )],
                    Settings::default(),
                    tx,
                    &GateProbe(std::sync::Mutex::new(gate)),
                    9,
                )
            });
            let mut rtt = false;
            let mut online = false;
            while let Ok(event) = rx.recv_timeout(Duration::from_secs(2)) {
                match event {
                    Event::Log(_, a, r) if a == "Ping" && r == "7 ms" => rtt = true,
                    Event::Probe(9, 0, s, d) if s == "Доступен" && d == "Проверка…" =>
                    {
                        online = true;
                        break;
                    }
                    _ => {}
                }
            }
            assert!(rtt && online, "ICMP result must not wait for domain lookup");
            release.send(()).unwrap();
        });
        assert!(
            rx.try_iter()
                .any(|e| matches!(e,Event::Probe(9,0,s,d) if s=="Доступен"&&d=="WORKGROUP"))
        );
    }
}
