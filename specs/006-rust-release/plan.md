# Implementation Plan: Rust-only release 0.4.0

**Branch**: `codex/rust-rewrite` | **Date**: 2026-11-17 | **Spec**: [spec.md](spec.md)

## Summary
Retire unused implementations without changing accepted Rust behavior. Version Windows metadata and UI, consolidate docs, validate the package, merge main and publish exact tag 0.4.0.

## Technical Context
**Language/Version**: Rust 1.99.1, edition 2024; Windows x64 MSVC.
**Primary Dependencies**: eframe/egui 0.33.3 (glow, AccessKit), Windows APIs, rfd, serde.
**Storage**: existing AppData JSON; UTF-8/Windows-1251 CSV; in-memory logs and explicit export.
**Testing**: fmt, clippy, 27 existing tests, QA host, packaged process smoke, PE/ZIP inspection, Windows CI.
**Target Platform**: Windows 10/11 x64 with OpenGL-capable driver.
**Project Type**: desktop application.
**Performance Goals**: preserve responsive UI and background tasks; no performance behavior changes.
**Constraints**: no passwords, real client data or unapproved remote targets; no force push/tag replacement.
**Scale/Scope**: one application, no rewrite, one stable release.

## Constitution Check
Maintainer-approved retirement supersedes WPF retention rule; update constitution to 3.0.0. Retain session credentials, CSV/settings compatibility, bounded administrative effects and truthful validation. No deviations after amendment.

## Phase 0: Research
See research.md: current branches, prior release, CI and GitHub rules verified. No unresolved clarification.

## Phase 1: Design
See data-model.md and contracts/release.md. Keep current binary names. Cargo package version is source of truth; derive Windows metadata, UI and package metadata from it. Release workflow checks tag/version equality and publishes only on successful Windows checks.

## Project Structure
- src/QuickRemoteToolkit.Rust/: sole implementation and Rust QA host.
- docs/: current design and screenshots; README owns setup/user/release instructions.
- specs/001–005/: explicitly historical evidence; 006 owns finalization and validation.
- .github/workflows/release.yml: Windows validation/artifact and tag-only release job.

## Complexity Tracking
No new runtime dependencies. PE resource generation uses existing Windows SDK. PowerShell packaging and stdlib Python verification are build/test-only.
