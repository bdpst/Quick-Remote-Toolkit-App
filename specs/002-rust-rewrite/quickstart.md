> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Quickstart
1. Установить Rust stable MSVC и Visual Studio C++ Build Tools + Windows SDK.
2. `cargo test --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml`.
3. `powershell -File src/QuickRemoteToolkit.Rust/build.ps1`.
4. Запустить rust-publish/QuickRemoteToolkitRust.exe; открыть samples/clients.example.csv, проверить обе темы, поиск, копирование, избранное и перезапуск.
5. На разрешённом тестовом ПК сравнить каждую команду с PowerShell/CMD; сообщения ОК/Отмена/тайм-аут; членство добавить/удалить и проверить независимо. Не использовать реальные CSV в git.
6. Проверить недоступный ПК и отказ доступа; интерфейс не зависает. Экспорт TXT/CSV с кириллицей.
7. Проверить запуск внутри RDP и на Windows 10/11 без .NET; измерить размер/память отдельно.
