# AGENTS.md

Инструкции для Codex и других помощников, работающих с этим репозиторием.

## Проект

Quick Remote Toolkit App - Windows-приложение, переносимое на Rust (WPF сохраняется как эталон) для быстрых действий удаленной поддержки Windows-клиентов.

Основные возможности:

- чтение списка клиентов из CSV;
- запуск Remote Assistance, ping, tracert, C$, Event Viewer, Computer Management, MSTSC, WinRS cmd и gpupdate;
- параллельная проверка доступности;
- журнал действий и экспорт логов.

## Структура

- `src/QuickRemoteToolkit.Rust/` - целевое Rust-приложение: models, services, native, ui, design.
- `src/QuickRemoteToolkit.App/` - сохраняемый WPF-эталон.
- `src/QuickRemoteToolkit.App/Models/` - модели данных.
- `src/QuickRemoteToolkit.App/Services/` - работа с CSV, настройками и запуском удаленных действий.
- `src/QuickRemoteToolkit.App/Assets/` - иконка приложения.
- `samples/clients.example.csv` - пример CSV без реальных данных.
- `.github/workflows/release.yml` - проверки Rust и portable build artifact для Windows.
- `publish/` - локальная публикация, не считать источником правды для кода.

## Команды

Проверки Rust (Windows x64 MSVC):

```powershell
cargo fmt --check --manifest-path .\src\QuickRemoteToolkit.Rust\Cargo.toml
cargo clippy --locked --manifest-path .\src\QuickRemoteToolkit.Rust\Cargo.toml --all-targets -- -D warnings
cargo test --locked --manifest-path .\src\QuickRemoteToolkit.Rust\Cargo.toml
cargo build --release --locked --manifest-path .\src\QuickRemoteToolkit.Rust\Cargo.toml
.\src\QuickRemoteToolkit.Rust\build.ps1 -OutputDirectory .\publish-rust
```

WPF при изменении его кода: dotnet build solution Release --no-restore, затем
 dotnet publish csproj Release --no-restore -o ./publish. Не заменять WPF-поставку
до проверки Rust-паритета. Источник эталона — SHA в docs/design-guidelines.md.

Проверка статуса Git:

```powershell
git status --short
```

## CSV

Формат списка клиентов:

```csv
number;computer;ip;person
1;PC-NAME;192.168.1.10;User One
```

Не добавлять реальные рабочие данные клиентов в репозиторий. Для примеров использовать только обезличенные значения.

## Важные замечания по удаленным действиям

Приложение не хранит и не передает пароли. Все действия запускаются от текущей Windows-сессии пользователя.

- `C$` требует SMB-доступа и прав локального администратора на целевом ПК.
- `WinRS` требует рабочий WinRM и подходящие права на целевом ПК.
- Если `dir \\PC-NAME\c$` или `winrs -r:PC-NAME hostname` не работают вручную, проблема не в приложении.
- Не добавлять хранение паролей в код без отдельного решения по безопасному хранению учетных данных.

## Стиль разработки

- Держать изменения небольшими и проверяемыми.
- Не коммитить `publish/`, временные файлы и реальные CSV с клиентами.
- Не менять поведение удаленных команд без ручной проверки аналогичной команды в PowerShell/CMD.
- Для UI соблюдать docs/design-guidelines.md, docs/design-tokens.json и ui-parity-checklist.md.
- После изменения Rust выполнить Cargo-проверки выше и успешную публикацию в publish-rust/.
- WPF publish/ сохранять для сравнения. Не коммитить publish-rust/, target/ и архивы поставки.
- Не считать старт процесса успешным завершением удалённого действия; блокеры проверки фиксировать.

## Релизы

Тег вида `v0.2.1` запускает сборку portable-архива Windows. Workflow загружает build artifact;
GitHub Release не создаётся автоматически. В задаче переноса теги и релизы не публиковать.

```powershell
git tag -a v0.2.1 -m "Release v0.2.1"
git push origin v0.2.1
```

GitHub Actions собирает архив релиза автоматически.
