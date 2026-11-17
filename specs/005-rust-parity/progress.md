# Прогресс — 2026-11-17

Ветка codex/rust-rewrite. Эталон main 91689516c96d52877bba3480bf498b7d807ead64;
fetch подтвердил отсутствие дрейфа. Полный паритет НЕ объявлен.

Spec Kit 1.3.1: constitution→specify→plan/research→tasks→analyze/checklist→implement→converge.
Feature specs/005-rust-parity через SPECIFY_FEATURE_DIRECTORY/.specify/feature.json.
Конституция 2.0.0, AGENTS/README/workflow обновлены по разрешению пользователя.
WPF/publish и исторические specs/существующие изменения сохранены. Commit/push/tag/merge не выполнялись.

Rust + egui/eframe 0.33.3, AccessKit, OpenGL/WGL, native viewports и owner/modal disable.
Модули models/services/native/ui/design/columns/selection/icon; unsafe только native.
Windows SDK rc.exe в build.rs встраивает EXE icon и PerMonitorV2/asInvoker manifest.
Portable: publish-rust/QuickRemoteToolkitRust.exe и QuickRemoteToolkit.Rust-win-x64.zip.
Финальные хеши/размеры/PE — artifact-validation.json, запуск архива — portable-smoke.json.

Реализованы все строки матрицы: CSV/JSON/избранное/поиск, сортировка/перестановка колонок,
выбор/copy/контекст/двойной клик/Ctrl+F, ICMP/RTT/домен/лимиты/generation, все Windows
действия/SMB, WTS/сеансы/ответы, admin SID544/timeout; журнал/clear/экспорт/темы/диалоги.
Настройки atomically/first backup/unknown fields/BOM; signed int values WPF сохраняются.
Unicode casing согласован с .NET (включая ß/ı и кириллицу), PC-ß не сливается с PC-SS; нет todo!/unimplemented! и зависимости от WPF.

Проверки Windows: fmt PASS; clippy all-targets -D warnings PASS; QA host clippy PASS;
25 tests PASS. WPF→Rust→WPF JSON round-trip повторно PASS на финальных models.
Release/static CRT publish PASS; PE x64 GUI без CLR/VC runtime, icon/group/manifest PASS.
Повторная упаковка тех же входов дала одинаковый ZIP SHA-256 (не clean rebuild proof).

UI: настоящий framebuffer Rust и WPF client visual получены, исходный capture blocker
обойдён test host. 26/26 состояний в двух темах + 8/8 минимальных + 4/4 после последней
правки таблиц; 12 WPF окон + 2 минимальных. Подробности/намеренные отличия — ui-validation.md.
Фикстуры busy/reorder/etc не означают реального Windows input или сетевой интеграции.
Системные Sky capture/input остаются недоступны; финальный ZIP зарегистрировал окно
с PATH только System32/без dev env, UIA подтвердила PC-DEMO/кнопки/Load CSV; SetValue/input — GetCursorPos access denied. Чистой VM нет.

Следующие проверки: полная физическая клавиатура/мышь/100-200%/мониторы/экранный диктор;
визуальное согласование различий raster/wrapping и исправленной темы WPF-диалогов;
явно разрешённые тестовые ПК для SMB/WinRM/WTS и ручных эквивалентов; чистая Windows
и CI для текущих исходников (не пушились). T009/T010/T012–T014 остаются открыты.
Converge: три partial findings, tasks_appended, не converged. Не закрывать по наличию кода.


## Исправление разделителя журнала

Пользователь воспроизвёл отсутствие изменения высоты при drag/release. Причина:
Response.drag_delta() относится к одному кадру, а на отпускании равен0; прежний код
снова присваивал исходную высоту. Сохраняются координата нажатия и последняя preview
высота, применяется полное перемещение. Регрессионный UI-тест сначала упал (190 вместо
310), затем прошёл: несколько кадров движения, отпускание, обратный drag, обе границы,
светлая/тёмная темы. Это app-level Pointer events, физический Windows input не заявлен.
Cargo fmt/clippy PASS, 26 tests PASS. Публикация/хеши — splitter-fix-validation.json.


## Исправление поиска — 2026-11-17

По скриншоту пользователя исправлены вертикальное центрирование текста TextEdit
и расположение Ctrl+F. Подсказка центрируется по реальному galley, размер рамки
измеряется по тексту с внутренними отступами, справа зарезервирована область:
длинный запрос не рисуется под Ctrl+F. Рендеринг проверен настоящими кадрами обеих
тем при 1500×920 и1200×760 (8 запусков main/search) в evidence/rust-search-*20261117/.
Новый тест проверяет пустое поле, введённый и длинный текст, containment/clip и центр
при renderer scales100/125/150/200%; это не физическое переключение Windows DPI.
Всего27 tests PASS; fmt/clippy PASS. Поставка/хеши — search-fix-validation.json.


## Новая иконка — 2026-11-17

По прямому запросу пользователя заменена старая тонкая маленькая иконка: крупный
белый монитор со стрелкой на синем фоне. SVG — assets/QuickRemoteToolkit.svg;
ICO содержит отдельные16/20/24/32/40/48/64/128/256 PNG frames. Windows EXE resource,
native window/taskbar и custom title bar используют общий assets/QuickRemoteToolkit.ico.
Функция icon::application_icon переименована из reference_icon: новый рисунок —
намеренное пользовательское отличие от старого WPF SHA, его иконка сохранена.
Опциональный generator.ps1 (generate-icon.ps1) — только design-time конверсия;
Cargo и поставка не требуют WPF/.NET. Просмотрены native16 PNG/256 PNG и реальные
кадры двух тем (evidence/rust-icon-fix-20261117). ICO directory и PNG размеры проверены.
27 tests и clippy PASS. Хеши/поставка — icon-fix-validation.json.


## Единая иконка монитора с шестерёнкой — 2026-11-17

По уточнению пользователя заменён рисунок на монитор с шестерёнкой внутри экрана.
Маленькая иконка заголовка и большая плитка toolbar используют одну texture;
обе, а также native window icon, получают цвет Accent текущей темы (#2563EB/#60A5FA).
ICO EXE содержит тот же рисунок с базовым синим цветом. Новые кадры обеих тем:
evidence/rust-monitor-gear-20261117. 27 tests/clippy PASS. Поставка: gear-icon-validation.json.
