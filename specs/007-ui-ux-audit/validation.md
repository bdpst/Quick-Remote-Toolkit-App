# Validation and run record

Дата: 2026-11-19. Windows 10 build 19045 x64, Rust 1.99.1 (58cf8f9c3), x86_64-pc-windows-msvc.
База после `git fetch origin main`: 7bdbe3b30bb3940a8aaf9394af6b399d31a0fd0f; HEAD совпадал с origin/main.
Первое действие — `git status --short --branch`: чистое дерево main. Пользовательских
незакоммиченных изменений не было. Создана codex/007-ui-ux-audit от origin/main.

## Spec Kit

Прочитаны AGENTS, constitution 3.0.0, README, существующий гайд/JSON, материалы specs
001/002/004/005/006; исторические материалы не применялись как текущие инструкции.
Последовательно: specify (resolver spec-template) → setup-plan -Json → setup-tasks -Json →
check-prerequisites -Json -RequireTasks -IncludeTasks → implement документов.
Resolver успешно вернул составные шаблоны; sequential номер 007. Feature state хранится
локально в ignored .specify/feature.json. Фактическая Git-ветка не зависит от resolver fallback BRANCH.
.specify/extensions.yml отсутствует: before/after specify, plan, tasks, implement hooks не зарегистрированы.
Ignore rules проверены: target/, publish-rust/, *.tmp, *.log; новых ignore-файлов не требуется.

Implement gate (read-only):

| Checklist | Total | Checked | Unchecked | Status |
| --- | --- | --- | --- | --- |
| requirements.md | 16 | 16 | 0 | PASS |

После этого checklist не редактировался; он доказывает качество требований, не прохождение UI.

## Выполненные команды и evidence

- `ui-smoke.ps1 -OutputDirectory ./publish-rust/audit-007-normal`: 26/26 PASS.
- `ui-smoke.ps1 -OutputDirectory ./publish-rust/audit-007-minimum -Width 1200 -Height 760`: 26/26 PASS.
- `cargo build --locked --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml --bin QuickRemoteToolkitRust`: PASS.
- `cargo test --locked --manifest-path src/QuickRemoteToolkit.Rust/Cargo.toml`: 27/27 PASS.
  Включая selected target без исполнения команд, CSV encoding, export escaping,
  sorting/navigation/hidden selection обеих тем, copy display order, splitter preview/commit,
  локальный powershell timeout без remote effect, loopback ICMP.
- `git diff --check`, проверка локальных Markdown-ссылок и scope diff: PASS (финальная проверка).

Новые тесты не добавлялись. fmt/clippy/release/package не запускались: приложение/зависимости
не изменены, задача документальная. publish-rust не заменён новой поставкой; в нём только
ignored audit evidence и тестовые профили. Никакие рабочие удалённые ПК не использовались.

[normal results](evidence/normal/results.json) · [minimum results](evidence/minimum/results.json).
В evidence сохранены PNG, viewport metadata и state.json каждой сцены, без settings/CSV
с личными данными. CSV smoke содержит только PC-DEMO/PC-DEMO-02 и 192.0.2.1/2;
long scene использует синтетические длинные строки и IPv6 2001:db8::/32.

## Что именно проверено

Статус PARTIAL не равен успешному ручному прохождению.

| Сценарий | Метод и фактический результат | Статус / предел |
| --- | --- | --- |
| Запуск и загрузка CSV | QA Toolkit::new читает реальный синтетический файл, 2 клиента; обычный debug instance также показал 2 записи в accessibility tree | PASS загрузка; файловый picker не пройден |
| Импорт UTF-8/1251 и повреждённые строки | Существующие unit tests parse/load | PASS unit; GUI импорт не выполнен |
| Обновление CSV | reload исследован в ui.rs: сброс выбора/поколения/повторное чтение | PARTIAL source; кнопку и изменение файла вручную не проверили |
| Поиск Ctrl+F | QA search отображает seed PC-DEMO-02; тест search_text проверяет textbox/badge, модели — matches | PARTIAL; физический Ctrl+F не подтверждён |
| Выбор/скрытый выбор/сортировка | QA selected scenes; thousand_row test обеих тем, selected-target contract | PASS unit/render; физический выбор не выполнен |
| Избранное и фильтр | QA звезда, settings tests, source checkbox/save/rollback | PARTIAL; click/restart GUI не пройден |
| Ширина и перенос колонок | QA reordered, columns test, исходные ширины на обоих размерах | PARTIAL; physical drag не выполнен |
| Копирование | Existing copy visible order/log ranges test; source tooltip/hover button/menu | PASS unit; системный clipboard вручную не проверен |
| Контекстное меню | QA context обеих тем/размеров | PASS render seed; открытие правой кнопкой не проверено |
| Панель действий | QA main/collapsed/busy и source current/guard/thread | PASS render; прокрутка/команды вручную не пройдены |
| Высота журнала | Existing UI test drag preview/commit/extremes; QA logs | PASS unit; physical splitter не пройден |
| Экспорт | export_contract_and_escaping unit; source TXT/CSV picker/write/error | PARTIAL; системный Save As не пройден |
| Сообщение/администраторы | Оба viewport в обеих темах и при обоих main sizes; QA capture реального framebuffer | PASS render |
| Невалидный ввод | QA Ctrl+Enter при seconds=0, Enter с пустой учётной записью; точный dialog_error, messaging/administering false | PASS инжекция клавиш в egui; не физическая клавиатура |
| Фокус | QA metadata/снимок: текстовое поле и ошибочное поле активны; tree основных контролов прочитан | PARTIAL; полный Tab и screen reader не пройдены |
| Пустой список | QA empty очищает clients, без сетевых вызовов | PASS render; не доказательство пустого CSV picker flow |
| Нет результатов | source общего filtered indices empty; models matching tests | PARTIAL; физический поиск без результата не подтверждён |
| Длинные значения | QA long обеих тем/размеров | PASS render; long вместе с диалогами не проверено |
| Ожидание | QA busy задаёт probing=true, «Проверка», spinner; source guards | PASS render seed; latency/real network не измерены |
| Ошибки/тайм-аут | QA validation; unit powershell timeout и responses; services/native launch wording | PARTIAL; native error notice и timeout в реальном журнале не сняты |
| Удалённые команды | mock contract выбранной цели, synthetic states | Только безопасные состояния; SMB/WinRM/WTS/member изменения НЕ ВЫПОЛНЯЛИСЬ |

## Размер, DPI и визуальный просмотр

Все 52 сцены получены настоящим Windows OpenGL framebuffer, не реконструированы из кода.
Main viewport metadata: 1500×920 и 1200×760; `dpi=1` pixels_per_point во всех metadata
этого прогона. Это фактически один масштаб рендера 100% (96 DPI equivalent),
не исчерпывающая проверка настроек Windows/всех мониторов.
125/150/175/200%, mixed DPI, перенос окна, физическое 1366×768 и screen reader — NOT RUN.

Непосредственно просмотрены: normal light/dark main, оба диалога и validation errors
в обеих темах; light-long normal; minimum light-main/empty и dark-long/busy/context.
Остальные PNG автоматически получены/проверены smoke и доступны для просмотра;
не утверждается ручная визуальная оценка каждого из 52 кадров.

## Ограничение интерактивного прохода

Отдельный debug EXE запущен с APPDATA=publish-rust/audit-007-normal/profile;
существовавший пользовательский publish-rust EXE не использовался/не закрывался.
Computer Use смог прочитать accessibility tree тестового окна (2 синтетических клиента,
доступные названия большинства кнопок). Две попытки захвата вернули:
`window capture timed out: timed out waiting on channel`.
Через cua_repl дерево также прочитано; попытка клика:
`coordinate input geometry is unavailable`. Ctrl+F/type не дали подтверждённого изменения
дерева и не засчитаны. Дальнейший слепой ввод не выполнялся.
Поэтому ручные picker/export/drag/context/keyboard workflows остаются открытыми,
хотя среда Windows и автоматический QA-host доступны.

## Отделение факта от предположения

audit.md содержит 8 находок, все с ID/приоритетом/экраном/источником/шагами/фактом/
влиянием/предложением/приёмкой; 5 гипотез вынесены отдельно. Статусы remote outcome:
запуск, проверенный ответ и неизвестный timeout различаются в исходниках/tests;
проверка реального удалённого результата в этот аудит не входит.
Документы завершены, будущие исправления и NOT RUN сценарии не закрыты этой задачей.

Финальная проверка: 68 PNG (34 на размер), 68 metadata с dpi=1; ссылки Markdown разрешаются; 7 задач документов завершены. Отдельная независимая Assessment A подтвердила основные визуальные находки и добавила расчёт контраста выделенной строки 3.64:1. Тестовые процессы завершены; существовавшее пользовательское окно оставлено без изменений. Изменения tracked ограничены docs/design-guidelines.md; новые файлы только specs/007-ui-ux-audit/.
