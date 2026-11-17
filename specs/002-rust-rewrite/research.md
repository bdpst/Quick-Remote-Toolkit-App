# Research
- Decision: eframe/egui_extras =0.33.3, rfd =0.17.2. Rationale: native Rust GUI, известный update API, виртуализированная таблица, системные диалоги. Alternatives: Win32 вручную сложнее поддерживать; Tauri требует web UI. Sources: https://docs.rs/eframe/0.33.3/eframe/trait.App.html ; https://docs.rs/rfd/0.17.2/rfd/.
- Decision: MSVC toolchain с Windows SDK. Rationale: штатная Windows-сборка. Source: https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup.
- Decision: WTS/NetAPI FFI и существующие штатные Windows-команды; отдельный JSON с импортом WPF. Rationale: сохранить полномочия и совместимость, избежать взаимной перезаписи настроек.
- Decision: системный Segoe UI для кириллицы; не распространять файл шрифта. Glow проверить в RDP отдельно, возможны ограничения драйвера OpenGL.
