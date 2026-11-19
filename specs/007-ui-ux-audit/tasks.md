# Tasks: UI/UX audit

**Input**: spec.md, plan.md, research.md, data-model.md, contracts/evidence.md, quickstart.md.

## Phase 1: Setup
- [X] T001 Зафиксировать чистое дерево, main commit и рабочую ветку в specs/007-ui-ux-audit/research.md
## Phase 2: Foundational
- [X] T002 Сопоставить ui.rs/design.rs/QA host и правила доказательств в specs/007-ui-ux-audit/contracts/evidence.md
## Phase 3: US1 — доказательный аудит
**Independent Test**: каждый сценарий имеет честный статус, каждая находка — все поля evidence contract.
- [X] T003 [US1] Выполнить smoke обеих тем/размеров и сохранить отобранные снимки в specs/007-ui-ux-audit/evidence/
- [X] T004 [US1] Проверить доступность безопасного интерактивного прохода и описать фактическое покрытие/DPI и блокеры в specs/007-ui-ux-audit/validation.md
- [X] T005 [US1] Подготовить critique, находки, гипотезы и приоритеты в specs/007-ui-ux-audit/audit.md
## Phase 4: US2 — принятый гайдлайн
**Independent Test**: все категории FR-004 имеют действующее правило и источник; предложения отделены.
- [X] T006 [P] [US2] Обновить docs/design-guidelines.md по фактическим правилам и значениям
## Phase 5: Polish
- [X] T007 Проверить ссылки, scope diff, данные и требования в specs/007-ui-ux-audit/validation.md

## Dependencies
T001 → T002 → T003 → T004 → T005 → T007; T006 после T002/T003, независим от оформления T005.
## Parallel examples
US1: визуальная оценка evidence и read-only исследование исходников могут выполняться независимо после T003. US2: T006 параллелен оформлению T005, файлы разные.
## Implementation Strategy
MVP: US1 с проверенными доказательствами; затем US2; финальная проверка документов. Не реализовывать будущие исправления, не публиковать. Непроведённые физические/сетевые проверки сохранять открытыми в validation.md.


T004: интерактивная попытка ограничена capture/input errors; ручные сценарии NOT RUN/PARTIAL в validation.md не закрыты. T001–T007 завершены как задачи подготовки аудита/документации, не как реализация UX-исправлений.
