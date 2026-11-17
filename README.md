# Quick Remote Toolkit App

Графическая Windows-утилита для быстрых действий удаленной поддержки.

Целевой стек — Rust. Сохраняемая C#/WPF-версия служит эталоном переноса.
Сборка/запуск Rust описаны в разделе «Перенос на Rust» ниже. Изображения ниже — WPF,
они не подтверждают текущий внешний вид Rust.

<img width="1500" height="920" alt="изображение" src="https://github.com/user-attachments/assets/ac25ffdb-78e7-4fe1-86e8-d009f6bde4ed" />
<img width="1500" height="920" alt="изображение" src="https://github.com/user-attachments/assets/9c489bac-233d-409a-b28c-42ffc7a0cc50" />




## Возможности

- чтение клиентов из CSV `number;computer;ip;person`;
- поддержка CSV в Windows-1251 и UTF-8;
- поиск по компьютеру, IP, сотруднику и домену;
- копирование имени ПК, IP и имени сотрудника кнопками, появляющимися при наведении на ячейку, или через контекстное меню;
- действия: Remote Assistance, ping, tracert, `\\PC\c$`, Event Viewer, Computer Management, mstsc, WinRS cmd, WinRS gpupdate;
- автоматическая подготовка SMB-доступа к административным шарам от текущего пользователя Windows;
- параллельная проверка доступности клиентов;
- определение домена или рабочей группы во время проверки доступности;
- журнал действий в окне;
- отправка сообщения пользователям выбранного ПК с настройкой времени показа и ожиданием ответа;
- добавление и удаление пользователя из локальных администраторов выбранного ПК через PowerShell Remoting;
- сворачиваемые блоки «Подключение», «Инструменты» и «Операции»: нажмите на заголовок блока;
- экспорт журнала в TXT по умолчанию, CSV доступен как дополнительный формат;
- светлая и тёмная темы с сохранением выбранного режима;
- настройки в `%AppData%\QuickRemoteToolkitApp\settings.json`.

## Работа со Spec Kit

В проект подключён GitHub Spec Kit 1.3.1 для Codex с PowerShell-скриптами.
Общие шаблоны и процесс находятся в `.specify/`, навыки Codex — в `.agents/skills/`.
Эти файлы следует хранить в Git вместе с проектом. Правила репозитория — в `AGENTS.md`.

После установки перезапустите Codex, чтобы он обнаружил новые навыки.
В чате Codex (не в PowerShell) начните с:

```text
$speckit-constitution Сформируй принципы проекта на основе AGENTS.md и текущего кода.
```

Действующая `.specify/memory/constitution.md` — версия 2.0.0 с правилами перехода на Rust.
Для новой функции используйте `$speckit-specify <описание>`, затем
`$speckit-plan`, `$speckit-tasks` и `$speckit-implement`.
Спецификации функций сохраняются в `specs/`; небольшие исправления можно выполнять обычным запросом.

CLI установлен в пользовательское окружение через `uv`. В новом PowerShell:

```powershell
specify version
specify integration status
```

Для установки CLI на другом компьютере:

```powershell
uv tool install --python 3.12 specify-cli --from git+https://github.com/github/spec-kit.git@v1.3.1
```

Повторно инициализировать уже клонированный проект не требуется.

## Сборка сохранённого WPF-эталона

Эти команды нужны только для сравнения и отката. Основная Rust-сборка описана ниже. Для WPF нужен .NET SDK 10 или новее.

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

Локальная публикация выше является framework-dependent: она компактная, но требует установленный .NET Desktop Runtime.

## GitHub Actions

Текущий workflow `.github/workflows/release.yml` проверяет Rust на `windows-latest`
и сохраняет portable-архив `QuickRemoteToolkit.Rust-win-x64.zip` как build artifact.
Он не создаёт GitHub Release. В рамках переноса теги, релизы и merge в main не выполняются.

## CSV

Формат:

```csv
number;computer;ip;person
1;PC-NAME;192.168.1.10;User One
```

Можно использовать тот же `QuickRemoteToolkit.clients.csv`, что и в batch-версии.

## Админ-доступ

### Сообщение пользователям ПК

Выберите клиента и нажмите «Отправить сообщение…» в панели инструментов или контекстном меню.
Введите текст (до 2000 символов) и время показа от 1 до 3600 секунд (по умолчанию 300).
Нажмите «Отправить» или Ctrl+Enter. Приложение использует Windows API
[`WTSSendMessageW`](https://learn.microsoft.com/en-us/windows/win32/api/wtsapi32/nf-wtsapi32-wtssendmessagew)
от текущего пользователя Windows, ожидая ответ отдельно для каждого активного или
отключённого пользовательского сеанса. Сообщения отправляются параллельно.
В окне получателя есть кнопки «ОК» и «Отмена». В журнале отображаются нажатие «ОК»,
закрытие без подтверждения, истечение времени ожидания или ошибка Windows с кодом.
Нажатие «ОК» подтверждает действие пользователя, но не гарантирует прочтение текста.

Требуются сетевой доступ и права просмотра сеансов и отправки сообщений на целевом ПК.
Окно приложения остаётся доступным во время ожидания ответа. Повторная отправка
доступна после завершения текущего ожидания во всех сеансах.
Текст сообщения не сохраняется в настройках или журнале.

### Локальные администраторы

Выберите ПК и откройте «Локальные администраторы…» в блоке «Инструменты» или
контекстном меню. Введите учётную запись в формате `DOMAIN\User` для доменного
пользователя или `PC-NAME\User` для локального. Выберите добавление или удаление
и нажмите «Выполнить». Результат операции и ошибки появятся в журнале.

Приложение выполняет `Invoke-Command` от текущего пользователя Windows. Группа
определяется через `Get-LocalGroup -SID 'S-1-5-32-544'`, затем вызывается
`Add-LocalGroupMember` или `Remove-LocalGroupMember`. Это позволяет работать
независимо от языка имени группы «Администраторы».

На целевом ПК должны быть доступны PowerShell Remoting (WinRM) и модуль
`Microsoft.PowerShell.LocalAccounts`; текущей учётной записи нужны административные
права. Пароли не запрашиваются и не сохраняются. Удаляется только членство в группе,
сама учётная запись остаётся.

Интерфейс остаётся доступным во время выполнения. При превышении 60 секунд ожидание
прерывается; результат может быть неизвестен, поэтому перед повтором проверьте состав
группы на целевом ПК. Повторная операция доступна после завершения текущей.

### Текущая учётная запись

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

## Перенос на Rust

Целевой стек — Rust + egui/eframe 0.33.3 (OpenGL/WGL, AccessKit), Windows x64 MSVC.
WPF-эталон сохранён в src/QuickRemoteToolkit.App и publish/. Полный проверенный
паритет пока не заявляется: [матрица](specs/005-rust-parity/parity-matrix.md),
[текущий прогресс](specs/005-rust-parity/progress.md).

```powershell
cargo fmt --check --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml
cargo clippy --locked --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml --all-targets -- -D warnings
cargo test --locked --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml
./src/QuickRemoteToolkit.Rust/build.ps1 -OutputDirectory ./publish-rust
./publish-rust/QuickRemoteToolkitRust.exe
```

Rust 1.99.1, MSVC Build Tools и Windows SDK нужны для сборки. Для запуска .NET
и инструменты разработки не нужны; необходимы Windows 10/11 x64 и драйвер с
поддержкой OpenGL backend eframe. Системные WTS/NetAPI/ICMP DLL входят в Windows;
Windows PowerShell/WinRS/SMB нужны соответствующим удалённым действиям.
Чистая машина без SDK/.NET ещё требует отдельной проверки.

Настройки совместимы с WPF: %AppData%/QuickRemoteToolkitApp/settings.json.
Перед первой перезаписью Rust сохраняет settings.before-rust.json; для отката
закройте обе версии и восстановите эту копию. Нельзя запускать обе версии
одновременно при изменении настроек. При повреждённом JSON Rust не сохраняет
пустые defaults поверх исходного файла. Старый settings-rust.json прототипа
не заменяет общий файл WPF.

GitHub Actions проверяет Rust на Windows и создаёт portable-архив как build artifact.
Изменение workflow само по себе не создаёт GitHub Release; публикация тега/релиза
в текущей задаче не выполняется. WPF при необходимости собирается прежними dotnet
командами из начала документа.

Проверки GUI запускаются отдельным test host с feature `ui-qa`; он не включается
в portable-архив. Воспроизводимые команды и границы доказательств описаны в
[отчёте проверки интерфейса](specs/005-rust-parity/ui-validation.md).
