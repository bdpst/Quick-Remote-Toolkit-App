> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Проверка
Cargo fmt --check; clippy --all-targets -- -D warnings; test; build --release --locked с --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml. Затем build.ps1 -OutputDirectory publish-rust. UI: docs/ui-parity-checklist.md, одинаковые размеры/DPI; удалённое — только разрешённые цели. Матрица и progress фиксируют результаты и внешние блокеры.


GUI evidence без реальной сети: src/QuickRemoteToolkit.Rust/tests/ui-smoke.ps1
-OutputDirectory <новый каталог>; reference-smoke.ps1 снимает неизменённые окна
точного main SHA с изолированными настройками. Подробности в ui-validation.md.
PE/ZIP: python src/QuickRemoteToolkit.Rust/tests/verify-artifact.py
publish-rust/QuickRemoteToolkitRust.exe publish-rust/QuickRemoteToolkit.Rust-win-x64.zip
<новый JSON отчёт>. build.rs требует SDK rc.exe; QRT_RC задаёт нестандартный путь.
