# Implementation Plan: Rust rewrite
**Branch**: `codex/rust-rewrite` | **Date**: 2026-11-06 | **Spec**: [spec.md](spec.md)
## Summary
Полный перенос UI и сервисов в отдельный Rust executable. WPF остаётся эталоном.
## Technical Context
Rust stable, edition 2024; eframe/egui_extras =0.33.3, rfd =0.17.2, serde/serde_json, encoding_rs, chrono, base64. Windows x64 MSVC. Локальный JSON в AppData, CSV без изменений. cargo test, clippy, release build; ручная проверка UI и удалённых команд. Виртуализированная таблица для 1000+ строк. Фоновые потоки и канал событий с ограничением параллелизма.
## Constitution Check
Текущая сессия, отсутствие паролей, безопасные аргументы, отсутствие текста сообщений в логах сохраняются. WPF UI-правила применяются к WPF; в Rust сохраняются обе темы и структура. Запрос пользователя разрешает новый стек в отдельной ветке; WPF и его workflow не заменяются. После Rust-изменений также выполнить обязательные dotnet build/publish. Проверить повторно после реализации.
## Project Structure
`src/QuickRemoteToolkit.Rust/`: Cargo.toml, Cargo.lock, src/main.rs, models.rs, services.rs, native.rs, ui.rs, README.md, build.ps1. Документы в specs/002-rust-rewrite.
## Complexity Tracking
Win32 FFI нужен для WTS и NetGetJoinInformation; unsafe ограничен native.rs. PowerShell остаётся механизмом изменения членства. Пользовательские данные передаются через литералы и EncodedCommand.
