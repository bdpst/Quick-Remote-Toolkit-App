# Quick Remote Toolkit App

Графическая Windows-утилита для быстрых действий удаленной поддержки.

Это C# / WPF версия batch-утилиты `Quick Remote Toolkit`.

<img width="1500" height="920" alt="изображение" src="https://github.com/user-attachments/assets/ac25ffdb-78e7-4fe1-86e8-d009f6bde4ed" />
<img width="1500" height="920" alt="изображение" src="https://github.com/user-attachments/assets/9c489bac-233d-409a-b28c-42ffc7a0cc50" />




## Возможности

- чтение клиентов из CSV `number;computer;ip;person`;
- поддержка CSV в Windows-1251 и UTF-8;
- поиск по компьютеру, IP и сотруднику;
- действия: Remote Assistance, ping, tracert, `\\PC\c$`, Event Viewer, Computer Management, mstsc, WinRS cmd, WinRS gpupdate;
- автоматическая подготовка SMB-доступа к административным шарам от текущего пользователя Windows;
- параллельная проверка доступности клиентов;
- журнал действий в окне;
- экспорт журнала в TXT по умолчанию, CSV доступен как дополнительный формат;
- светлая и тёмная темы с сохранением выбранного режима;
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

Локальная публикация выше является framework-dependent: она компактная, но требует установленный .NET Desktop Runtime.

## GitHub Release

В репозитории есть workflow `.github/workflows/release.yml`.

Он собирает приложение на `windows-latest`. При push тега вида `v1.0.0` workflow создаст GitHub Release и приложит `QuickRemoteToolkit.App-win-x64.zip`.

Релизный архив автономный: в него входит .NET/WPF runtime, поэтому он заметно больше batch-файла и локального framework-dependent publish. Для уменьшения размера используется single-file compression, при этом trimming намеренно не включен из-за рисков для WPF/XAML.

## CSV

Формат:

```csv
number;computer;ip;person
1;PC-NAME;192.168.1.10;User One
```

Можно использовать тот же `QuickRemoteToolkit.clients.csv`, что и в batch-версии.

## Админ-доступ

Приложение не хранит пароли. Все действия выполняются от текущей Windows-сессии.

Перед открытием административной шары `C$` приложение выполняет подготовку SMB-сессии:

```cmd
net use \\PC-NAME\IPC$ /user:DOMAIN\User
```

`DOMAIN\User` подставляется автоматически из текущей Windows-сессии.

Затем открывает:

```cmd
\\PC-NAME\c$
```

Пароль не сохраняется и не передается приложением. Windows использует текущие доменные учетные данные автоматически.
