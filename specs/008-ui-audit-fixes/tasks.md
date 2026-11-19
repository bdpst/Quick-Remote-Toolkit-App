# Tasks: Исправления интерфейса
## Setup
- [X] T001 Зафиксировать материалы UX-01–08 и baseline AC-01–08 в specs/008-ui-audit-fixes/validation.md.
## Foundational
- [X] T002 Сохранить совместимость UX-01/AC-01 в src/QuickRemoteToolkit.Rust/src/ui.rs.
## US1 — Типографика
- [X] T003 [US1] UX-05/AC-05. Обеспечить читаемые поля и полный текст выбранной записи в src/QuickRemoteToolkit.Rust/src/ui.rs и src/QuickRemoteToolkit.Rust/src/design.rs.
## US2 — Компоновка
- [X] T004 [US2] UX-01/AC-01 и UX-03/AC-03. Связать команды с целью и добавить восстановление списка в src/QuickRemoteToolkit.Rust/src/ui.rs.
## US3 — Размеры и клавиатура
- [X] T005 [US3] UX-02/AC-02 и UX-06/AC-06. Настроить колонки, достижимость команд, фокус и splitter в src/QuickRemoteToolkit.Rust/src/ui.rs и src/QuickRemoteToolkit.Rust/src/design.rs.
## US4 — Состояния
- [X] T006 [US4] UX-04/AC-04, UX-07/AC-07, UX-08/AC-08. Согласовать контраст, ожидание и ошибки в src/QuickRemoteToolkit.Rust/src/design.rs, src/QuickRemoteToolkit.Rust/src/ui.rs, docs/design-tokens.json и docs/design-guidelines.md.
## Проверки
- [X] T007 Выполнить сборку, тесты, smoke и поставку для AC-01–08. Записать в specs/008-ui-audit-fixes/validation.md.
- [ ] T008 Вручную пройти AC-01–08 и Windows DPI, записать в specs/008-ui-audit-fixes/validation.md.

Порядок T001 → T002 → T003 → T004 → T005 → T006 → T007/T008. Независимые проверки T007 и T008 возможны после сборки. Независимые критерии каждой истории перечислены в spec.md. Первый проверяемый результат — чтение полных данных.

T008 выполнена частично. Ручные сценарии и ограничения отмечены в validation.md.
