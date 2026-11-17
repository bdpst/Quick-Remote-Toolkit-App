> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Implementation Plan: Избранные компьютеры
**Date**: 2026-11-06 | **Spec**: [spec.md](spec.md)
## Summary
Добавить звездочку в таблицу, фильтр над таблицей и сохранение в существующих настройках. CSV и удаленные команды не менять.
## Technical Context
C# / net10.0-windows, WPF, System.Text.Json. Windows desktop, локальные настройки AppData. Новых зависимостей нет. Фильтрация через существующий ICollectionView; один проход по списку при изменении фильтра.
## Constitution Check
До и после проектирования: PASS. Пароли и CSV не затрагиваются; настройки в профиле; динамические кисти; команды не меняются; проверка Release и обеих тем обязательна.
## Project Structure
- Models/ClientEntry.cs: IsFavorite, FavoriteKey, CanFavorite.
- Models/AppSettings.cs: FavoriteComputers, ShowFavoritesOnly.
- MainWindow.xaml и .cs: звездочка, фильтр, координация сохранения и обновления дубликатов.
- Services/SettingsService.cs: атомарная замена файла настроек.
- specs/001-favorite-computers/: спецификация, исследования, модель, UI-контракт, проверки, задачи.
## Validation
Сборка Release, проверки идентификации/старых настроек/сохранения и интеграционный прогон UI. Изолировать настройки тестового процесса от реального профиля.
## Complexity Tracking
Исключений нет.
