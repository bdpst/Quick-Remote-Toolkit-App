# Дизайн-гайдлайн Quick Remote Toolkit

## 1. Источник и достоверность

Дата: 2026-11-16, Asia/Yekaterinburg. Эталон — `main`, полный SHA **91689516c96d52877bba3480bf498b7d807ead64**. После успешного `git fetch origin` main и origin/main совпали. Рабочая ветка `codex/rust-rewrite`: `ce3b76a6c6db3ff98c73f8f89271bf8e770e11e6`. Исследованы `git show main:<path>` и отдельный `git archive main` во временном каталоге, без подмены файлов рабочей ветки.

- **Подтверждено сборкой/стартом**: x64 SDK 10.0.302 собрал эталон с 0 ошибок/предупреждений; создан отдельный процесс с обезличенным samples/clients.example.csv. Старт процесса не доказывает корректный рендер.
- **Подтверждено кодом**: все правила ниже, если прямо не сказано иначе. Прочитаны все XAML, обработчики, модели, сервисы, App.xaml.cs, проект, manifest, состав Assets.
- **Требует визуальной проверки**: все экраны и состояния. Computer Use: `FrameArrived timed out: timed out waiting on channel`; после повторного выбора/активации — `window capture timed out: timed out waiting on channel`. Снимков нет, design-reference/ не создаётся; реконструкций нет.

Процесс проверки завершён; исходные настройки восстановлены, SHA-256 совпал. Сетевые и административные операции не запускались. Приложение и publish/ не изменены; требование публикации после изменения приложения здесь не применяется.

### Обозначения источников

Все сокращения означают файл **фиксированного SHA**, а не файл Rust-ветки. Ссылки с суффиксом файла открывают эталон; рядом с правилом указан символ или элемент.

| Код | Файл эталона |
| --- | --- |
| A | [App.xaml](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/App.xaml) |
| T | [Services/ThemeService.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Services/ThemeService.cs) |
| M | [MainWindow.xaml](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/MainWindow.xaml) |
| H | [MainWindow.xaml.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/MainWindow.xaml.cs) |
| S | [SendMessageWindow.xaml](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/SendMessageWindow.xaml) |
| SH | [SendMessageWindow.xaml.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/SendMessageWindow.xaml.cs) |
| L | [LocalAdministratorsWindow.xaml](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/LocalAdministratorsWindow.xaml) |
| LH | [LocalAdministratorsWindow.xaml.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/LocalAdministratorsWindow.xaml.cs) |
| C | [Models/ClientEntry.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Models/ClientEntry.cs) |
| CFG | [Services/SettingsService.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Services/SettingsService.cs) |
| DEFAULTS | [Models/AppSettings.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Models/AppSettings.cs) |
| CSV | [Services/CsvClientStore.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Services/CsvClientStore.cs) |
| ACT | [Services/RemoteActionService.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Services/RemoteActionService.cs) |
| MSG | [Services/RemoteMessageService.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Services/RemoteMessageService.cs) |
| ADM | [Services/LocalAdministratorsService.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Services/LocalAdministratorsService.cs) |
| DOM | [Services/DomainMembershipService.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Services/DomainMembershipService.cs) |
| PROJECT | [QuickRemoteToolkit.App.csproj](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/QuickRemoteToolkit.App.csproj) |
| MANIFEST | [app.manifest](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/app.manifest) |
| START | [App.xaml.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/App.xaml.cs) |
| LOG | [Models/LogEntry.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Models/LogEntry.cs) |
| STATUS | [Models/ClientStatus.cs](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Models/ClientStatus.cs) |
| ICON | [Assets/QuickRemoteToolkit.ico](https://github.com/bdpst/Quick-Remote-Toolkit-App/blob/91689516c96d52877bba3480bf498b7d807ead64/src/QuickRemoteToolkit.App/Assets/QuickRemoteToolkit.ico) |

## 2. Главное окно и геометрия

Window: Title «Quick Remote Toolkit», 1500×920, минимум 1200×760, CenterScreen, WindowStyle None, CanResize. WindowChrome: CaptionHeight 32, ResizeBorderThickness 6, GlassFrameThickness 0, CornerRadius 0, UseAeroCaptionButtons false. Это собственная шапка. Внешняя сетка: 32 и `*`. [M: Window/WindowChrome]

Шапка: CardBackgroundBrush, нижняя граница DividerBrush толщиной 1. Слева Margin 10,0,0,0; иконка 16×16 с правым Margin 8, название. Справа три кнопки 46×32: U+E921 «Свернуть», U+E922 «Развернуть» либо U+E923 «Восстановить», U+E8BB «Закрыть». Font Segoe MDL2 Assets 10. [M: верхний Border; A: TitleBarButtonStyle; H: UpdateMaximizeRestoreButton]

Одинарное нажатие шапки вызывает DragMove, двойное — Normal/Maximized. Кнопки меняют WindowState или Close. WM_GETMINMAXINFO выбирает ближайший монитор, задаёт MaxPosition как WorkArea−MonitorArea, MaxSize как WorkArea, учитывая панель задач. Координаты/размеры и состояние окна не сохраняются. Drag развёрнутого окна и смешанные DPI не проверены. [H: TitleBar_MouseLeftButtonDown, ToggleMaximizeRestore, WindowMessageHook, OnClosed; CFG]

Иконка Assets/QuickRemoteToolkit.ico используется как Image шапки и Window.Icon (BitmapFrame из каталога приложения; при отсутствии файла SetWindowIcon возвращает управление). Проект задаёт ApplicationIcon и копирование ICO в output/publish. Иных изображений в Assets эталона нет. ICO имеет 4799 байт, 4 PNG-кадра 16×16, 32×32, 48×48, 256×256, все 32 bpp; это физические размеры растров, а не DIP шапки. Содержимое ICO переносить из эталона без перерисовки. [M: Image; H: SetWindowIcon; PROJECT; ICON: заголовок ICO]

Основная Grid с Margin 16: колонки `*` и 320; строки Auto (панель сверху), `*` (клиенты), Auto (splitter), 190 (журнал). Верхняя панель в обоих столбцах, Margin снизу 12. У левых карточек справа Margin 12. [M: Grid Grid.Row=1]

Верхняя панель слева направо: знак 42×42, Radius 10, Margin справа 12, U+E977 размером 23 White; название 22 Bold (группа Margin справа 24); поиск в `*` с MaxWidth 660; «Открыть CSV» (Margin 12,0,8,0); primary «Обновить» (0,0,8,0); тема Width 42, Margin/Padding 0. Бейдж Ctrl+F справа поиска: Margin 0,0,10,0, Border 1, Radius 6, Padding 7,2. [M: верхний Grid]

Карточка клиентов: Padding 10, Margin 0,0,12,0. Checkbox сверху Margin 4,2,4,10; внутренняя рамка Radius 8, Border 1, Padding 1. Справа ScrollViewer в строках 1–3, вертикальная прокрутка Auto, карточки «Подключение», «Инструменты», «Операции», «CSV». Первые три независимо сворачиваются, изначально открыты, Margin снизу 12; CSV не сворачивается. Состояние секций не сохраняется. [M: ClientsGrid, ScrollViewer, SectionExpanderStyle; CFG]

Expander: ToggleButton, прозрачный Border Padding 0,2; справа U+E76C, открыто U+E70D (MDL2 11, Margin слева 10); hover меняет стрелку на AccentBrush. Контент Margin сверху 10 и Collapsed при закрытии. [M: SectionExpanderStyle]

Splitter: Height 10, Margin 0,6,12,6, Transparent, SizeNS, Rows/PreviousAndNext, ShowsPreview true, tooltip «Изменить размер журнала». Минимумы отдельных строк/сохранение пропорций не заданы. Таблицы используют штатную прокрутку WPF, боковая панель — свою; общего ScrollViewer окна нет. Ширина панели 320 фиксирована. [M: GridSplitter/DataGrid]

## 3. Единицы, шрифты и повторяющиеся параметры

XAML использует WPF DIP (1/96 дюйма), FontSize тоже DIP, не pt. Физические пиксели зависят от DPI: номинально 16 DIP при 150% → 24 px с округлением. Border 1 DIP не означает всегда 1 px. Manifest: true/pm, PerMonitorV2,PerMonitor, longPathAware true; явного повышения прав нет. [MANIFEST]

| Параметр | Значение | Источник |
| --- | --- | --- |
| Основной текст | Segoe UI 12, Normal | A: Window |
| Рендер | Display, ClearType, Fixed hinting; ClearTypeHint Enabled, UseLayoutRounding/SnapsToDevicePixels true | A: Window |
| Название / секции | 22 Bold / 14 SemiBold | M: шапка, заголовки |
| Получатель диалога | 16 SemiBold, Wrap | S/L: RecipientText |
| Иконки | Segoe MDL2 Assets 16, Margin справа 10 | A: IconTextStyle |
| Иконки секций / шевроны | 15, Margin справа 8 / 11, Margin слева 10 | A: SectionHeaderIconStyle/ChevronTextStyle/ToolButtonStyle |
| Сортировка / Ctrl+F | 9 / 11 | A: DataGridColumnHeader; M: бейдж |
| Карточка | Padding 14, Border 1, Radius 12 | A: CardStyle |
| Button | MinHeight 38, Padding 14,8, Margin 0,0,0,8, Border 1, Radius 8 | A: Button |
| ToolButton | MinHeight 34, Padding 12,7, Stretch, текст слева/шеврон справа | A: ToolButtonStyle |
| PrimaryButton | MinHeight 40, SemiBold, White | A: PrimaryButtonStyle |
| TextBox | MinHeight 40, Padding 12,7, Border 1, Radius 8, вертикальный центр | A: TextBox |
| Строка таблицы | 26, изменение высоты запрещено | A: DataGrid |
| Cell / GridText | Padding 8,0 / Margin 8,0,0,0, вертикальный центр | A: DataGridCell/GridTextStyle |
| Header таблицы | SemiBold, Padding 8,5, Border 0,0,1,1 | A: DataGridColumnHeader |
| Меню / tooltip | Border 1; ContextMenu Padding 4, MenuItem/ToolTip Padding 8,5 | A: соответствующие стили |

Копируемые Computer/IP/Person имеют CharacterEllipsis. У остальных GridTextStyle ellipsis/wrap не заданы. Путь CSV и пояснения диалогов Wrap. Высоты scrollbar, header таблицы, системных окон, focus adorner не заданы проектом и не заменяются выдуманными метриками. [M: CopyCellTemplate/CsvPathText; A: GridTextStyle; S/L]

## 4. Полные цветовые токены

A содержит светлые значения по умолчанию; H применяет T.Apply до InitializeComponent. DynamicResource обновляется при переключении. [T: LightPalette/DarkPalette/Apply; H: конструктор]

| Ресурс | Светлая | Тёмная | Назначение |
| --- | --- | --- | --- |
| WindowBackgroundBrush | #F5F7FB | #111827 | фон окна |
| CardBackgroundBrush | #FFFFFF | #1F2937 | фон карточек и полей |
| CardMutedBrush | #F8FAFD | #273449 | приглушённые поверхности, кнопки и заголовки |
| TextPrimaryBrush | #0F172A | #F1F5F9 | основной текст |
| TextSecondaryBrush | #475569 | #CBD5E1 | вторичный текст |
| TextMutedBrush | #7C8798 | #94A3B8 | приглушённый текст и шевроны |
| FavoriteBrush | #CA8A04 | #FACC15 | активное избранное |
| BorderBrushSoft | #D8E0EC | #475569 | мягкие границы |
| DividerBrush | #E5EAF3 | #344256 | разделители и линии таблиц |
| AccentBrush | #2563EB | #60A5FA | акцент, primary и фокус |
| AccentHoverBrush | #1D4ED8 | #3B82F6 | primary hover |
| AccentSoftBrush | #E8F1FF | #1E3A5F | мягкий акцент (текущий код напрямую не использует ресурс) |
| SelectedRowBrush | #DDEBFF | #1E4976 | выбранные строки/ячейки |
| AlternatingRowBrush | #FBFCFE | #243044 | чередующаяся строка |
| ButtonHoverBrush | #F1F6FF | #30415A | кнопка hover |
| ButtonHoverBorderBrush | #BFD0EA | #5B708D | граница кнопки hover |
| ButtonPressedBrush | #E8F1FF | #1E3A5F | кнопка pressed |
| PrimaryPressedBrush | #1E40AF | #1D4ED8 | primary pressed |
| HeaderHoverBrush | #F1F5FB | #30415A | заголовок таблицы hover |

Цвета вне ThemeService одинаковы в обеих темах:

| Значение | Назначение | Источник |
| --- | --- | --- |
| White (#FFFFFF) | primary текст/иконки, знак приложения, close hover/pressed | A: PrimaryButtonStyle/CloseTitleBarButtonStyle; M: верхняя панель |
| Transparent (#00FFFFFF WPF) | titlebar, expander, copy/favorite, splitter | A: TitleBarButtonStyle; M: локальные шаблоны |
| #C42B1C / #A4261D | close hover / pressed | A: CloseTitleBarButtonStyle |
| #16A34A / #DC2626 | Online / Offline | C: StatusBrush |
| #2563EB / #8A94A3 | Checking / Unknown | C: StatusBrush |

CheckBox, RadioButton, части menu/scrollbar/focus используют штатные WPF/Windows состояния. Их системные цвета не являются точными токенами проекта. Машиночитаемые токены и единицы: [design-tokens.json](design-tokens.json).

## 5. Компоненты и состояния

| Компонент | Обычное → hover → pressed/selected → disabled | Дополнительно | Источник |
| --- | --- | --- | --- |
| Button | CardMuted/BorderBrushSoft → ButtonHover/ButtonHoverBorder → ButtonPressed → opacity .55 | Cursor Hand; собственного focus-триггера нет | A: Button |
| ToolButton | Как Button | Слева ContentPresenter, справа U+E76C, 11 TextMuted | A: ToolButtonStyle |
| Primary | Accent/Accent → AccentHover/AccentHover → PrimaryPressed → opacity .55 | Текст White | A: PrimaryButtonStyle |
| Titlebar | Transparent → ButtonHover → ButtonPressed | Собственного disabled-триггера нет | A: TitleBarButtonStyle |
| Close | Transparent → #C42B1C → #A4261D | Hover/pressed текст White | A: CloseTitleBarButtonStyle |
| TextBox | CardBackground/BorderBrushSoft; hover не задан | KeyboardFocused рамка Accent, disabled .55; error-рамки нет | A: TextBox |
| Header | CardMuted → HeaderHover | SortDirection показывает ▲/▼ TextSecondary | A: DataGridColumnHeader |
| Client row | CardBackground/AlternatingRow → SelectedRow, текст Accent | Специального hover нет; выбранная cell тоже SelectedRow/Accent | M: RowStyle; A: DataGridCell |
| Star | U+E734 TextSecondary → U+E735 FavoriteBrush при избранном | Tooltip меняется; CanFavorite→IsEnabled; наследует Button | M: Favorite button |
| Copy | Hidden → Visible на hover ячейки | Пустая строка/«-»: disabled; наследует Button | M: CopyCellTemplate |
| Checkbox/radio/menu/tooltip/separator | Общие foreground/background/border, где заданы | Собственных шаблонов состояний нет, остаётся WPF | A/M/L |

Spinner, progress bar, toast, общая busy-подложка, красная рамка ошибки отсутствуют. Выполнение показывают текст статуса или журнал. Кнопки основной панели на время операций визуально не отключаются. MessageBox (OK/Information либо OK/Error), OpenFileDialog/SaveFileDialog используют системное оформление. [H: обработчики; SH/LH]

## 6. Таблица клиентов

ReadOnly, AutoGenerateColumns false, CanUserAddRows false, Single selection, AlternationCount 2, HeadersVisibility Column, горизонтальные линии DividerBrush. Начальная сортировка не задана — порядок CSV. [M: ClientsGrid; A: DataGrid]

| Заголовок | Ширина DIP | Значение / SortMemberPath | Оформление |
| --- | --- | --- | --- |
| ★ | 48 | IsFavorite | Кнопка 28×24, центр, Padding cell 0 |
| № | 58 | Number | GridTextStyle слева |
| Компьютер | 190 | Computer | CopyCellTemplate |
| IP | 150 | Ip | CopyCellTemplate |
| Сотрудник | * | Person | CopyCellTemplate, остаток ширины |
| Статус | 150 | StatusText | Margin слева 8, круг 9×9 с Margin справа 9 |
| Домен | 180 | Domain | GridTextStyle |
| Последняя проверка | 170 | LastCheckedText / LastChecked | dd.MM.yyyy HH:mm:ss, начально «—» |

Источник: M ClientsGrid.Columns; C LastCheckedText. Сортировка разрешена, индикатор ▲/▼. Статус сортируется по строке, время по DateTime?. Resize/reorder columns не запрещены отдельно, но Header template не содержит PART_LeftHeaderGripper/PART_RightHeaderGripper: ручное изменение ширин нельзя считать проверенным. [A: DataGrid/DataGridColumnHeader]

Статусы: Unknown «Неизвестно», Checking «Проверка», Online «Доступен», Offline «Не отвечает». PingClientAsync ставит LastChecked=Now в начале, Domain «Проверка…». Target — IP, если не пустой/«-», иначе Computer. Success → Online и «N ms» в журнале; иной ответ/исключение → Offline и Domain «Недоступно». При успехе DOM получает домен по Computer: имя домена, «Рабочая группа: X»/«Рабочая группа», «Не присоединён», «Неизвестно», «Ошибка запроса», «Нет ответа» (тайм-аут 5 секунд). Ошибка домена — отдельная запись Domain. [C: Target/StatusText/StatusBrush; H: PingClientAsync; DOM: GetAsync/Get]

SelectionChanged пуст. ПКМ выбирает строку и фокусирует её; вне строки не сбрасывает прежний выбор. DoubleClick вызывает Remote Assistance для выбранного клиента, кроме попадания внутрь Button; проверки принадлежности двойного клика строке нет. [H: ClientsGrid_*]

Контекстное меню в точном порядке: Remote Assistance; Ping; Tracert; разделитель; Открыть C$; Event Viewer; Computer Management; MSTSC; WinRS cmd; gpupdate /force; Локальные администраторы…; Отправить сообщение…; разделитель; Копировать имя ПК; Копировать IP; Копировать имя сотрудника. [M: ClientsGrid.ContextMenu]

Copy-кнопка U+E8C8: 24×22, Margin 4,0,4,0, Padding 0, Tag=значение. Обработчик Handled=true, игнорирует whitespace/«-»; успех: «Копирование» / «Значение скопировано в буфер обмена.», без самого значения. Ошибка — журнал и MessageBox «Не удалось скопировать». Контекстное копирование через RunForSelected не фильтрует пустые значения и пишет «Copy computer»/«Copy IP»/«Copy person» + «Запущено.». Ctrl+C таблицы отдельно не переопределён; результат template columns не проверен. [M: CopyCellTemplate/CopyCellButtonStyle; H: CopyCell_Click/Copy*_Click]

Избранное: ключ computer:+Trim().ToUpperInvariant(), при пустом/«-» имени резервный ip: с тем же преобразованием. Без обоих CanFavorite=false. Сохраняется сразу, набор ключей сортируется; дубликаты ключа обновляются вместе. Старые ключи сохраняются при смене CSV. Ошибка Save отменяет изменение и показывает «Не удалось сохранить избранное. Изменение отменено.\n{ошибка}», заголовок «Ошибка сохранения настроек», журнал «Избранное». Фильтр «Только избранные» сохраняется, совмещается с поиском через И. [C: FavoriteKey; H: Favorite_Click/FavoritesOnly_Click/SaveFavoriteSettings]

## 7. Поиск, CSV и предпочтения

Поиск TextChanged: Trim, OrdinalIgnoreCase Contains по Computer, Ip, Person **и Domain**, не по Number. Placeholder «Поиск компьютера, сотрудника или IP...» виден только при Text=""; tooltip «Поиск по имени компьютера, IP или сотруднику». Ctrl+F — Focus и SelectAll. Бейдж остаётся виден, отдельного правого Padding для введённого текста нет. Ноль результатов — таблица с заголовками без пустой иллюстрации/надписи. [M: SearchBox; H: FilterClient/FocusSearch]

«Открыть CSV»: Title «Выберите CSV клиентов», фильтр `CSV files (*.csv)|*.csv|All files (*.*)|*.*`. Отмена без изменений; выбор сохраняет путь и вызывает LoadClients. «Обновить» перечитывает файл. LoadClients сначала очищает клиентов, статусы/домен/время сбрасываются, избранное восстанавливается, поиск и фильтр остаются. [H: ChooseCsv_Click/ReloadClients_Click/LoadClients]

CSV: первая строка пропускается без валидации заголовков, разделитель `;`, UTF-8 strict → fallback 1251; App.OnStartup регистрирует CodePagesEncodingProvider. Пустые строки/<3 поля пропускаются; лишние поля игнорируются; кавычки/экранированные `;` не разбираются. Неверный Number → clients.Count+1; остальные поля Trim, Person необязателен. [CSV: Load/ReadLines; START: OnStartup]

CsvPathButton: Padding 12, MinHeight 0, текст Wrap, hover→AccentBrush. CsvInfoText: TextMuted, Margin сверху 12, «Загружено: dd.MM.yyyy HH:mm   Клиентов: N», N — полный список. Нет файла: «CSV не выбран или файл не найден.», CsvInfoText пуст, запись Load CSV. Ошибка чтения: ex.Message в журнале и MessageBox «Ошибка чтения CSV»; старый путь/информация могут остаться. [M: CSV-карточка; H: LoadClients]

Путь открывает Explorer `/select,"{csvPath}"`. Нет файла: Information «CSV-файл не выбран или больше не существует.» / «Quick Remote Toolkit». Ошибка: запись Open CSV location и MessageBox «Не удалось открыть расположение CSV». [H: CsvPath_Click]

Настройки: `%AppData%\QuickRemoteToolkitApp\settings.json`, defaults ClientsCsvPath="", PingTimeoutMs=1000, ParallelPingLimit=24, FavoriteComputers=[], ShowFavoritesOnly=false, IsDarkTheme=false. При отсутствии settings ищется Documents/GitHub/Quick-Remote-Toolkit/QuickRemoteToolkit.clients.csv. Save: settings-{GUID}.tmp → File.Move overwrite, finally удаляет остаток. Путь, тема, избранное и его фильтр сохраняются. Поиск, выбор, журнал, размеры и раскрытие секций — нет. UI для ping timeout/параллелизма отсутствует. [CFG]

Тема: светлая кнопка U+E708, tooltip «Включить тёмную тему»; тёмная U+E706, «Включить светлую тему». ToggleTheme меняет bool, применяет T, сохраняет CFG, обновляет кнопку. Save темы/пути и Load повреждённого settings не обёрнуты локальным catch; откат/специальный диалог не гарантируются. [H: ToggleTheme_Click/UpdateThemeButton/ChooseCsv_Click; CFG]

## 8. Действия

| Карточка | Кнопки сверху вниз |
| --- | --- |
| Подключение | Remote Assistance; Ping; Tracert; Открыть C$ |
| Инструменты | Event Viewer; Computer Management; MSTSC; WinRS cmd; Отправить сообщение…; Локальные администраторы…; gpupdate /force |
| Операции | Проверить всех; Экспорт логов |

Источник: M, обработчики H с соответствующими названиями. Иконки действий — MDL2, AccentBrush; тултипы поясняют действие и выбранную цель. ToolButton имеет правый шеврон, включая экспорт и очистку журнала.

ACT использует Computer для msra/eventvwr/compmgmt/mstsc/winrs/gpupdate; tracert — Target. C$ готовит IPC$ через net.exe от текущего DOMAIN\user (5 секунд), потом Explorer `\\Computer\c$`, даже если подготовка вернула текст ошибки. Требуются SMB/админправа, WinRS — WinRM. RunForSelected пишет «Запущено.» после создания процесса, не после удалённого успеха. Исключение — журнал и MessageBox с заголовком действия. Аргументы не менялись и вручную не проверялись на сети. [ACT: все методы; H: RunForSelected/AdminShare_Click]

Команды остаются enabled без выбора и при Offline/Unknown; при вызове без выбора Information «Выберите клиента.» / «Quick Remote Toolkit». PingAll идёт по **всей _clients**, не по фильтру, с SemaphoreSlim(ParallelPingLimit). Повторный ping/ping-all не запрещён, отмены нет. [H: RunForSelected/PingSelected_Click/PingAll_Click]

Сообщения и администрирование имеют отдельные глобальные flags на окно. Повтор: «Дождитесь завершения текущей отправки.» / «Отправить сообщение» либо «Дождитесь завершения текущей операции.» / «Локальные администраторы». Кнопки не disabled; взаимного запрета этих двух разных операций нет. [H: SendMessage_Click/LocalAdministrators_Click]

## 9. Журнал

Нижняя левая карточка: исходная строка 190, Padding 10; внутренняя рамка Radius 8/Border 1/Padding 1. Заголовок «Журнал», U+E8FD Accent слева; справа ToolButton «Очистить журнал», tooltip «Удалить текущие записи журнала из окна». LogGrid ReadOnly, AutoGenerateColumns/CanUserAddRows false. [M: LogGrid]

| Колонка | Ширина DIP | Значение / сортировка |
| --- | --- | --- |
| Время | 155 | TimeText dd.MM.yyyy HH:mm:ss / Time |
| Компьютер | 180 | Computer |
| Действие | 230 | Action |
| Результат | * | Result |

Источники: M LogGrid.Columns, LOG TimeText, A DataGrid. Single selection для журнала не задано — остаётся WPF default. AddLog Insert(0): новые сверху, пока пользовательская сортировка не меняет отображение. Пустое состояние — только заголовки. ClearLogs_Click делает _logs.Clear без подтверждения и записи об очистке; пустая/повторная очистка допустимы. Фоновые действия не отменяются, поздние записи появятся; старые экспортированные файлы не меняются. [H: AddLog/ClearLogs_Click]

Экспорт: SaveFileDialog, фильтр TXT/CSV/All files, DefaultExt txt, FilterIndex 1, имя `quick-remote-toolkit-log-yyyyMMdd-HHmmss.txt`. Формат выбирает расширение .csv без учёта регистра. TXT: `[yyyy-MM-dd HH:mm:ss] Computer | Action | Result`. CSV: `yyyy-MM-dd HH:mm:ss;Computer;Action;Result`, без заголовка/экранирования. File.WriteAllLines UTF-8, порядок _logs, не сортировка LogGrid. После записи добавляется Export logs с путём; этой записи в только что сохранённом файле нет. Пустой журнал → пустой файл, затем запись экспорта. Ошибка записи не перехватывается локально. [H: ExportLogs_Click]

## 10. Сообщение

Title «Отправить сообщение», 560×460, min 460×420, CenterOwner, ShowInTaskbar false, стандартная рамка и WPF resize (не переопределены). Модальное Owner=MainWindow. Grid Margin 20, строки Auto/Auto/*/Auto/Auto/Auto. [S; H: SendMessage_Click]

Точные элементы:

- RecipientText «Сообщение на {computer}», 16 SemiBold, Wrap.
- Пояснение Margin 0,8,0,12, TextSecondary, Wrap: «Сообщение получат пользователи выбранного ПК. Нажатие ОК, Отмена или истечение времени появится в журнале. Требуются права просмотра сеансов и отправки сообщений.»
- MessageText пустой, MaxLength 2000, AcceptsReturn, Wrap, вертикальный Top, scrollbar Auto, Automation.Name «Текст сообщения».
- Строка Margin сверху 12: «Время показа (секунды):» (Margin справа 12), DisplaySecondsText «300», Width 90, MaxLength 4, Automation.Name «Время показа в секундах».
- StatusText: Margin 0,8,0,12, TextSecondary/Wrap; «От 1 до 3600 секунд. Ctrl+Enter — отправить.»
- Справа «Отмена» Width 110, IsCancel, Margin справа 8; primary «Отправить» Width 130, Margin 0, начально disabled. [S; SH: конструктор]

Loaded фокус MessageText; TextChanged включает отправку только по непустому не-whitespace тексту, не по секундам. Ctrl+Enter вызывает Send_Click даже при disabled Button; обычный Enter многострочный, IsDefault у отправки нет. Escape/Отмена закрывают без выполнения. Пустой текст → «Введите текст сообщения.» и фокус MessageText. int.TryParse неуспешен или вне 1–3600 → «Укажите время показа от 1 до 3600 секунд.» и фокус секунд. Ошибка остаётся TextSecondary, редактирование не очищает её автоматически. Успех → Trim текста, seconds, DialogResult=true: окно **закрывается до отправки**, без busy UI. [SH: конструктор/MessageText_TextChanged/Send_Click]

Главное окно пишет «Сообщение» / «Отправка…». MSG получает активные (0) и отключённые (4) сеансы с непустым user и отправляет всем параллельно. У получателя системный Title «Сообщение от службы поддержки», OK/Cancel/Information; это внешнее, непроверенное окно. В журнале `{user} (сеанс {id}): отправка и ожидание ответа, до {seconds} сек.`, затем «Пользователь нажал ОК.», «Пользователь закрыл сообщение без подтверждения (Отмена).», «Время ожидания истекло.», «Получен ответ Windows: {response}.» либо Win32-ошибка с кодом. Нет сеансов → «На ПК не найдено пользовательских сеансов.». NUL в тексте отклоняется сервисом. Общая ошибка — MessageBox «Не удалось отправить сообщение», в журнале переводы строк заменены пробелом. Текст не сохраняется в журнале/настройках. OK не гарантирует прочтения. [MSG: SendAsync/GetSessions; H: SendMessage_Click]

## 11. Локальные администраторы

Title «Локальные администраторы», Width 560, SizeToContent Height, NoResize, CenterOwner, ShowInTaskbar false; стандартная рамка. Точная высота не задана. Модальное Owner=MainWindow, StackPanel Margin 20. [L; H: LocalAdministrators_Click]

- RecipientText «Локальные администраторы — {computer}», 16 SemiBold, Wrap.
- «Пользователь (ДОМЕН\логин или ИМЯ-ПК\логин)», Margin 0,16,0,8.
- MemberText пустой, MaxLength 256, Automation.Name «Учётная запись пользователя».
- «Действие», Margin 0,16,0,8.
- Radio «Добавить в локальные администраторы» выбран, Margin снизу 10; затем «Удалить из локальных администраторов».
- TextSecondary/Wrap, Margin 0,16,0,12: «Операция изменит права указанной учётной записи на выбранном ПК. Требуются WinRM и административные права текущего пользователя.»
- StatusText пустой, TextSecondary/Wrap, Margin снизу 12.
- Справа «Отмена» 110 IsCancel, Margin справа 8; primary «Выполнить» 130 IsDefault, Margin 0. [L; LH: конструктор]

Loaded фокус MemberText. Enter выполняет, Escape отменяет. Apply_Click: Trim, ровно одна `\`, непустые части, отсутствие char.IsControl; проверки существования аккаунта нет. Ошибка «Введите учётную запись в формате ДОМЕН\логин или ИМЯ-ПК\логин.», фокус MemberText; кнопка enabled даже с пустым вводом. Успех → Member/AddMember, DialogResult=true и закрытие. Нет предварительного списка членов/дополнительного подтверждения. [LH: Apply_Click]

В главном журнале «Добавление администратора»/«Удаление администратора» и `{Member}: выполнение…`, потом `{Member}: успешно.` или ошибка с заменой переводов строк. ADM: PowerShell Invoke-Command, SID S-1-5-32-544, Add/Remove-LocalGroupMember (меняется членство, не удаляется аккаунт). Тайм-аут 60 секунд: «Ожидание превысило 60 секунд. Результат операции неизвестен: проверьте состав группы на ПК перед повтором.». Ошибка также MessageBox с заголовком действия. Finally снимает флаг даже после тайм-аута: автоматического запрета до проверки группы нет. В диалоге состояния ожидания нет. [H: LocalAdministrators_Click; ADM: ChangeAsync/CreateScript]

## 12. Клавиатура и доступность

Явная комбинация главного окна — Ctrl+F. Tab/Shift+Tab и Space/Enter используют WPF; своих TabIndex/TabNavigation нет, мнемоники не заданы. Схема дерева: кнопки шапки → поиск/CSV/обновить/тема → фильтр/таблица → действия → splitter/журнал. Это не подтверждённый обход: точные остановки шаблонных кнопок, header, scrollbar требуют запуска. [M: InputBindings/дерево; A: templates]

Message: начальный фокус текст, затем секунды/Отмена/Отправить; Ctrl+Enter валидирует, Enter вводит строку. Administrators: аккаунт/radio/Отмена/Выполнить; Enter default, Escape cancel. Ошибки фокусируют поля. Навигация стрелками radio и вид focus adorner остаются системными и не проверены. [S/SH/L/LH]

AutomationProperties.Name: поля диалогов как выше; star «Избранное: {Computer}»; copy «Копировать {значение}»; toggle секции — Header.Children[1].Text. Остальные полагаются на штатные peers/Content/ToolTip. Нет live regions для журнала/статусов/ошибок или собственной high-contrast темы. Статус дублирует цвет текстом. Доступность icon-only кнопок и Hidden copy при Tab требует реальной проверки. [M: templates; S/L; A]

## 13. Различия и несогласованности

Сравнение коммитов main→codex/rust-rewrite: изменены только MainWindow.xaml/.xaml.cs, удалены 9 файлов specs/003-clear-logs/. В Rust-ветке StackPanel заголовка без «Очистить журнал» и нет ClearLogs_Click. Остальной отслеживаемый UI совпадает. Существующие незакоммиченные .gitignore/README, .agents/.specify assets, specs/002-rust-rewrite, src/QuickRemoteToolkit.Rust и src/QuickRemoteToolkit.Win32 сохранены; это не main. [git diff main codex/rust-rewrite]

Особенности текущей реализации, **не новые требования на исправление**:

1. Tooltip поиска не упоминает Domain, FilterClient ищет по нему.
2. Copy появляется только при hover; DoubleClick не проверяет строку.
3. Панель enabled без выбора/при выполнении, guards показывают диалог; повторные ping допускаются.
4. CSV-экспорт без escaping, ошибки экспорта/темы/пути и повреждённые settings без локального catch.
5. Диалоги закрыты до выполнения; после admin timeout флаг снят, проверка группы требуется текстом. specs/002-rust-rewrite может требовать более строгого поведения — оно не доказано WPF.
6. Цвета статусов и White primary не адаптируются как пары тем. Системные части диалогов/checkbox/radio/menu/scrollbar и контраст требуют визуальной проверки.
7. Header template без grippers, хотя resize columns не запрещён свойством. Интерактивные ширины не проверены.

Источники каждого пункта — указанные в разделах обработчики и стили; исходники не исправляются этой задачей.

## 14. Матрица экранов и состояний

Каждая строка обязательна в **обеих темах**. Сейчас все визуально **не проверены**; правила подтверждены кодом. Практические шаги: [ui-parity-checklist.md](ui-parity-checklist.md).

| ID | Экран/состояния | Источники |
| --- | --- | --- |
| P01 | Главное: 1500×920, min 1200×760, resize, splitter, прокрутка | M/A |
| P02 | Шапка: drag/minimize/maximize/restore, close hover/pressed, мониторы | M/H WindowMessageHook |
| P03 | CSV: нет/пуст/UTF-8/1251/ошибка/длинный путь/отмена/reload | H/CSV |
| P04 | Поиск: пустой/пробелы/4 поля/ноль/Ctrl+F | H FilterClient |
| P05 | Таблица: выбор/полосы/сортировки/длинный текст/ПКМ/copy/doubleclick | M/H |
| P06 | Избранное: star/duplicate/fallback/filter/save failure/restart | C/H |
| P07 | Unknown/Checking/Online/Offline, время, все Domain результаты | C/H/DOM |
| P08 | Действия: без выбора/Offline/полный набор/repeat/all collection | H/ACT |
| P09 | Правая панель: секции/CSV hover/scroll | M |
| P10 | Журнал: порядок/сортировка/clear/поздняя запись/TXT/CSV/пусто | H/M |
| P11 | Message: пустой/текст/2000/1–3600/ошибки/Tab/Ctrl+Enter/Escape | S/SH |
| P12 | После Message: ожидание/repeat/все WTS ответы/ошибки | H/MSG |
| P13 | Admin: пустой/валидный/ошибка/256/radio/Enter/Escape | L/LH |
| P14 | После Admin: выполнение/repeat/успех/ошибка/60 секунд | H/ADM |
| P15 | Темы: toggle/restart/все состояния Button/TextBox/menu/tooltip | A/T/CFG |
| P16 | 100/125/150/200%, два монитора, Tab, экранный диктор | MANIFEST/A/M/S/L |

Удалённые сценарии P07/P08/P12/P14 — только на разрешённых тестовых целях. Без них сохранять статус «не проверено», отдельно проверять безопасный ввод/отмену. Будущие отличия Rust принимаются отдельным решением.

## 15. Точные подсказки и знаки кнопок

Источник всей таблицы — M Button, TextBlock.Text, ToolTip и Click; имена обработчиков — H. Иконка темы и maximize меняются по §2/§7. Локальные copy/star описаны в §6.

| Подпись | Знак MDL2 | ToolTip | Click |
| --- | --- | --- | --- |
| Кнопка шапки | U+E921 | Свернуть | MinimizeWindow_Click |
| Кнопка шапки | U+E922 | Развернуть | MaximizeRestoreWindow_Click |
| Кнопка шапки | U+E8BB | Закрыть | CloseWindow_Click |
| Открыть CSV | U+E8E5 | Выбрать CSV-файл со списком клиентов | ChooseCsv_Click |
| Обновить | U+E72C | Перечитать список клиентов из выбранного CSV | ReloadClients_Click |
| Без текста (тема) | U+E708 | Включить тёмную тему | ToggleTheme_Click |
| Remote Assistance | U+E8AF | Открыть Windows Remote Assistance для выбранного клиента | RemoteAssistance_Click |
| Ping | U+E701 | Проверить доступность выбранного клиента | PingSelected_Click |
| Tracert | U+E774 | Открыть трассировку маршрута до выбранного клиента | Tracert_Click |
| Открыть C$ | U+E8B7 | Открыть административную шару C$ выбранного клиента | AdminShare_Click |
| Event Viewer | U+E8A5 | Открыть просмотр событий выбранного удаленного ПК | EventViewer_Click |
| Computer Management | U+E977 | Открыть управление компьютером для выбранного клиента | ComputerManagement_Click |
| MSTSC | U+E7F4 | Запустить подключение через Remote Desktop | Mstsc_Click |
| WinRS cmd | U+E756 | Открыть удаленную командную строку через WinRS | WinRsCmd_Click |
| Отправить сообщение… | U+E8F2 | Отправить сообщение пользователям выбранного ПК | SendMessage_Click |
| Локальные администраторы… | U+EA18 | Добавить или удалить пользователя из локальных администраторов выбранного ПК | LocalAdministrators_Click |
| gpupdate /force | U+E713 | Запустить обновление групповых политик на выбранном клиенте | Gpupdate_Click |
| Проверить всех | U+E716 | Параллельно проверить ping для всех клиентов из списка | PingAll_Click |
| Экспорт логов | U+E74E | Сохранить журнал действий в TXT или CSV | ExportLogs_Click |
| Динамический путь CSV | Нет | Открыть расположение CSV-файла | CsvPath_Click |
| Очистить журнал |  | Удалить текущие записи журнала из окна | ClearLogs_Click |


## Дополнение: измерения и реальные кадры 2026-11-16

При продолжении Rust-переноса получены настоящие WPF client visual и Rust OpenGL
framebuffer при 120 DPI/125%, 1500×920 и 1200×760 DIP. Полный метод, ограничения
и локальные evidence находятся в [отчёте](../specs/005-rust-parity/ui-validation.md).
Это дополняет исходную запись о неудачном desktop capture; эталон SHA не менялся.

Изолированный WPF runtime подтвердил header26/row26, MinColumnWidth20,
ClipboardCopyMode.ExcludeHeader, Extended журнала и возможность перестановки колонок.
Default cell template не применяет Padding к TextBlock: фактический текстовый отступ
GridTextStyle равен8, не16. Нативная рамка диалогов уменьшает client размер относительно
outer560×460 (при125% пример:545.6×422.4 DIP). Элементы TextBox могут фактически иметь
высоту45.96 DIP из-за шаблона, хотя MinHeight40.

Также зафиксирован дефект эталонных SendMessageWindow/LocalAdministratorsWindow:
implicit Style Window не применяется (Style=null), фон остаётся белым в тёмной теме.
В Rust читаемая тёмная тема диалогов зарегистрирована как намеренное отличие.
Палитры/размеры источника в этом документе не меняются для подгонки под Rust.
