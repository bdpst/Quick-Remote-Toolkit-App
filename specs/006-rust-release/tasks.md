# Tasks: Rust-only release 0.4.0

## Phase 1: Setup
- [x] T001 Inspect branches, tags, CI, packaging and GitHub rules; record specs/006-rust-release/research.md.
- [x] T002 Specify bounded finalization, resolve requirements and create plan/contracts/checklist in specs/006-rust-release/.

## Phase 2: Foundation
- [x] T003 Amend .specify/memory/constitution.md and AGENTS.md for sole Rust support.

## Phase 3: US1 — Compatible package
Independent test: unpacked release process with isolated AppData and demo CSV; version/resource checks.
- [x] T004 [US1] Derive version 0.4.0 in Cargo.toml/Cargo.lock, build.rs Windows metadata, src/main.rs and src/ui.rs.
- [x] T005 [US1] Update build.ps1 package provenance and tests/verify-artifact.py; add tests/portable-smoke.ps1.

## Phase 4: US2 — One implementation
Independent test: source/reference audit and README commands build without old code.
- [x] T006 [US2] Remove exclusive src/QuickRemoteToolkit.App, src/QuickRemoteToolkit.Win32, solution and comparison tools after dependency audit.
- [x] T007 [US2] Consolidate README/design/checklist/assets docs, CHANGELOG; mark historical specs and keep unperformed tests open.

## Phase 5: US3 — Publish
Independent test: published ZIP version/checksum/main commit and stable release URL.
- [x] T008 [US3] Update .github/workflows/release.yml for main checks, exact 0.4.0 trigger and stable release assets/notes.
- [x] T009 [US3] Run Windows fmt/clippy/tests/release/package/PE, packaged smoke, both-theme main/dialog QA; record validation.md and current screenshots.
- [x] T010 [US3] Review diff/links/privacy; commit, push, PR/allowed merge; verify final main CI.
- [x] T011 [US3] Tag verified main 0.4.0; await workflow and verify downloaded release/version/commit/checksums.

## Phase 6: Polish
- [x] T012 Record completed release evidence, final status and remaining unperformed integration checks in validation.md.

## Dependencies and strategy
T001→T002→T003→T004→T005→T006→T007→T008→T009→T010→T011→T012.
US1 package and US2 documentation inspections can be read independently; mutations and publication remain sequential. MVP is compatible package; task completes only with published verified release or a concrete external blocker.
