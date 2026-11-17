> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Проверки переноса — 2026-11-16

## Эталон и процесс

Рабочая ветка codex/rust-rewrite. Эталон main/origin/main после fetch:
91689516c96d52877bba3480bf498b7d807ead64. main не продвинулась относительно дизайн-гайда.
Файлы эталона исследованы отдельно через ref/archive. Очистка журнала сохранена,
хотя коммит Rust-ветки отменял её; WPF не изменялся. Существующие изменения пользователя
в README/.gitignore, skills/.specify, specs/002 и прототипах сохранены и продолжены по задаче.

Spec Kit 1.3.1, Codex skills/PowerShell: прочитаны и исполнены constitution, specify,
plan/research, tasks, analyze, implement, converge. Прямой slash API отсутствует,
skills не запускались как shell commands. Использованы resolve-template, setup-plan,
setup-tasks, check-prerequisites. Активный каталог задан через штатный
SPECIFY_FEATURE_DIRECTORY=specs/005-rust-parity и .specify/feature.json. Ветка сохранена.
extensions.yml отсутствует, hooks неприменимы. Clarify не требовался: пользовательский
контракт определён запросом и исходниками. Конституция 2.0.0 (MAJOR) и AGENTS обновлены
по прямому разрешению пользователя; старые specs не переписаны. Временный Sync Impact
Report оставлен в конституции до будущего коммита.

Анализ spec/plan/tasks до implement: 8 FR, 11 исходных задач, покрытие 100%,
существенных конфликтов/неопределённостей 0, checklist требований 4/4.
Plan research GUI выполнен отдельным агентом gui_research. Сохранён egui/eframe:
нужные API присутствуют в закреплённой версии, другой стек потребовал бы повторного
переноса. Это решение не подменяет визуальную проверку.

## Автоматические проверки Windows

- cargo fmt --check — PASS.
- cargo clippy --locked --all-targets -- -D warnings — PASS, предупреждений кода приложения нет.
- cargo test --locked — 25 PASS, 0 failed.
- Cargo host Windows x64 MSVC; release --locked и финальная статическая CRT линковка
  /Brepro выполняются build.ps1. Финальные размеры/хеши — artifact-validation.json.
- Команды используют --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml.
- Cargo host печатает `could not canonicalize path C:\Users\<user>`; это предупреждение
  окружения launcher, не подавленное предупреждение собственного Rust-кода.

Тесты проверяют обе CSV-кодировки, пустые/ошибочные строки, 0 и отрицательные номера,
settings PascalCase/defaults/BOM/unknown fields/atomic backup и ошибочный JSON,
ключи избранного/дубликаты/fallback/no-empty, поиск домена/trim, Target IP «-»,
безопасные цели/PowerShell quotes/кириллицу, CSV export escaping/TXT/empty,
лимит worker с подменным ProbeAdapter, ошибку, закрытый receiver, late generation,
все команды через ActionAdapter без запуска удалённых эффектов, формат аккаунта,
параллельные mock WTS ответы OK/Cancel/timeout/other/error, локальный безопасный
PowerShell timeout без удалённой операции, UTF-16 лимиты и ICMP loopback.

Headless egui-тест рендерит 1000 строк в обеих темах, проверяет сортированную навигацию,
сброс скрытого выбора, ограниченность числа shapes и отсутствие изменения набора.
Это проверка логики/layout без пиксельной приёмки и без доказательства Windows input.

## Совместимость данных

Дополнительно во временном каталоге выполнен реальный round-trip:
C# AppSettings эталона + System.Text.Json (.NET SDK 10.0.302) → JSON → текущий
Rust models::Settings + serde → JSON → C# AppSettings. Сохранены все проверяемые
значения: путь с кириллицей, PingTimeoutMs, ParallelPingLimit, FavoriteComputers,
ShowFavoritesOnly, IsDarkTheme. Результат: PASS WPF -> Rust -> WPF.
Временный verifier импортировал только Settings из общего models.rs, поэтому его
компилятор сообщил о неиспользуемых Client/Log/Event; это не предупреждения сборки
приложения, которая проходит clippy -D warnings. Verifier не входит в поставку.

Общий settings.json сохраняется атомарно, до первой перезаписи создаётся
settings.before-rust.json. Unknown поля сохраняются, повреждённый JSON не заменяется
defaults. Старый prototype settings-rust.json не подменяет WPF settings.json.
WPF/Rust не следует одновременно запускать для изменения одного settings.json.

## UI и реальные интеграции

Сборка запущена на Windows 10 с отдельным APPDATA и обезличенными PC-DEMO/192.0.2.x.
Sandbox-процесс не дал targetable window; вне sandbox окно зарегистрировалось,
UI Automation выдала дерево главного окна: панели, кнопки, поиск, таблица,
избранное и журнал. Это подтверждает создание настоящего Windows UI.

Сначала системный захват кадра/geometry были недоступны. Затем получены настоящие
OpenGL framebuffer кадры через отдельный feature-gated host и WPF RenderTargetBitmap
из точного archive SHA с адаптером только пути settings и отключённым StartupUri.
Полный отчёт, 26+8+4 Rust запусков и 12+2 WPF — ui-validation.md. Доказательство
не заменяет физический Windows input/другие DPI. Ошибки Enter/Ctrl+Enter проверены
на app-level input без сетевой операции. WPF-диалоги имеют дефект тёмной темы;
Rust использует читаемый CardBackground. Raster/wrapping не идентичны эталону.

Финальный статический EXE из ZIP запускался с отдельным APPDATA, PATH только System32,
без DOTNET_ROOT/CARGO_HOME/RUSTUP_HOME. Окно зарегистрировано; UIA финального исправленного архива подтвердила PC-DEMO, кнопки и Load CSV;
SetValue через Sky получил GetCursorPos access denied. Только свой тестовый процесс завершён.
Личные настройки и работающий WPF пользователя не использовались тестами.

Реальные SMB/WinRM/WTS/домен и изменения членства не выполнялись: явно разрешённых
тестовых ПК нет. Наличие адаптеров и unit-тестов не доказывает реальную интеграцию.
Чистая машина без средств разработки и .NET не предоставлена. PE-проверка подтверждает
нативный x64 и отсутствие CLR/импортов hostfxr/mscoree; статическая CRT устраняет
VCRUNTIME/MSVCP зависимость. Для запуска остаются системные DLL и OpenGL-драйвер.

## Поставка и намеренные отличия

publish-rust/ содержит EXE, RUNTIME.txt и ZIP. ZIP имеет упорядоченные записи,
фиксированные timestamps 1980-01-01; PE линкуется /Brepro, CRT статически.
Иконка встроена, WPF и publish/ сохранены. CI обновлён под Cargo/Windows,
создаёт build artifact; удалённо не запускался, ничего не пушилось.

Намеренные отличия от дефектов эталона: CSV export экранируется; injection targets
отклоняются; повторные probe/reload защищены; ошибки настройки/экспорта сообщаются
без аварийного завершения. ParallelPingLimit ограничен эффективно 1–128 worker,
при этом значение настройки не переписывается. Текст сообщения не журналируется.
Точное соответствие системных рамок native viewport и WPF, поведения выбора/фокуса,
всех hover/pressed/disabled состояний требует P01–P16, а не предположения по коду.

## Converge

Проверены 8 FR/4 SC, решения плана, 5 принципов конституции и 11 задач независимо
от старых checkbox. Три actionable finding: partial HIGH FR-005/SC-003 (UI приёмка),
partial HIGH FR-003/004/SC-001 (реальная сеть), partial MEDIUM FR-007/SC-004
(чистая среда/CI). Добавлены T012–T014, исходные задачи не перенумерованы/переписаны
этапом converge. Outcome: tasks_appended, НЕ converged. Исторические отчёты 002
не считаются приёмкой новой спецификации 005. Полный перенос пока не подтверждён.

Дополнительный read-only аудит gui_research: закрыты 5 доказанных gaps — action-specific validation, UI Error event, типизированный PingReply с RTT/Ping log, отдельный dd.MM display-time журнала, OEM net.exe decoding. Повторный просмотр подтвердил все 5 исправлений; OEM CP866/unused invalid IP/RTT включены в регрессионные проверки.

## Финальное продолжение

Добавлены columns/selection: перенос ширины логической колонки, сортировка независимо
от отображаемого порядка, Extended журнал/Ctrl+Shift/A/C, ExcludeHeader, сохранение
выбора при новых логах. Headless-тесты проверяют Copy output и фактически видимую
PC-0999 после ArrowDown. В таблицах реальные размеры widgets ограничены строкой26,
полосы прокрутки постоянно видимы при необходимости. Splitter preview и порядок
Tab соответствуют структуре исходного Grid; все физические переходы пока не проверены.

Settings используют signed i32 как WPF int, отрицательные значения сохраняются;
некорректный PingTimeoutMs даёт диагностическую ошибку, эффективный лимит 1–128.
EXE содержит icon/group/PerMonitorV2/asInvoker resources, проверенные PE script.
Повторная упаковка дала тот же SHA-256; metadata — artifact-validation.json.
Это не проверка сборки на другой машине или чистой пересборки всех зависимостей.

Финальная Unicode-сверка с фактическим .NET ToUpperInvariant: ß/ﬃ/ı/İ/σ/ς/ё/ǰ/ŉ.
Rust full case expansion объединял разные PC-ß и PC-SS и отличал поиск от OrdinalIgnoreCase.
Добавлено one-to-one invariant casing с dotless-i; regression assertions проверяют
разные ключи, Cyrillic case и поиск. JSON поля/значения при этом не мигрируются.
