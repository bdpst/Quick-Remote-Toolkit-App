# Quick Remote Toolkit — Rust

Windows x64 MSVC, Rust 1.99.1, egui/eframe 0.33.3 (OpenGL/WGL, AccessKit).
Продолжение переноса; полный визуальный и удалённый паритет ещё не подтверждён.
Сборка: cargo fmt --check, cargo clippy --all-targets -- -D warnings, cargo test
с --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml; зависимости закреплены Cargo.lock.
Публикация: ./src/QuickRemoteToolkit.Rust/build.ps1 -OutputDirectory ./publish-rust.
Запуск: ./publish-rust/QuickRemoteToolkitRust.exe. Архив в том же каталоге.

.NET для запуска не требуется; нужны Windows 10/11 x64, системные DLL и OpenGL-драйвер.
WinRS/PowerShell Remoting/SMB требуют штатных компонентов и прав текущего пользователя.
Системные Segoe UI/MDL2 загружаются из Windows, не распространяются. ICO встроена.
Настройки: %AppData%/QuickRemoteToolkitApp/settings.json, совместимые PascalCase поля WPF,
BOM, defaults, избранное computer:/ip:. Перед первой перезаписью сохраняется
settings.before-rust.json; повреждённый JSON не перезаписывается defaults.
Старый prototype settings-rust.json не подменяет общий settings.json.

[Матрица](../../specs/005-rust-parity/parity-matrix.md),
[прогресс](../../specs/005-rust-parity/progress.md).
Текст сообщений не пишется в журнал/настройки, пароли не хранятся.
WPF source и publish остаются для сравнения и отката; старые specs — история, новая приёмка в 005.

Иконка EXE и манифест PerMonitorV2/asInvoker собираются Windows SDK `rc.exe` через
build.rs. Если SDK установлен в нестандартное место, задайте QRT_RC полным путём к rc.exe.
Дополнительного runtime это не добавляет.

GUI-тесты: tests/ui-smoke.ps1 -OutputDirectory <новый каталог>. Отдельный qrt-ui-qa
рендерит настоящий UI с обезличенными состояниями и проверяет Enter/Ctrl+Enter на
невалидном вводе. Текст тестового сообщения не записывается. Этот host не поставляется.
Сравнение с эталоном: tests/reference-smoke.ps1 -OutputDirectory <новый каталог>.
Для reference host нужен .NET SDK 10, для Rust-приложения .NET Desktop Runtime не требуется.
