> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Data Model
- ClientEntry.IsFavorite: bool, runtime observable.
- ClientEntry.FavoriteKey: normalized computer:name, fallback ip:address; empty if both absent or '-'.
- ClientEntry.CanFavorite: key is not empty.
- AppSettings.FavoriteComputers: List<string>, default empty; accept null from old/manual JSON as empty. Keep absent computers; deduplicate on update.
- AppSettings.ShowFavoritesOnly: bool, default false.
Transitions: save candidate -> update all rows sharing key -> refresh view. On save failure restore settings and show error.
