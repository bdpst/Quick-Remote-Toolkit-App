# Research

- Decision: база origin/main 7bdbe3b30bb3940a8aaf9394af6b399d31a0fd0f после git fetch 2026-11-19; чистое дерево, отдельная ветка codex/007-ui-ux-audit.
  Rationale: воспроизводимый текущий Rust-only интерфейс. Alternatives: исторические WPF specs не используются как норматив.
- Decision: штатный QA-host + ручная работа с отдельным экземпляром и профилем.
  Rationale: host использует настоящий Toolkit/framebuffer, но состояния search/busy/reordered задаются напрямую; не доказывает соответствующие пользовательские действия. Alternatives: исходники/README-снимки — только вспомогательные.
- Decision: Impeccable critique, режим Operate; без web detector/Live/hooks по запросу пользователя.
  Rationale: egui не имеет DOM/CDP; критерии применимы, веб-инструменты нет. Alternatives: веб-детектор дал бы ложное ощущение проверки.
- Decision: design.rs — runtime источник, JSON — неполная ручная карта, без генерации.
  Rationale: код содержит дополнительные interaction colors, метрики и прямые status colors в ui.rs. Alternatives: изменение токенов/кода выходит за scope.
- Decision: физические масштабы фиксировать только по фактическим metadata; не имитировать Windows DPI изменением размера viewport.
  Rationale: размер окна и pixels_per_point — разные параметры.
- Extension hooks: .specify/extensions.yml отсутствует; before/after всех четырёх этапов не зарегистрированы.
- Spec resolution: sequential 007, active resolver применён; feature.json выбирает каталог независимо от имени ветки. setup-plan BRANCH сообщает feature fallback, фактическая Git-ветка указана выше.
