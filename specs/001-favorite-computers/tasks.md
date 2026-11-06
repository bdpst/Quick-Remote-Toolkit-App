# Tasks: Избранные компьютеры
## Phase 1: Setup
- [X] T001 Проверить конституцию и ignore-файлы; создать артефакты specs/001-favorite-computers/.
## Phase 2: Foundation
- [X] T002 Добавить ключ: имя без учета регистра, резервный IP, пустой ключ недоступен; IsFavorite в src/QuickRemoteToolkit.App/Models/ClientEntry.cs.
## Phase 3: US1 — звездочка (MVP)
Independent test: два нажатия меняют состояние; дубликаты согласованы.
- [X] T003 [US1] Добавить кнопку звездочки и сохранение с обновлением дубликатов в src/QuickRemoteToolkit.App/MainWindow.xaml и .cs.
## Phase 4: US3 — сохранение
Independent test: перезапуск и reload сохраняют отметки.
- [X] T004 [US3] Добавить FavoriteComputers (default empty, null as empty), ShowFavoritesOnly (default false) в src/QuickRemoteToolkit.App/Models/AppSettings.cs; восстановление в MainWindow.xaml.cs.
- [X] T005 [US3] Обеспечить атомарную запись и обработку отказа в src/QuickRemoteToolkit.App/Services/SettingsService.cs и MainWindow.xaml.cs.
## Phase 5: US2 — фильтр
Independent test: пересечение поиска и избранного, снятие последней отметки.
- [X] T006 [US2] Добавить переключатель и пересечение с поиском в src/QuickRemoteToolkit.App/MainWindow.xaml и .cs.
## Phase 6: Validation
- [X] T007 Проверить Release, поведение и обе темы; записать результаты в specs/001-favorite-computers/validation.md.
## Dependencies & Execution Order
T001 -> T002 -> T003/T004/T005 (единая интеграция) -> T006 -> T007.
Параллельных задач нет из-за общих файлов. Стратегия: звездочка (MVP), сохранение, фильтрация, проверка полного результата. Все три истории входят в поставку.

