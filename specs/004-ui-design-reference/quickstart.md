> Исторический документ. Технологический контекст и пути относятся к этапу до Rust-only 0.4.0. Миграция принята пользователем 2026-11-17; неподтверждённые проверки ниже не объявляются успешными. Текущие инструкции: [README](https://github.com/bdpst/Quick-Remote-Toolkit-App#readme), финализация: specs/006-rust-release. Удалённый код доступен в истории Git (WPF: 91689516c96d52877bba3480bf498b7d807ead64).

# Проверка результата
1. Прочитать docs/design-guidelines.md и убедиться, что указан SHA и ограничения визуальной проверки.
2. Разобрать docs/design-tokens.json через ConvertFrom-Json или python json; сравнить 19 ключей обеих палитр ThemeService фиксированного SHA.
3. Проверить все ссылки на файлы и все source-поля против git ls-tree эталона; сверить три окна и локальные цвета.
4. Пройти docs/ui-parity-checklist.md на обезличенных данных при будущем переносе; это ещё не тест Rust.
5. Сравнить git diff и status с начальным состоянием: никаких новых изменений src/, README, .gitignore, старых specs.
6. Сборка эталона: x64 C:/Program Files/dotnet/dotnet.exe build <каталог git archive>/QuickRemoteToolkit.App.sln -c Release. No-restore разрешён после восстановления. Готовый publish не использовать как доказательство SHA.
