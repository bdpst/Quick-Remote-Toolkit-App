# Проверка
Cargo fmt --check; clippy --all-targets -- -D warnings; test; build --release --locked с --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml. Затем build.ps1 -OutputDirectory publish-rust. UI: docs/ui-parity-checklist.md, одинаковые размеры/DPI; удалённое — только разрешённые цели. Матрица и progress фиксируют результаты и внешние блокеры.


GUI evidence без реальной сети: src/QuickRemoteToolkit.Rust/tests/ui-smoke.ps1
-OutputDirectory <новый каталог>; reference-smoke.ps1 снимает неизменённые окна
точного main SHA с изолированными настройками. Подробности в ui-validation.md.
PE/ZIP: python src/QuickRemoteToolkit.Rust/tests/verify-artifact.py
publish-rust/QuickRemoteToolkitRust.exe publish-rust/QuickRemoteToolkit.Rust-win-x64.zip
<новый JSON отчёт>. build.rs требует SDK rc.exe; QRT_RC задаёт нестандартный путь.
