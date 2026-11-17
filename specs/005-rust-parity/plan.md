> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Implementation Plan: Полный паритет Rust
**Branch**: codex/rust-rewrite | **Date**: 2026-11-16 | **Spec**: [spec.md](spec.md)
## Summary
Продолжить существующий egui/eframe, исправить контракты, UI и поставку. Эталон main 91689516c96d52877bba3480bf498b7d807ead64, fetch 2026-11-16 подтвердил отсутствие новых коммитов.
## Technical Context
Rust 1.99.1, edition 2024; eframe/egui 0.33.3, MSVC x64, Windows. serde/encoding_rs/chrono/windows-sys, rfd для системных файловых диалогов. JSON общий с WPF. Cargo test/clippy/fmt/release; UIA/снимки при доступности, мок-адаптеры для удалённых операций. Цель — полный паритет без .NET; OpenGL/WGL backend и системные Windows DLL. Данные 1000+ строк, сетевые задачи в worker/каналах.
## Constitution Check
PASS для плана по 2.0.0: текущие права, нет паролей/публикации данных, исходники WPF сохранены, publish-rust отдельно. Непроверенное не объявлять паритетом. Текущий код имеет разрывы, перечисленные задачами и матрицей; эти разрывы не разрешаются наличием старых [x].
## Project Structure
src/QuickRemoteToolkit.Rust/src: models, services, native, ui, design. specs/005-rust-parity: spec/plan/research/data-model/contracts/quickstart/tasks/progress/parity-matrix/validation. docs исходного дизайна не менять ради сокрытия несовпадений.
## Phase 0: Research
Результат исследования GUI в research.md. Контракт settings.json нового запроса приоритетнее отдельного settings-rust из старой 002. Историческую 002 не переписывать.
## Phase 1: Design
Модели и адаптеры разделены без переизобретения приложения; сортированные индексы сохраняют стабильную цель, generation-id защищает late results. Диалоги — native viewport, а не плавающая egui::Window. При невыполненной UI-проверке оставлять точное ограничение.
## Complexity Tracking
Unsafe только native.rs. Windows штатные утилиты и WTS остаются интеграциями; старое WPF не запускается внутри Rust.
