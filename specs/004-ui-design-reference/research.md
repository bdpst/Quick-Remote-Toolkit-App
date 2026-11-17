# Исследование
## Решение: установленный Spec Kit
CLI specify version = 1.3.1; .specify/integration.json и init-options.json указывают Codex skills, PowerShell и разделитель `-`. Проверены 9 .agents/skills/speckit-*/SKILL.md, шаблоны и PowerShell scripts; SHA-256 всех 9 skills совпали с codex.manifest.json. Для работы прочитаны specify, plan, tasks, implement, analyze. Прямой slash-вызов не предоставлен: инструкции исполняются агентом и скриптами. Наличие папки не было единственным доказательством установки. extensions.yml отсутствует, hooks пропущены на всех этапах. Обновление с GitHub не нужно; конституция сохранена.
## Решение: явный контекст feature
SPECIFY_FEATURE_DIRECTORY=specs/004-ui-design-reference; SPECIFY_FEATURE_NO_PERSIST=1. Поддержаны common.ps1. Feature directory не равен Git-ветке. Выбор временный и не меняет пользовательский feature.json. Core specify допускает независимый каталог; для этого сеанса persist намеренно отключён. Скрипты создания веток не вызываются.
## Решение: эталон main
Fetch успешен после разрешённой эскалации доступа к .git; main=origin/main=91689516c96d52877bba3480bf498b7d807ead64. codex/rust-rewrite=ce3b76a (полный SHA см. validation.md). Содержимое main извлечено git archive во временный каталог, не поверх рабочей ветки. Альтернатива worktree не нужна для чтения ref и отдельной сборки.
## Решение: уровни доказательства
Сборка и старт процесса подтверждены; ни один визуальный экран не подтверждён. Computer Use: FrameArrived timed out, повтор: window capture timed out. Предпочтения временно заменены обезличенным CSV и восстановлены побайтно, SHA-256 совпал. Снимков нет; каталог design-reference не создаётся. Реконструкции не делаются.
## Решение: документация, а не редизайн
Недостатки исходника и противоречия спецификации Rust перечисляются отдельно. Удалённые команды изучены по коду, не исполнены. Исходники, publish, конституция и прежние specs не редактируются.
