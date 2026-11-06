# Implementation Plan: Очистка журнала

**Branch**: текущая рабочая ветка | **Date**: 2026-11-06 | **Spec**: [spec.md](spec.md)
**Input**: specs/003-clear-logs/spec.md

## Summary
Добавить кнопку в заголовок журнала и обработчик очистки существующей коллекции. Не менять экспорт или удалённые команды.

## Technical Context
**Language/Version**: C# / .NET 10.
**Primary Dependencies**: WPF; новых зависимостей нет.
**Storage**: ObservableCollection<LogEntry> в MainWindow, только память.
**Testing**: Release build, проверка WPF-сценария и обеих тем, publish.
**Target Platform**: Windows.
**Project Type**: desktop-app.
**Performance Goals**: одно действие очищает весь текущий журнал.
**Constraints**: общий ToolButtonStyle и DynamicResource; UI dispatcher сериализует очистку и новые записи.
**Scale/Scope**: одно окно, одна кнопка, один обработчик.

## Constitution Check
До и после проектирования: PASS. Пароли, удалённые команды, CSV и выбор клиента не затрагиваются. Минимальный diff; существующие ресурсы тем; экспорт читает ту же коллекцию. Проверки сборки, сценария и публикации обязательны. Внешнее исследование не требуется: поведение полностью определяется локальным кодом.

## Project Structure
Документы: spec.md, checklists/requirements.md, plan.md, research.md, data-model.md, contracts/ui.md, quickstart.md, tasks.md в specs/003-clear-logs/.
Исходники: src/QuickRemoteToolkit.App/MainWindow.xaml и MainWindow.xaml.cs.
**Structure Decision**: сохранить текущую структуру WPF; новые сервисы не нужны.

## Complexity Tracking
Нарушений и дополнительной сложности нет.
