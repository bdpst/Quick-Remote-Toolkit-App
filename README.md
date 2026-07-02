# Quick Remote Toolkit App

Графическая Windows-утилита для быстрых действий удаленной поддержки.

Это C# / WPF версия batch-утилиты `Quick Remote Toolkit`.

## Возможности MVP

- чтение клиентов из CSV `number;computer;ip;person`;
- поддержка CSV в Windows-1251 и UTF-8;
- поиск по компьютеру, IP и сотруднику;
- действия: Remote Assistance, ping, tracert, `\\PC\c$`, Event Viewer, Computer Management, mstsc, WinRS cmd, WinRS gpupdate;
- параллельная проверка доступности клиентов;
- журнал действий в окне;
- экспорт журнала в CSV;
- настройки в `%AppData%\QuickRemoteToolkitApp\settings.json`.

## Сборка

Нужен .NET SDK 10 или новее.

```powershell
dotnet build .\QuickRemoteToolkit.App.sln
```

Локальная публикация без загрузки runtime packs:

```powershell
dotnet publish .\src\QuickRemoteToolkit.App\QuickRemoteToolkit.App.csproj -c Release -o .\publish
```

Готовый `QuickRemoteToolkit.exe` появится в:

```text
publish\
```

Self-contained single-file сборка выполняется в GitHub Actions при создании релиза.

## GitHub Release

В репозитории есть workflow `.github/workflows/release.yml`.

Он собирает приложение на `windows-latest`. При push тега вида `v1.0.0` workflow создаст GitHub Release и приложит `QuickRemoteToolkit.App-win-x64.zip`.

## CSV

Формат:

```csv
number;computer;ip;person
1;PC-NAME;192.168.1.10;User One
```

Можно использовать тот же `QuickRemoteToolkit.clients.csv`, что и в batch-версии.
