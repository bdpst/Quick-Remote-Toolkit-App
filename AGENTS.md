# AGENTS.md

Quick Remote Toolkit — поддерживаемое Windows-приложение на Rust. WPF/C++ удалены;
история доступна в Git. Основной код: `src/QuickRemoteToolkit.Rust/src/`, ресурсы:
`assets/`, проверки/QA: `tests/`, дизайн: `docs/`, спецификации: `specs/`.

## Проверки и локальная поставка

Windows x64 MSVC, Rust 1.99.1, MSVC Build Tools и Windows SDK. Из корня:

```powershell
cargo fmt --check --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml
cargo clippy --locked --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml --all-targets -- -D warnings
cargo test --locked --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml
cargo build --release --locked --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml
./src/QuickRemoteToolkit.Rust/build.ps1 -OutputDirectory ./publish-rust
```

После изменения приложения выполнить проверки и обновить `publish-rust/`. Каталог
относителен checkout; персональные пути не нужны. Не коммитить target/, поставки,
архивы, временные файлы и реальные CSV. Сохранить работу пользователя; без force push/reset.

## Правила реализации

- UI: `docs/design-guidelines.md`; цвета/шрифты в `src/design.rs`. Проверять обе темы,
  поиск, splitter и затронутые диалоги. Сохранять монитор с шестерёнкой во всех иконках.
- Блокирующие Windows API/процессы/сеть выполнять вне UI-потока. Возвращать результаты
  через события; защищать повторный запуск и устаревшие ответы. Не считать старт
  процесса подтверждением успешной удалённой операции.
- CSV: `number;computer;ip;person`, UTF-8/Windows-1251. Сохранять контракт JSON,
  AppData-путь, unknown поля, backup и защиту повреждённых настроек. Не трогать личные данные.
- Только текущая Windows-сессия, без хранения/передачи паролей и обхода прав.
  SMB/WinRM/WTS требуют соответствующих служб и прав. Изменённые удалённые команды
  проверять ручным эквивалентом только на явно разрешённых тестовых ПК.
- Для существенных изменений Spec Kit: specify → plan → tasks → implement;
  использовать установленный resolver и существующие материалы. Небольшие исправления
  не требуют новой спецификации. Исторические specs не являются актуальными инструкциями.
- Фиксировать выполненные проверки и ограничения, не закрывать непроведённые тесты.

## Выпуск

Единственный источник версии — Cargo.toml; синхронизировать Cargo.lock. build.rs,
UI и build.ps1 получают её автоматически. Подробные команды и процесс — в README.
Сначала локальные проверки и CI, затем разрешённое слияние с main и успешный main CI.
Тег `0.4.0` (в дальнейшем также поддерживаются `v*`) ставить на проверенный main.
Workflow публикует стабильный Release с ZIP, BUILD-INFO.json и SHA256SUMS.
Проверить скачанный архив, версию и source commit. Не перемещать существующие теги,
не обходить правила GitHub; PR/релиз публиковать только в рамках разрешённой задачи.
