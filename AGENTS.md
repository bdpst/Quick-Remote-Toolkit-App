# AGENTS.md

Инструкции для Codex и других помощников, работающих с этим репозиторием.

## Проект

Quick Remote Toolkit App - WPF-приложение на C#/.NET для быстрых действий удаленной поддержки Windows-клиентов.

Основные возможности:

- чтение списка клиентов из CSV;
- запуск Remote Assistance, ping, tracert, C$, Event Viewer, Computer Management, MSTSC, WinRS cmd и gpupdate;
- параллельная проверка доступности;
- журнал действий и экспорт логов.

## Структура

- `src/QuickRemoteToolkit.App/` - исходный код WPF-приложения.
- `src/QuickRemoteToolkit.App/Models/` - модели данных.
- `src/QuickRemoteToolkit.App/Services/` - работа с CSV, настройками и запуском удаленных действий.
- `src/QuickRemoteToolkit.App/Assets/` - иконка приложения.
- `samples/clients.example.csv` - пример CSV без реальных данных.
- `.github/workflows/release.yml` - сборка GitHub Release.
- `publish/` - локальная публикация, не считать источником правды для кода.

## Команды

Сборка:

```powershell
dotnet build .\QuickRemoteToolkit.App.sln -c Release --no-restore
```

Публикация локальной portable-версии:

```powershell
dotnet publish .\src\QuickRemoteToolkit.App\QuickRemoteToolkit.App.csproj -c Release --no-restore -o .\publish
```

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
- Для UI соблюдать текущий светлый дизайн, общие стили из `App.xaml` и существующие размеры/отступы.
- После изменений запускать `dotnet build`.

## Релизы

Релиз создается пушем тега вида `v0.2.1`.

```powershell
git tag -a v0.2.1 -m "Release v0.2.1"
git push origin v0.2.1
```

GitHub Actions собирает архив релиза автоматически.
