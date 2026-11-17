# Validation — 0.4.0

## Local Windows checks (2026-11-17)

- fmt --check, clippy all targets -D warnings, clippy UI QA host: PASS.
- cargo test --locked: 27 passed, 0 failed. Windows release build and static CRT package: PASS.
- Windows FileVersion/ProductVersion: 0.4.0; PE AMD64/GUI, icon/group/version/manifest, PerMonitorV2/asInvoker, no CLR or external VC runtime: PASS.
- Three ZIP entries match EXE/runtime/build metadata; timestamps fixed, version/hash checks PASS.
- Extracted ZIP launched with isolated APPDATA, synthetic CSV and PATH=System32; native title contains 0.4.0. Normal close, CSV path/theme/favorite/unknown JSON field and backup preserved: PASS.
- Initial smoke caught a readiness race in the test (intermediate titleless window); corrected wait for versioned main window, repeated smoke PASS. No application behavior change was needed.
- QA host: 10 passes, both themes × main/message/admin/message-error/admin-error. No remote action started. Current real framebuffer screenshots in docs/images; no WPF screenshot presented as Rust.
- Current docs/source links audited; legacy technology references retained only as history, compatibility rationale or Win32 Windows API names.
- WPF source/solution, C++ prototype and reference comparison/icon-conversion tools removed after Rust dependency audit. Shared CSV and Rust assets retained. No user data outside repo removed.

Details: local-validation.json. Local precommit package has source_dirty=true and base HEAD, so it is validation evidence, not the final release artifact. Final release must come from clean checked main CI.

## Release gates

- [PR #1](https://github.com/bdpst/Quick-Remote-Toolkit-App/pull/1) merged as c8fa0afbf90c79900069f174835ba3bde480ae7b. Branch and PR CI passed.
- [Main CI](https://github.com/bdpst/Quick-Remote-Toolkit-App/actions/runs/44031272654) on that merge commit passed before tag creation.
- Exact annotated tag 0.4.0 points at that checked main commit. [Release workflow](https://github.com/bdpst/Quick-Remote-Toolkit-App/actions/runs/44031880849) succeeded and published a stable [Release](https://github.com/bdpst/Quick-Remote-Toolkit-App/releases/tag/0.4.0).
- Downloaded all three public assets; GitHub digests and SHA256SUMS matched. ZIP SHA-256: 45cd1aa29af26f2bbfbe7f12f9028653f3926e94e5401be60e3914f81b03b82e.
- Downloaded Windows EXE has FileVersion/ProductVersion 0.4.0 and correct resources. Embedded BUILD-INFO.json matches the external asset and checked commit; source_dirty=false.
- Published ZIP smoke passed: normal launch and close, versioned main window, isolated settings/backup/unknown fields preserved. No remote operations performed.
- Local publish-rust updated to the downloaded release bytes, not a different local rebuild.

Full evidence: release-verification.json. This post-publication documentation record does not change the tagged application or move tag 0.4.0. Later documentation commits on main are distinct from the immutable release source commit.

## Unperformed checks

No authorized test computers were provided: real SMB/WinRM/WTS/domain/member changes remain untested. A separate clean desktop Windows without SDK and exhaustive physical DPI/multimonitor tests were not run. These old open checks are not marked successful; maintainer accepted Rust independently of exhaustive legacy parity.
