> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Проверка настоящего UI — 2026-11-16

Эталон: `91689516c96d52877bba3480bf498b7d807ead64`. Windows, 120 DPI (125%).
Размеры главного окна: 1500×920 и 1200×760 DIP. Полный P01–P16 пока не закрыт.

## Метод и доказательства

`tests/ui-smoke.ps1` собирает отдельный `qrt-ui-qa` с feature `ui-qa`. Он использует
те же ui/design/models/services/native, системные Segoe UI/MDL2 и настоящий eframe
OpenGL renderer. Последний paint callback читает реальный framebuffer после восьми
кадров. Это не нарисованные макеты и не headless shapes. Test host не поставляется.

Состояния search/empty/busy/collapsed/reordered/logs/long/context вводятся тестовой
фикстурой; они не доказывают успешный сетевой вызов или физический ввод Windows.
В message-error и admin-error Enter/Ctrl+Enter поступают в egui InputState до
обработчиков настоящего диалога. Проверяются ошибка и отсутствие запуска операции;
это app-level input, не проверка доставки клавиатуры через Windows.

У всех запусков отдельный APPDATA, CSV содержит только PC-DEMO/192.0.2.x.
Ни сообщение пользователю, ни изменение членства на ПК тестами не выполнялись.

Получены результаты:

- `evidence/rust-verified-20261116/results.json`: 26/26 запусков, 13 состояний × 2 темы.
- `evidence/rust-minimum-20261116/results.json`: 8/8 запусков при 1200×760 DIP.
- После последней правки полос прокрутки/выравнивания статуса: ещё 4/4 запуска,
  `evidence/rust-layout-final-20261116/results.json` (main/long, обе темы, 1200×760).
- `evidence/wpf-final-20261116/`: 12 эталонных окон, оба диалога/ошибки/главное/меню.
- `evidence/wpf-minimum-20261116/`: 2 главных окна при 1200×760 DIP.

Ранние `rust-final-*` и `rust-context-check*` — промежуточные/диагностические записи,
не финальное доказательство. Генерируемые изображения/профили игнорируются Git.

WPF reference host извлекает исходники через git archive указанного SHA. Изменяет
только путь SettingsService на обязательный QRT_QA_APPDATA и убирает StartupUri
для исключения второго окна. XAML/обработчики трёх окон не переписываются. Ошибки
WPF для визуального сравнения задаются как fixture, без удалённой операции.
RenderTargetBitmap снимает настоящий WPF client visual, без native DWM рамки.
Отдельно сохраняется настоящий ContextMenu visual. Метаданные записывают DIP/DPI.

## Проверенные исправления

По сопоставлению кадров исправлены расположение поиска, размеры элементов строки,
26-DIP заголовки и строки, 8-DIP отступы текста, чередование/линии/границы таблиц,
центрирование звезды и статуса, обычные и минимальные размеры, полосы прокрутки,
отступы секций/журнала, ширина и плоские элементы контекстного меню, фон/поля/
естественная высота диалогов, поправка outer/client размеров native viewport.

WPF runtime дополнительно подтвердил MinColumnWidth=20, ClipboardCopyMode=ExcludeHeader,
Single selection клиентов, Extended журнала, CanUserReorderColumns=True.
Rust сохраняет логическую ширину при перестановке колонок, поддерживает Ctrl/Shift,
Ctrl+A/C, прокручивает выбранную стрелками строку без анимационной задержки.
Отдельные headless UI-тесты проверяют реальный Copy output и видимость PC-0999.
Разделитель журнала показывает preview, применяет высоту после отпускания.

Начальный фокус обоих полей наблюдался как focused=true/keyboard=true в раннем
native запуске. В части фоновых пакетных запусков Windows оставила focused=false;
поэтому полная проверка активации/Tab/Shift+Tab/закрытия остаётся открытой.

## P01–P16

| Сценарии | Доказано | Осталось |
|---|---|---|
| P01–P05 | Оба размера/темы, длинные строки, поиск/выбор/reorder/menu rendering; модели и command adapters | Физические клики/doubleclick, drag колонок, file dialogs, Explorer и clipboard Windows |
| P06–P08 | Busy rendering; ICMP loopback, RTT/лимит/generation/domain через tests | Разрешённые ПК, реальные домен/SMB/команды/отказы |
| P09–P10 | Журнал/clear button/extended selection rendering; Copy output, TXT/CSV tests | Windows file dialog/очистка и экспорт через физический ввод |
| P11–P14 | Оба native диалога, ошибки Enter/Ctrl+Enter, auto height, UTF16 limits | Все Tab/Space/Escape/Enter/radio transitions; WTS/WinRM/timeout на разрешённых целях |
| P15 | Обе палитры во всех 13 состояниях; JSON theme round-trip | Переключение/перезапуск через физический ввод |
| P16 | Настоящие 125% кадры, два размера, AccessKit поля/кнопки в коде | 100/150/200%, перенос мониторов/WorkArea, экранный диктор, полная клавиатурная приёмка |

## Зафиксированные отличия

Фактические WPF SendMessageWindow/LocalAdministratorsWindow имеют Style=null и белый
фон даже после тёмной ThemeService.Apply; вторичный текст становится малоконтрастным.
Rust использует CardBackground/TextPrimary для читаемых диалогов обеих тем.
Палитра эталонных tokens от этого не менялась. Стандартные checkbox/radio/scrollbar
и пустая icon-column меню WPF имеют системное оформление; Rust использует общую
палитру и пустой отступ без артефактов системного шаблона.

Растеризация и переносы текста egui отличаются от WPF: например, пояснение сообщения
при ширине 560 outer занимает 2 строки вместо 3. Это зарегистрированное визуальное
отличие, не доказанный пиксельный паритет. Native DWM рамки не сравнивались.

Системный захват Sky/WGC по-прежнему давал timeout/нет geometry, активация финального
portable окна — GetCursorPos access denied (0x80070005). Финальный EXE из ZIP с
ограниченным PATH зарегистрировал своё окно; UIA дерево этого запуска не получено.
Эти ограничения не подменяются app-level кадрами/тестами.

## Повторение

Из корня репозитория, каждый раз в новый каталог:

```powershell
./src/QuickRemoteToolkit.Rust/tests/ui-smoke.ps1 -OutputDirectory ./specs/005-rust-parity/evidence/rust-new
./src/QuickRemoteToolkit.Rust/tests/ui-smoke.ps1 -OutputDirectory ./specs/005-rust-parity/evidence/rust-small-new -Scenes main,long,message,admin -Width 1200 -Height 760
./src/QuickRemoteToolkit.Rust/tests/reference-smoke.ps1 -OutputDirectory ./specs/005-rust-parity/evidence/wpf-new
```

Reference host требует SDK .NET 10 только для исследования WPF. Поставляемое Rust
приложение не использует этот host или .NET Desktop Runtime.


## Регрессия: перетаскивание разделителя

Предыдущие rendering кадры не проверяли завершение жеста. После сообщения пользователя
добавлен `journal_splitter_keeps_preview_until_release_then_commits_total_drag`.
На прежнем коде после суммарного движения120DIP высота оставалась190 вместо310.
Исправлен расчёт относительно press origin и сохранена preview высота при отпускании.
Тест проверяет обе темы, обратное движение и крайние положения; всего26 tests PASS.
Проверка выполнена через настоящий egui handler с PointerMoved/PointerButton,
не через физическую мышь Sky. Исправленная поставка — splitter-fix-validation.json.


## Регрессия поиска — 2026-11-17

TextEdit ранее выравнивал текст сверху с фиксированным margin11, а надпись Ctrl+F
была привязана к правому краю поля, не к центру своей рамки (оставалось2DIP справа).
Теперь vertical_align=Center, симметричные вертикальные отступы, measured badge и
центрирование galley внутри неё; TextEdit clip ограничен до области shortcut.
Тест search_text_is_centered_and_shortcut_stays_inside_its_badge проверяет обе темы,
пустую/короткую/длинную строку и4 renderer scales. 8 native renderer запусков main/search
в двух размерах сохранены в evidence/rust-search-fix-20261117 и rust-search-small-20261117.
Кадры просмотрены: placeholder/значение центрированы, Ctrl+F внутри рамки. Всего27 tests.


## Пользовательская замена иконки — 2026-11-17

Вместо уменьшенного тонкого рисунка двух компьютеров используется один крупный
монитор со стрелкой. Редактируемый SVG и многокадровый ICO в Rust/assets; WPF-эталон
не меняется. Это прямо согласованное запросом пользователя визуальное отличие,
не попытка выдать новую иконку за исходный reference SHA. Иконка просмотрена при
нативных16px и в реальном title bar на кадрах обеих тем. Windows ресурс EXE и native
window icon получают тот же ICO. Экранный кэш Windows taskbar отдельно не очищается.
