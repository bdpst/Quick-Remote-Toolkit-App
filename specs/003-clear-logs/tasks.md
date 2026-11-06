# Tasks: Очистка журнала

**Input**: specs/003-clear-logs/spec.md, plan.md и design artifacts.

## Phase 1: Setup
- [x] T001 Зафиксировать поведение и проверить требования в specs/003-clear-logs/spec.md и checklists/requirements.md.

## Phase 2: Foundational
- [x] T002 Проверить общий источник данных и стили в src/QuickRemoteToolkit.App/MainWindow.xaml.cs и App.xaml; описать решение в specs/003-clear-logs/research.md.

## Phase 3: User Story 1 (P1)
Цель: очистить журнал одним нажатием. Независимая проверка: текущие записи удаляются, следующие поступают.
- [x] T003 [US1] Добавить кнопку справа в заголовке журнала с ToolButtonStyle в src/QuickRemoteToolkit.App/MainWindow.xaml.
- [x] T004 [US1] Добавить ClearLogs_Click с очисткой всего набора в src/QuickRemoteToolkit.App/MainWindow.xaml.cs.

## Phase 4: Validation
- [x] T005 Проверить сборку и сценарии из specs/003-clear-logs/quickstart.md; записать результат в validation.md.
- [x] T006 Выполнить публикацию src/QuickRemoteToolkit.App/QuickRemoteToolkit.App.csproj в publish/ и проверить итоговый diff.

## Dependencies & Parallel Execution
T001 → T002 → T003 → T004 → T005 → T006. Для этой небольшой функции параллельная работа не нужна.

## Implementation Strategy
Единственная история US1 — полный MVP. Две задачи реализации, две проверки, две подготовительные задачи; всего 6. Каждый пункт содержит ID и пути; US1 помечена в обоих пунктах реализации.

