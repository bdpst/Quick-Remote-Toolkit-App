# Tasks: Полный паритет Rust
## Phase 1: Setup
- [x] T001 Обновить конституцию 2.0.0/AGENTS, проверить установку и эталон (FR-008).
- [x] T002 Исследовать GUI и текущий код в specs/005-rust-parity/research.md (FR-005).
## Phase 2: Foundation
- [x] T003 Составить полную specs/005-rust-parity/parity-matrix.md и progress.md (FR-001–008).
## Phase 3: US1 — данные
- [x] T004 [US1] Исправить settings/CSV/keys/search/rollback в models.rs/services.rs/ui.rs и покрыть тестами (FR-001/002).
- [x] T005 [US1] Добавить сортировку таблиц и правильную цель/копирование в ui.rs (FR-002).
## Phase 4: US2 — операции
- [x] T006 [US2] Probe выбранного/всех, generation, checking/time/domain и adapters tests в services.rs/models.rs/ui.rs (FR-003/006).
- [x] T007 [US2] Проверить команды/SMB/PowerShell/WTS, input validation, handles, тайм-ауты и shutdown в services.rs/native.rs (FR-003/004/006).
## Phase 5: US3 — UI и поставка
- [x] T008 [US3] Полные темы/chrome/native dialogs/menu/keyboard/focus/clear/export в ui.rs/design.rs (FR-005).
- [ ] T009 [US3] Cargo-проверки и UI риски/матрица P01–P16, specs/005-rust-parity/validation.md (FR-005/006/007).
- [ ] T010 [US3] README/build.ps1/Windows CI, release archive/publish-rust и runtime-проверка (FR-007).
## Phase 6: Polish
- [x] T011 Сверить specs/005-rust-parity/parity-matrix.md, задачи и converge, исключить ложные claims (FR-008).
## Dependencies
T001→T002→T003→T004→T005→T006→T007→T008→T009→T010→T011. Каждая история проверяется независимо по quickstart/матрице. MVP — US1, цель задания — все истории. Параллелизм кода не используется: общий UI и contracts зависимы.
## Phase 7: Convergence
- [ ] T012 Выполнить реальную визуальную и интерактивную приёмку P01–P16 в обеих темах, DPI/мониторы/модальные окна/клавиатура, записать evidence в specs/005-rust-parity/validation.md per FR-005/SC-003 (partial, HIGH; capture/input geometry blocked).
- [ ] T013 На явно разрешённых тестовых ПК подтвердить SMB/WinRM/WTS/домен/тайм-ауты и ручные эквиваленты, обновить specs/005-rust-parity/parity-matrix.md per FR-003/FR-004/SC-001 (partial, HIGH; разрешённые цели не предоставлены).
- [ ] T014 Проверить чистую Windows без SDK/.NET и запустить обновлённый Windows CI, записать результаты в specs/005-rust-parity/validation.md per FR-007/SC-004 (partial, MEDIUM; отдельная чистая среда отсутствует, workflow не запущен удалённо).

## Implementation evidence update — 2026-11-16

T008: реализация завершена, приёмка вынесена в T009/T012. Получены настоящие кадры
Rust/WPF и app-level invalid Enter/Ctrl+Enter (ui-validation.md); не все физические
взаимодействия/масштабы подтверждены. T010: финальная локальная публикация/PE/repack
проверены; чистая среда/удалённый CI из T014 остаются открыты. T012 больше не блокируется
отсутствием любых кадров: остаются Windows input/100/150/200%/мониторы и raster differences.
T013/T014 не закрываются mocks, ограниченным PATH или наличием workflow. Старые
Convergence tasks сохранены без переписывания. Итоговый outcome остаётся tasks_appended.
