# Implementation Plan
Ветка codex/008-ui-audit-fixes. [Спецификация](spec.md).

## Technical Context
Rust 1.99.1, Windows x64 MSVC, egui/eframe, существующие ui.rs, columns.rs, design.rs и QA host. CSV, JSON и AppData сохраняются.

## Constitution Check
PASS до и после проектирования: правки ограничены представлением и управлением UI. Сетевые команды, разрешения и правила интерпретации результатов сохраняются. Проверки и упаковка согласно README.

## Project Structure
Код в src/QuickRemoteToolkit.Rust/src/. Правила оформления в docs/. Приёмка и снимки в текущей спецификации. Поставка в publish-rust/.

Фазы: типографика и доступ к полному тексту → выбор цели и композиция → компактный viewport и splitter → состояния, токены и проверки.
