> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Research
- Decision: eframe/egui_extras =0.33.3, rfd =0.17.2. Rationale: native Rust GUI, известный update API, виртуализированная таблица, системные диалоги. Alternatives: Win32 вручную сложнее поддерживать; Tauri требует web UI. Sources: https://docs.rs/eframe/0.33.3/eframe/trait.App.html ; https://docs.rs/rfd/0.17.2/rfd/.
- Decision: MSVC toolchain с Windows SDK. Rationale: штатная Windows-сборка. Source: https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup.
- Decision: WTS/NetAPI FFI и существующие штатные Windows-команды; отдельный JSON с импортом WPF. Rationale: сохранить полномочия и совместимость, избежать взаимной перезаписи настроек.
- Decision: системный Segoe UI для кириллицы; не распространять файл шрифта. Glow проверить в RDP отдельно, возможны ограничения драйвера OpenGL.
