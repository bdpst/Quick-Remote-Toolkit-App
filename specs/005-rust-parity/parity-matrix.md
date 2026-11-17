# Матрица паритета

Эталон: main 91689516c96d52877bba3480bf498b7d807ead64. Источники WPF — src/QuickRemoteToolkit.App/ этого SHA, Rust — src/QuickRemoteToolkit.Rust/src/. Статус кода не равен проверенному паритету.

| ID | Сценарий | Исходный код | Контракт | Rust | Проверка | Статус |
| --- | --- | --- | --- | --- | --- | --- |
| D01 | CSV UTF-8/1251, пустые/ошибки/номера | CsvClientStore.Load | первая строка пропускается; 0 сохраняется; ошибочный номер=index+1 | services::parse_csv/load_csv; native::default_csv_path | UTF-8/1251, invalid/empty/zero/negative: unit PASS; UI file dialogs pending | код + unit PASS |
| D02 | Настройки WPF JSON/BOM/atomic/rollback | SettingsService; AppSettings | settings.json, PascalCase, defaults, unknown поля, атомарная запись | services::load_settings_at/save_settings_at; models::Settings | BOM/atomic/backup/unknown fields PASS; фактический WPF→Rust→WPF PASS | данные PASS |
| D03 | Избранное/дубликаты/IP fallback/filter | ClientEntry.FavoriteKey; Favorite_Click | computer:UPPER или ip:UPPER; rollback ошибки | Client::key; Settings::favorite/toggle; ui save rollback | keys/fallback/empty/duplicates unit PASS; filter/rollback UI pending | код + unit PASS |
| D04 | Поиск 4 поля/trim/Ctrl+F | FilterClient; FocusSearch | имя/IP/сотрудник/domain без номера | Client::matches; ui::toolbar/client_content | domain+trim unit PASS; Ctrl+F SelectAll реализован; UI automation input не подтвердился | код + unit PASS |
| U01 | Таблица 8 колонок/sort/select/ПКМ/copy/doubleclick | MainWindow.xaml; ClientsGrid_* | по гайду §6 | ui::client_table/copy_cell; client_sort | 1000 строк/обе темы/sort+arrow/hidden selection headless PASS; фактические меню/copy/doubleclick pending | код + headless PASS |
| N01 | Выбранный/все ping/timeout/limit | PingClientAsync; PingAll_Click | выбранный не внешний cmd; вся коллекция; время начала | services::probe_with/ProbeAdapter; Event generation; ui::poll_events | лимит 3/16 целей, ошибка, закрытый receiver, late generation PASS; loopback ICMP PASS | код + unit PASS |
| N02 | Домен/workgroup/timeout | DomainMembershipService | 5 секунд, все результаты, лог ошибок | native::domain; services::probe_with | NetAPI memory/timeout/code review; реальные domain/workgroup/ошибки доступа pending | реальная интеграция pending |
| N03 | RA/tracert/EventViewer/compmgmt/MSTSC/WinRS/gpupdate | RemoteActionService | штатные процессы, Computer/Target, безопасные аргументы | services::action_with/ActionAdapter; native::shell_open | все команды с mock adapter и кириллической целью PASS; shell injection rejects PASS; реальная интеграция pending | код + unit PASS |
| N04 | C$/SMB текущая сессия | OpenAdminShare; EnsureSmbSession | net use IPC$, 5s, диагностика, Explorer | SystemActions::smb; services::action_with(3) | mock SMB target PASS; реальные net.exe/SMB stdout/stderr и текущие права pending | код; интеграция pending |
| N05 | WTS messages/user sessions/responses | RemoteMessageService | 2000/1–3600/active+disconnected/parallel/no text logs | native::send_message; services::dispatch_sessions | параллельные 5 mock сеансов, OK/Cancel/timeout/other/error PASS; реальные WTS pending | код + unit PASS |
| N06 | Admin SID/change/timeout | LocalAdministratorsService | SID 544, change membership, 60s unknown, no auto repeat | services::administrators/powershell; valid_member | SID/script review, quote/account validation и локальный безопасный process timeout PASS; реальный WinRM pending | код + unit PASS |
| U02 | Message/Admin native dialogs | SendMessageWindow; LocalAdministratorsWindow | размеры/подписи/owner/focus/keys, закрыть до network | ui::dialogs; native::set_dialog_owner | native viewports + owner/input disable, autosize admin, validation/focus/keys реализованы; визуально/интерактивно pending | приёмка pending |
| U03 | Темы/19 tokens/states/card/fonts | ThemeService; App.xaml | обе темы и все компоненты по JSON | design::Palette/primary/section; ui fonts | Segoe regular/bold/semibold/MDL2; distinct hover/pressed/primary; headless two themes PASS; пиксели pending | приёмка pending |
| U04 | Chrome/icon/multimonitor/DPI | WindowChrome; WindowMessageHook | 32 header; 46x32 buttons; nearest WorkArea; ICO | icon::application_icon; ui::title_bar; eframe/winit | ICO, captions 46x32 и maximize glyph, borderless resize: код; Win10 окно/UIA получены; WorkArea/DPI pending | приёмка pending |
| U05 | Секции/CSV path/location/reload | SectionExpanderStyle; CsvPath_Click | 3 expander + CSV; открыть Explorer/select | design::section; ui::sidebar_content/toolbar/reload | 3 right-chevron headers, CSV path clickable, info/load time; UI dialogs/location/reload pending | приёмка pending |
| L01 | Журнал/clear/TXT/CSV | AddLog; ClearLogs_Click; ExportLogs_Click | новые сверху; clear без подтверждения; TXT default | ui::log/log_content/export; services::export_logs | newest-first/clear, log sort и TXT default, CSV escaping unit PASS; очистка/экспорт через UI pending | код + unit PASS |
| B01 | Сборка/portable/CI/archive/runtime | csproj; release.yml | Rust locked MSVC x64, publish-rust, без .NET | Cargo.lock; build.ps1; .github/workflows/release.yml | fmt/clippy/25 tests PASS; release/ZIP/PE checks; Windows CI definition обновлён, удалённый запуск CI не выполнялся; чистая VM pending | сборка PASS; приёмка pending |
| S01 | Shutdown/late events/busy guards/1000 rows | MainWindow async handlers | UI не блокируется; повтор защищён; destroyed UI недоступен | worker channels; generation; busy guards; native handles | mock dropped receiver/generation/parallelism и headless1000 PASS; shutdown на реальном WTS/admin pending | код + unit PASS |

## Намеренные изменения
Отклонять shell injection; CSV export экранировать; блокировать повторные probe/reload во время probe для стабильности индексов. Общий settings.json вместо prototype settings-rust — прямое требование нового запроса. Эти изменения не разрешают менять остальной UI.

## Доказательства продолжения 2026-11-16

UI U01–U05/L01: настоящее rendering проверено по ui-validation.md: 26 + 8 + 4 Rust
запусков, 12 + 2 эталонных WPF окон, обе темы/два размера, включая меню/ошибки.
Это частичная приёмка: физический Windows input/все DPI/сеть пока не доказаны.
Перестановка колонок/width identity, Extended log selection/Copy ExcludeHeader,
scroll-to-selection покрыты отдельными тестами, всего25. B01: icon+manifest PE,
static CRT/ZIP совпадение/финальный portable smoke — artifact-validation.json.
D02: финальный WPF→Rust→WPF round-trip повторно PASS, signed WPF int сохраняются.
Намеренные отличия темы WPF-диалогов и raster/wrapping зарегистрированы в UI отчёте.


U01/U03, splitter regression: многокадровый Pointer drag/release воспроизведён и
исправлен; total movement, reverse drag и bounds в обеих темах проверены UI-тестом.
Актуальное количество Cargo tests:26. Rendering evidence прежней поставки не является
доказательством этого жеста; текущая поставка записана в splitter-fix-validation.json.


U04, 2026-11-17: пользователь запросил новую читаемую иконку. Новый SVG/ICO —
крупный монитор со стрелкой, 9 отдельных размеров от16до256. Native title bar кадры
обеих тем и PNG16 проверены; user-approved визуальное отличие от WPF reference.
