# Tasks: Эталон интерфейса
**Input**: spec.md, plan.md, research.md, data-model.md, contracts/artifacts.md.
## Phase 1: Setup
- [x] T001 Проверить установку Spec Kit и ограничения в specs/004-ui-design-reference/research.md (FR-006).
## Phase 2: Foundational
- [x] T002 Зафиксировать main и сравнение веток для docs/design-guidelines.md (FR-001).
## Phase 3: US1 — воспроизведение (P1, MVP)
**Goal**: точное описание существующего UI. **Independent Test**: найти правило каждого окна и его источник.
- [x] T003 [US1] Описать все окна, темы, компоненты и поведение в docs/design-guidelines.md (FR-002, FR-003).
- [x] T004 [US1] Зафиксировать попытку запуска, ограничения и несогласованности в docs/design-guidelines.md (FR-001, FR-005).
## Phase 4: US2 — токены (P2)
**Goal**: машиночитаемые значения. **Independent Test**: JSON разбирается, обе палитры совпадают.
- [x] T005 [US2] Создать docs/design-tokens.json: colors, sharedColors, metrics, typography, единицы и источники (FR-004).
## Phase 5: US3 — проверка паритета (P2)
**Goal**: проверяемые сценарии. **Independent Test**: все окна и темы представлены.
- [x] T006 [US3] Создать матрицу в docs/design-guidelines.md и сценарии в docs/ui-parity-checklist.md (FR-005).
## Phase 6: Polish
- [x] T007 Проверить JSON, ссылки, полноту и неизменность приложения; записать specs/004-ui-design-reference/validation.md (FR-001–006, SC-001–003).
## Dependencies
T001 → T002 → T003 → T004 → T005 → T006 → T007. Параллельных задач нет: токены сверяются с гайдом, чеклист — с окончательными правилами. MVP: US1. Затем US2 и US3, итоговая проверка.
