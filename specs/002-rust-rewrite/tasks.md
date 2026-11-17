> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Tasks: Rust rewrite
## Setup
- [x] T001 Создать ветку и документы в specs/002-rust-rewrite/.
- [x] T002 Создать Cargo проект и ignore в src/QuickRemoteToolkit.Rust/Cargo.toml и .gitignore.
## Foundational
- [x] T003 Модели, JSON и канал событий в src/QuickRemoteToolkit.Rust/src/models.rs.
## US1 — Список
- [x] T004 [US1] CSV обеих кодировок в src/QuickRemoteToolkit.Rust/src/services.rs.
- [x] T005 [US1] Таблица, поиск, копирование, избранное в src/QuickRemoteToolkit.Rust/src/ui.rs.
## US2 — Поддержка
- [x] T006 [US2] Команды и ограниченная проверка в src/QuickRemoteToolkit.Rust/src/services.rs.
- [x] T007 [US2] NetAPI домен в src/QuickRemoteToolkit.Rust/src/native.rs.
## US3 — Администрирование
- [x] T008 [US3] WTS сообщения в src/QuickRemoteToolkit.Rust/src/native.rs.
- [x] T009 [US3] PowerShell и окно подтверждения в src/QuickRemoteToolkit.Rust/src/services.rs и ui.rs.
## US4 — Оформление
- [x] T010 [US4] Темы, настройки, экспорт в src/QuickRemoteToolkit.Rust/src/ui.rs.
## Verification
- [x] T011 Проверить cargo fmt/test/clippy/build и записать результаты в specs/002-rust-rewrite/validation.md.
- [x] T012 Выполнить dotnet build/publish и Rust publish через src/QuickRemoteToolkit.Rust/build.ps1.
- [ ] T013 Проверить UI обеих тем и удалённый паритет по specs/002-rust-rewrite/quickstart.md.
## Dependencies
T001 → T002 → T003 → T004/T006/T007/T008 → T005/T009/T010 → T011 → T012 → T013.
## Parallel opportunities
После моделей CSV и Win32 API допускают независимую работу; UI интегрируется последовательно. Все изменения выполняются в текущем чате, исследование GUI выполнено отдельным исследователем по speckit-plan.
## Implementation strategy
Сначала список как MVP, затем все удалённые функции и оформление. Не объявлять полный перенос завершённым до проверки паритета.

## Перенос дизайна WPF
- [x] T014 [US4] Перенести палитры, карточки, заголовок, панели, таблицы, иконки и оформление диалогов в src/QuickRemoteToolkit.Rust/src/design.rs и src/QuickRemoteToolkit.Rust/src/ui.rs.
- [ ] T015 [US4] Подтвердить внешний вид обеих тем и масштабирование вручную по specs/002-rust-rewrite/validation.md; автоматический захват окна возвращает тайм-аут.
