> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Data model
Client: number i32, computer/ip/person строки, status/domain runtime строки. Computer нормализуется trim + lowercase для избранного.
Settings: PascalCase имена WPF, ClientsCsvPath, PingTimeoutMs (100–30000), ParallelPingLimit (1–128), FavoriteComputers, ShowFavoritesOnly, IsDarkTheme. Rust файл settings-rust.json; исходный settings.json только читается для импорта.
Log: локальное время, цель, действие, результат; текст сообщений исключён.
Background events: Log, ProbeResult, Finished. UI применяет результаты через канал; повторные административные операции отключены до Finished.
