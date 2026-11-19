# Data model аудита

Finding: id (уникальный UX-NN), priority (P0–P3), screen, source (runtime / QA snapshot / source), steps, actual, impact, proposal, acceptance. Hypothesis хранится отдельно и не считается подтверждённой находкой.
ValidationScenario: id, themes, viewport, pixels_per_point, method, result (PASS / PARTIAL / NOT RUN), evidence, limitation.
Evidence: path, mainCommit, captureDate, syntheticData, provenance. Только обезличенные изображения без чужих окон.
Связи: сценарий ссылается на доказательство, находка — на сценарий/исходник. Жизненный цикл: наблюдение → подтверждение либо гипотеза → отдельная будущая реализация; аудит не закрывает исправления.
