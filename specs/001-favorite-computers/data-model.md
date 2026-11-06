# Data Model
- ClientEntry.IsFavorite: bool, runtime observable.
- ClientEntry.FavoriteKey: normalized computer:name, fallback ip:address; empty if both absent or '-'.
- ClientEntry.CanFavorite: key is not empty.
- AppSettings.FavoriteComputers: List<string>, default empty; accept null from old/manual JSON as empty. Keep absent computers; deduplicate on update.
- AppSettings.ShowFavoritesOnly: bool, default false.
Transitions: save candidate -> update all rows sharing key -> refresh view. On save failure restore settings and show error.
