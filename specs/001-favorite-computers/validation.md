# Validation — 2026-11-06
## Results
- Release build: PASS, 0 warnings, 0 errors. Explicit x64 SDK executable used because PATH resolves x86 runtime without SDK first.
- Isolated WPF harness using copied application source under ignored obj/favorites-validation, with settings path redirected to its test-profile: 10 PASS assertions.
- Assertions: old settings defaults; stable name/case/IP changes; fallback/empty identity; duplicate synchronization; favorites filter; search AND favorites; new MainWindow restores saved state; CSV reload; removal hides all duplicate rows; removal persisted.
- Light and dark WPF RenderTargetBitmap images inspected: stars and filter readable in both palettes.
- Settings written only to isolated profile; synthetic PC-DEMO/PC-OTHER and documentation IP range 192.0.2.x used.
- git diff --check: PASS.
## Limits
Harness invokes event handlers directly. Physical keyboard/mouse input and disk-denial error dialog were not exercised. Error rollback and double-click guard reviewed in code. Remote commands were not changed or executed.
## Reproduce manually
Follow quickstart.md against samples CSV; test failure dialog only in an isolated profile. Temporary harness and PNG files are local ignored build artifacts, not application sources.
