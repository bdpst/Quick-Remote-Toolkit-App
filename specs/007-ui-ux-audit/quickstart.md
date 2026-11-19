# Quickstart validation

Windows x64 MSVC, Rust 1.99.1, PowerShell 7, OpenGL 3.3. Из корня checkout:

```powershell
./src/QuickRemoteToolkit.Rust/tests/ui-smoke.ps1 -OutputDirectory ./publish-rust/audit-007-normal
./src/QuickRemoteToolkit.Rust/tests/ui-smoke.ps1 -OutputDirectory ./publish-rust/audit-007-minimum -Width 1200 -Height 760
```

Использовать новые output directories при повторе. Host создаёт отдельный AppData и demo.csv. Ожидаются 26 PASS на размер; state.json: messaging/administering false. Сопоставить картинки обеих тем и metadata dpi.
Для ручного прохода запускать отдельный debug executable с APPDATA, указывающим только на тестовый профиль. Открыть только demo.csv, Ctrl+F/избранное/колонки/копирование/splitter, диалоги с невалидным вводом, отмену/экспорт; не запускать сеть. Восстановить/закрыть только этот экземпляр. Не использовать уже открытое личное приложение.
Проверки Windows DPI, screen reader и реальных сетевых результатов обозначать отдельно; подробная матрица в validation.md.
