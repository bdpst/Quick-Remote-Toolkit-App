# Implementation Plan: UI/UX audit

**Branch**: `codex/007-ui-ux-audit` | **Date**: 2026-11-19 | **Spec**: [spec.md](spec.md)

## Summary
Собрать framebuffer и интерактивные доказательства актуального Rust UI, сопоставить с исходниками, обновить существующий гайд; код не менять.

## Technical Context
**Language/Version**: Rust 1.99.1 (только исследование), Markdown.
**Primary Dependencies**: egui/eframe 0.33; штатный ui-qa host; Windows computer-use.
**Storage**: изолированный APPDATA и синтетический CSV в ignored publish-rust/.
**Testing**: ui-smoke.ps1 в 1500×920 и 1200×760; интерактивный безопасный проход; проверки ссылок/diff.
**Target Platform**: Windows x64 MSVC.
**Project Type**: desktop-app; результат задачи — документация.
**Performance Goals**: не меняются; задержки UI фиксировать наблюдением, не выдавать за benchmark.
**Constraints**: без рабочих ПК, реальных данных, изменений исходников, публикации и merge.
**Scale/Scope**: главное окно, два прикладных диалога, системные диалоги импорта/экспорта и безопасные состояния.

## Constitution Check
До исследования и после проектирования PASS: текущая сессия, изоляция данных, Rust-only, честные ограничения, неизвестный timeout не равен успеху. Контракты CSV/JSON и бренд не меняются. Release gates не применяются к документации; ui-smoke требует сборки QA-host.

## Project Structure
`docs/design-guidelines.md` — редактируется; `docs/design-tokens.json` и `src/QuickRemoteToolkit.Rust/src/` — read-only.
`specs/007-ui-ux-audit/` — spec, plan, research, data-model, contracts/evidence.md, quickstart, tasks, audit, validation, evidence/.
`publish-rust/audit-007-*` — ignored сырые тестовые профили/снимки/результаты.

## Phase 0: Research
Исследовать актуальность main, границу возможностей smoke, соответствие палитры/геометрии, клавиатуру и статусы. Решения в research.md.

## Phase 1: Design
Единица аудита и контракт доказательств описаны в data-model.md и contracts/evidence.md. Сценарии воспроизведения — quickstart.md. Разделять факт, оценку и гипотезу.
