# Rust application

Единственная поддерживаемая реализация Quick Remote Toolkit.
Команды сборки, требования, настройки и выпуск: [основной README](../../README.md).

src/: ui/design, models, services, native, columns/selection, icon.
assets/: SVG/ICO и Windows manifest. build.rs создаёт Windows-ресурсы из Cargo version.
build.ps1: static CRT, ZIP, BUILD-INFO.json, SHA256SUMS.
tests/: проверка PE/ZIP и запуск распакованной сборки; Rust-host ui-qa использует
изолированный профиль и не поставляется пользователям.
