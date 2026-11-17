# Data model
Client: number i32, computer/ip/person строки, status/domain runtime строки. Computer нормализуется trim + lowercase для избранного.
Settings: PascalCase имена WPF, ClientsCsvPath, PingTimeoutMs (100–30000), ParallelPingLimit (1–128), FavoriteComputers, ShowFavoritesOnly, IsDarkTheme. Rust файл settings-rust.json; исходный settings.json только читается для импорта.
Log: локальное время, цель, действие, результат; текст сообщений исключён.
Background events: Log, ProbeResult, Finished. UI применяет результаты через канал; повторные административные операции отключены до Finished.
