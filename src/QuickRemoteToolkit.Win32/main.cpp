#define _WIN32_WINNT 0x0A00
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <algorithm>
#include <commctrl.h>
#include <commdlg.h>
#include <cwctype>
#include <filesystem>
#include <fstream>
#include <shellapi.h>
#include <sstream>
#include <string>
#include <vector>

#include <windowsx.h>
struct Client
{
    std::wstring number, computer, ip, person;
};
struct Entry
{
    std::wstring time, pc, action, result;
};
struct Section
{
    HWND header;
    std::vector<HWND> buttons;
    bool open = true;
};
static HWND window, search, table, logs, countLabel, pathLabel, heading;
static HFONT font, titleFont;
static UINT dpi = 96;
static std::vector<Client> clients;
static std::vector<size_t> visible;
static std::vector<Entry> entries;
static std::vector<Section> sections;
static std::wstring csvPath;
enum
{
    Csv = 100,
    Reload,
    Search,
    Clients,
    Logs,
    Remote = 200,
    Ping,
    Trace,
    Share,
    Events,
    Management,
    Rdp,
    Winrs,
    Gpupdate,
    Export,
    SectionBase = 300,
    CopyPc = 400,
    CopyIp,
    CopyPerson
};
static int px(int n)
{
    return MulDiv(n, (int)dpi, 96);
}
static std::wstring text(HWND h)
{
    int n = GetWindowTextLengthW(h);
    std::wstring s(n + 1, L'\0');
    GetWindowTextW(h, s.data(), n + 1);
    s.resize(n);
    return s;
}
static std::wstring lower(std::wstring s)
{
    std::transform(s.begin(), s.end(), s.begin(), [](wchar_t c) { return std::towlower(c); });
    return s;
}
static HWND control(const wchar_t *cls, const wchar_t *title, DWORD style, int id)
{
    HWND h = CreateWindowExW(0, cls, title, WS_CHILD | WS_VISIBLE | style, 0, 0, 0, 0, window, (HMENU)(INT_PTR)id,
                             nullptr, nullptr);
    SendMessageW(h, WM_SETFONT, (WPARAM)font, TRUE);
    return h;
}
static void column(HWND h, int i, const wchar_t *title, int width)
{
    LVCOLUMNW c{};
    c.mask = LVCF_TEXT | LVCF_WIDTH;
    c.pszText = (LPWSTR)title;
    c.cx = px(width);
    ListView_InsertColumn(h, i, &c);
}
static void cell(HWND h, int row, int col, const std::wstring &s)
{
    ListView_SetItemText(h, row, col, (LPWSTR)s.c_str());
}
static void log(const std::wstring &pc, const std::wstring &action, const std::wstring &result)
{
    SYSTEMTIME now{};
    GetLocalTime(&now);
    wchar_t time[32];
    swprintf_s(time, L"%02d:%02d:%02d", now.wHour, now.wMinute, now.wSecond);
    entries.push_back({time, pc, action, result});
    LVITEMW item{};
    item.mask = LVIF_TEXT;
    item.iItem = 0;
    item.pszText = time;
    int row = ListView_InsertItem(logs, &item);
    cell(logs, row, 1, pc);
    cell(logs, row, 2, action);
    cell(logs, row, 3, result);
}
static void populate()
{
    std::wstring query = lower(text(search));
    SendMessageW(table, WM_SETREDRAW, FALSE, 0);
    ListView_DeleteAllItems(table);
    visible.clear();
    for (size_t i = 0; i < clients.size(); ++i)
    {
        auto &c = clients[i];
        if (!query.empty() && lower(c.computer + L" " + c.ip + L" " + c.person).find(query) == std::wstring::npos)
            continue;
        int row = (int)visible.size();
        visible.push_back(i);
        LVITEMW item{};
        item.mask = LVIF_TEXT;
        item.iItem = row;
        item.pszText = c.number.data();
        ListView_InsertItem(table, &item);
        cell(table, row, 1, c.computer);
        cell(table, row, 2, c.ip);
        cell(table, row, 3, c.person);
    }
    SendMessageW(table, WM_SETREDRAW, TRUE, 0);
    InvalidateRect(table, nullptr, TRUE);
    SetWindowTextW(
        countLabel,
        (L"Показано: " + std::to_wstring(visible.size()) + L" / " + std::to_wstring(clients.size())).c_str());
}
static std::vector<std::wstring> fields(const std::wstring &line)
{
    std::vector<std::wstring> f;
    std::wstring s;
    bool quote = false;
    for (size_t i = 0; i < line.size(); ++i)
    {
        if (line[i] == L'"')
        {
            if (quote && i + 1 < line.size() && line[i + 1] == L'"')
            {
                s += L'"';
                ++i;
            }
            else
                quote = !quote;
        }
        else if (line[i] == L';' && !quote)
        {
            f.push_back(s);
            s.clear();
        }
        else
            s += line[i];
    }
    if (quote)
        throw std::runtime_error("Unclosed CSV field");
    f.push_back(s);
    return f;
}
static void load(const std::wstring &path)
{
    try
    {
        std::ifstream file(std::filesystem::path(path), std::ios::binary);
        if (!file)
            throw std::runtime_error("Cannot open CSV");
        std::string bytes((std::istreambuf_iterator<char>(file)), {});
        UINT cp = CP_UTF8;
        DWORD flags = MB_ERR_INVALID_CHARS;
        int n = MultiByteToWideChar(cp, flags, bytes.data(), (int)bytes.size(), nullptr, 0);
        if (!n && !bytes.empty())
        {
            cp = 1251;
            flags = 0;
            n = MultiByteToWideChar(cp, flags, bytes.data(), (int)bytes.size(), nullptr, 0);
        }
        std::wstring content(n, L'\0');
        MultiByteToWideChar(cp, flags, bytes.data(), (int)bytes.size(), content.data(), n);
        if (!content.empty() && content[0] == 0xfeff)
            content.erase(0, 1);
        std::wistringstream stream(content);
        std::wstring line;
        std::getline(stream, line);
        if (!line.empty() && line.back() == L'\r')
            line.pop_back();
        if (lower(line) != L"number;computer;ip;person")
            throw std::runtime_error("Invalid header");
        std::vector<Client> loaded;
        while (std::getline(stream, line))
        {
            if (!line.empty() && line.back() == L'\r')
                line.pop_back();
            if (line.empty())
                continue;
            auto f = fields(line);
            if (f.size() != 4 || f[1].empty())
                throw std::runtime_error("Invalid row");
            loaded.push_back({f[0], f[1], f[2], f[3]});
        }
        clients = std::move(loaded);
        csvPath = path;
        SetWindowTextW(pathLabel, path.c_str());
        populate();
        log(L"", L"CSV", L"Загружено клиентов: " + std::to_wstring(clients.size()));
    }
    catch (...)
    {
        MessageBoxW(window,
                    L"Не удалось прочитать CSV. Формат: number;computer;ip;person, четыре поля в каждой строке.",
                    L"Ошибка CSV", MB_ICONERROR);
    }
}
static std::wstring choose(bool save)
{
    wchar_t path[MAX_PATH] = L"";
    if (save)
        wcscpy_s(path, L"quick-remote-toolkit-log.txt");
    OPENFILENAMEW d{};
    d.lStructSize = sizeof(d);
    d.hwndOwner = window;
    d.lpstrFile = path;
    d.nMaxFile = MAX_PATH;
    d.lpstrFilter = save ? L"Журнал TXT\0*.txt\0\0" : L"Список клиентов CSV\0*.csv\0\0";
    d.lpstrDefExt = save ? L"txt" : L"csv";
    d.Flags = OFN_EXPLORER | OFN_NOCHANGEDIR | (save ? OFN_OVERWRITEPROMPT : OFN_FILEMUSTEXIST);
    return (save ? GetSaveFileNameW(&d) : GetOpenFileNameW(&d)) ? path : L"";
}
static void layout()
{
    RECT r{};
    GetClientRect(window, &r);
    int w = MulDiv(r.right, 96, dpi), h = MulDiv(r.bottom, 96, dpi);
    auto move = [](HWND c, int x, int y, int width, int height) {
        MoveWindow(c, px(x), px(y), px(width), px(height), TRUE);
    };
    move(heading, 20, 15, w - 40, 30);
    move(search, 20, 58, w - 340, 34);
    move(GetDlgItem(window, Csv), w - 300, 58, 135, 34);
    move(GetDlgItem(window, Reload), w - 155, 58, 135, 34);
    int side = 260, left = w - side - 50;
    move(table, 20, 110, left, h - 330);
    move(countLabel, 20, h - 210, left, 23);
    move(pathLabel, 20, h - 183, left, 23);
    move(logs, 20, h - 150, left, 130);
    ListView_SetColumnWidth(table, 3, px(std::max(120, left - 355)));
    ListView_SetColumnWidth(logs, 3, px(std::max(120, left - 350)));
    int y = 110;
    for (auto &s : sections)
    {
        move(s.header, w - side - 20, y, side, 34);
        y += 42;
        for (auto button : s.buttons)
        {
            ShowWindow(button, s.open ? SW_SHOW : SW_HIDE);
            if (s.open)
            {
                move(button, w - side - 20, y, side, 32);
                y += 39;
            }
        }
        y += 12;
    }
}
static void addSection(const wchar_t *name, std::initializer_list<std::pair<int, const wchar_t *>> buttons)
{
    Section s{};
    s.header = control(L"BUTTON", name, BS_PUSHBUTTON | WS_TABSTOP, SectionBase + (int)sections.size());
    for (auto item : buttons)
        s.buttons.push_back(control(L"BUTTON", item.second, BS_PUSHBUTTON | WS_TABSTOP, item.first));
    sections.push_back(s);
}
static Client *selected()
{
    int row = ListView_GetNextItem(table, -1, LVNI_SELECTED);
    return row >= 0 && (size_t)row < visible.size() ? &clients[visible[row]] : nullptr;
}
static bool copy(const std::wstring &value)
{
    if (value.empty() || !OpenClipboard(window))
        return false;
    bool ok = false;
    HGLOBAL mem = GlobalAlloc(GMEM_MOVEABLE, (value.size() + 1) * sizeof(wchar_t));
    if (mem)
    {
        void *p = GlobalLock(mem);
        if (p)
        {
            memcpy(p, value.c_str(), (value.size() + 1) * sizeof(wchar_t));
            GlobalUnlock(mem);
            EmptyClipboard();
            if (SetClipboardData(CF_UNICODETEXT, mem))
                ok = true;
            else
                GlobalFree(mem);
        }
        else
            GlobalFree(mem);
    }
    CloseClipboard();
    return ok;
}
static void action(int id)
{
    if (id == Csv)
    {
        auto p = choose(false);
        if (!p.empty())
            load(p);
        return;
    }
    if (id == Reload)
    {
        if (!csvPath.empty())
            load(csvPath);
        return;
    }
    if (id == Export)
    {
        auto p = choose(true);
        if (p.empty())
            return;
        std::wstring content;
        for (auto &e : entries)
            content += e.time + L" | " + e.pc + L" | " + e.action + L" | " + e.result + L"\r\n";
        int n = WideCharToMultiByte(CP_UTF8, 0, content.data(), (int)content.size(), nullptr, 0, nullptr, nullptr);
        std::string bytes(n, '\0');
        WideCharToMultiByte(CP_UTF8, 0, content.data(), (int)content.size(), bytes.data(), n, nullptr, nullptr);
        std::ofstream file(std::filesystem::path(p), std::ios::binary);
        file << "\xef\xbb\xbf" << bytes;
        if (file.good())
            log(L"", L"Экспорт", L"Журнал сохранён.");
        else
            MessageBoxW(window, L"Не удалось сохранить журнал.", L"Ошибка", MB_ICONERROR);
        return;
    }
    Client *c = selected();
    if (!c)
    {
        MessageBoxW(window, L"Выберите клиента.", L"Quick Remote Toolkit", MB_ICONINFORMATION);
        return;
    }
    if (id >= CopyPc && id <= CopyPerson)
    {
        bool ok = copy(id == CopyPc ? c->computer : id == CopyIp ? c->ip : c->person);
        log(c->computer, L"Копирование", ok ? L"Скопировано." : L"Не удалось скопировать.");
        return;
    }
    std::wstring target = (id == Ping || id == Trace) && !c->ip.empty() && c->ip != L"-" ? c->ip : c->computer;
    if (target.empty() || !std::all_of(target.begin(), target.end(), [](wchar_t ch) {
            return std::iswalnum(ch) || ch == L'.' || ch == L'-' || ch == L'_';
        }))
    {
        MessageBoxW(window, L"Имя ПК или IP содержит недопустимые символы.", L"Ошибка", MB_ICONERROR);
        return;
    }
    std::wstring exe, args;
    switch (id)
    {
    case Remote:
        exe = L"msra.exe";
        args = L"/offerra \"" + target + L"\"";
        break;
    case Ping:
        exe = L"cmd.exe";
        args = L"/d /k ping \"" + target + L"\"";
        break;
    case Trace:
        exe = L"cmd.exe";
        args = L"/d /k tracert \"" + target + L"\"";
        break;
    case Share:
        exe = L"explorer.exe";
        args = L"\\\\" + target + L"\\c$";
        break;
    case Events:
        exe = L"eventvwr.msc";
        args = L"/computer:" + target;
        break;
    case Management:
        exe = L"compmgmt.msc";
        args = L"/computer:\\\\" + target;
        break;
    case Rdp:
        exe = L"mstsc.exe";
        args = L"/v:" + target;
        break;
    case Winrs:
        exe = L"cmd.exe";
        args = L"/d /k winrs -r:" + target + L" cmd";
        break;
    case Gpupdate:
        exe = L"cmd.exe";
        args = L"/d /k winrs -r:" + target + L" gpupdate /force";
        break;
    default:
        return;
    }
    INT_PTR result = (INT_PTR)ShellExecuteW(window, L"open", exe.c_str(), args.c_str(), nullptr, SW_SHOWNORMAL);
    log(c->computer, text(GetDlgItem(window, id)),
        result > 32 ? L"Запущено." : L"Ошибка запуска: " + std::to_wstring(result));
}
static LRESULT CALLBACK procedure(HWND h, UINT message, WPARAM wp, LPARAM lp)
{
    switch (message)
    {
    case WM_CREATE: {
        window = h;
        dpi = GetDpiForWindow(h);
        font = CreateFontW(-px(13), 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE, DEFAULT_CHARSET, 0, 0, CLEARTYPE_QUALITY,
                           0, L"Segoe UI");
        titleFont = CreateFontW(-px(19), 0, 0, 0, FW_SEMIBOLD, FALSE, FALSE, FALSE, DEFAULT_CHARSET, 0, 0,
                                CLEARTYPE_QUALITY, 0, L"Segoe UI");
        heading = control(L"STATIC", L"Quick Remote Toolkit   /   Win32-прототип", 0, 0);
        SendMessageW(heading, WM_SETFONT, (WPARAM)titleFont, TRUE);
        search = control(L"EDIT", L"", WS_BORDER | ES_AUTOHSCROLL | WS_TABSTOP, Search);
        SendMessageW(search, EM_SETCUEBANNER, TRUE, (LPARAM)L"Поиск: компьютер, IP или сотрудник");
        control(L"BUTTON", L"Выбрать CSV…", BS_PUSHBUTTON | WS_TABSTOP, Csv);
        control(L"BUTTON", L"Обновить", BS_PUSHBUTTON | WS_TABSTOP, Reload);
        table = control(WC_LISTVIEWW, L"", LVS_REPORT | LVS_SINGLESEL | LVS_SHOWSELALWAYS | WS_BORDER | WS_TABSTOP,
                        Clients);
        ListView_SetExtendedListViewStyle(table, LVS_EX_FULLROWSELECT | LVS_EX_DOUBLEBUFFER | LVS_EX_GRIDLINES);
        column(table, 0, L"№", 45);
        column(table, 1, L"Компьютер", 175);
        column(table, 2, L"IP", 135);
        column(table, 3, L"Сотрудник", 300);
        logs = control(WC_LISTVIEWW, L"", LVS_REPORT | WS_BORDER | WS_TABSTOP, Logs);
        ListView_SetExtendedListViewStyle(logs, LVS_EX_FULLROWSELECT | LVS_EX_DOUBLEBUFFER);
        column(logs, 0, L"Время", 80);
        column(logs, 1, L"Компьютер", 135);
        column(logs, 2, L"Действие", 135);
        column(logs, 3, L"Результат", 400);
        countLabel = control(L"STATIC", L"Клиентов: 0", 0, 0);
        pathLabel = control(L"STATIC", L"CSV не выбран", SS_PATHELLIPSIS, 0);
        addSection(L"▾  Подключение",
                   {{Remote, L"Remote Assistance"}, {Ping, L"Ping"}, {Trace, L"Tracert"}, {Share, L"Открыть C$"}});
        addSection(L"▾  Инструменты", {{Events, L"Event Viewer"},
                                       {Management, L"Computer Management"},
                                       {Rdp, L"MSTSC"},
                                       {Winrs, L"WinRS cmd"},
                                       {Gpupdate, L"gpupdate /force"}});
        addSection(L"▾  Операции", {{Export, L"Экспорт логов"}});
        layout();
        wchar_t exe[MAX_PATH];
        GetModuleFileNameW(nullptr, exe, MAX_PATH);
        auto sample = std::filesystem::path(exe).parent_path().parent_path() / L"samples" / L"clients.example.csv";
        if (std::filesystem::exists(sample))
            load(sample.wstring());
        return 0;
    }
    case WM_SIZE:
        if (table)
            layout();
        return 0;
    case WM_GETMINMAXINFO: {
        auto info = (MINMAXINFO *)lp;
        info->ptMinTrackSize = {px(1000), px(820)};
        return 0;
    }
    case WM_COMMAND: {
        int id = LOWORD(wp);
        if (id == Search && HIWORD(wp) == EN_CHANGE)
        {
            populate();
            return 0;
        }
        if (HIWORD(wp) != BN_CLICKED)
            return 0;
        if (id >= SectionBase && id < SectionBase + (int)sections.size())
        {
            auto &s = sections[id - SectionBase];
            s.open = !s.open;
            auto label = text(s.header);
            label[0] = s.open ? L'▾' : L'▸';
            SetWindowTextW(s.header, label.c_str());
            layout();
            return 0;
        }
        action(id);
        return 0;
    }
    case WM_CONTEXTMENU:
        if ((HWND)wp == table)
        {
            POINT p{GET_X_LPARAM(lp), GET_Y_LPARAM(lp)};
            if (p.x == -1)
                GetCursorPos(&p);
            else
            {
                POINT local = p;
                ScreenToClient(table, &local);
                LVHITTESTINFO hit{};
                hit.pt = local;
                int row = ListView_HitTest(table, &hit);
                if (row < 0)
                    return 0;
                ListView_SetItemState(table, row, LVIS_SELECTED | LVIS_FOCUSED, LVIS_SELECTED | LVIS_FOCUSED);
            }
            if (!selected())
                return 0;
            HMENU menu = CreatePopupMenu();
            AppendMenuW(menu, MF_STRING, CopyPc, L"Копировать имя ПК");
            AppendMenuW(menu, MF_STRING, CopyIp, L"Копировать IP");
            AppendMenuW(menu, MF_STRING, CopyPerson, L"Копировать имя сотрудника");
            int id = TrackPopupMenu(menu, TPM_RETURNCMD | TPM_RIGHTBUTTON, p.x, p.y, 0, h, nullptr);
            DestroyMenu(menu);
            if (id)
                action(id);
            return 0;
        }
        break;
    case WM_DESTROY:
        DeleteObject(font);
        DeleteObject(titleFont);
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcW(h, message, wp, lp);
}
int WINAPI wWinMain(HINSTANCE instance, HINSTANCE, PWSTR, int show)
{
    SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_SYSTEM_AWARE);
    INITCOMMONCONTROLSEX controls{sizeof(controls), ICC_LISTVIEW_CLASSES};
    InitCommonControlsEx(&controls);
    WNDCLASSEXW cls{};
    cls.cbSize = sizeof(cls);
    cls.hInstance = instance;
    cls.lpfnWndProc = procedure;
    cls.lpszClassName = L"QRTWin32Prototype";
    cls.hCursor = LoadCursorW(nullptr, IDC_ARROW);
    cls.hIcon = LoadIconW(instance, MAKEINTRESOURCEW(101));
    cls.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
    RegisterClassExW(&cls);
    HWND h = CreateWindowExW(0, cls.lpszClassName, L"Quick Remote Toolkit — C++ / Win32", WS_OVERLAPPEDWINDOW,
                             CW_USEDEFAULT, CW_USEDEFAULT, 1400, 920, nullptr, nullptr, instance, nullptr);
    if (!h)
        return 1;
    ShowWindow(h, show);
    UpdateWindow(h);
    MSG msg{};
    while (GetMessageW(&msg, nullptr, 0, 0) > 0)
    {
        if (!IsDialogMessageW(h, &msg))
        {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    return (int)msg.wParam;
}