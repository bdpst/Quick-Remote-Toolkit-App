# Feature Specification: Rust-only release 0.4.0

**Feature Branch**: `codex/rust-rewrite`
**Created**: 2026-11-17
**Status**: Accepted for implementation
**Input**: Maintainer approval of Rust UI; retire legacy implementations, merge main and publish stable 0.4.0.

## User Scenarios & Testing

### User Story 1 - Upgrade without losing data (Priority: P1)
Users download the Windows package and keep existing CSV, favorites and settings.
**Why this priority**: Existing support workflows must continue.
**Independent Test**: Start extracted package with isolated settings and synthetic CSV.
**Acceptance Scenarios**:
1. Given existing settings, when 0.4.0 starts, then values remain readable and saving preserves unknown fields.
2. Given the package, when launched, then the main window identifies version 0.4.0 without an old executable or managed runtime.

### User Story 2 - One supported implementation (Priority: P1)
Contributors build and maintain only the accepted application.
**Why this priority**: Removes conflicting instructions and unused code.
**Independent Test**: Follow README from a checkout; inspect tracked files and docs.
**Acceptance Scenarios**:
1. Given the source tree, when following build commands, then only the supported Windows application is built.
2. Given historical specifications, when reading them, then superseded context and unperformed checks are explicit.

### User Story 3 - Verified public release (Priority: P1)
Users obtain a stable 0.4.0 release built from the verified main commit.
**Why this priority**: A tag alone is not an installable release.
**Independent Test**: Download published archive, compare checksums, inspect executable version and tag ancestry.
**Acceptance Scenarios**:
1. Given successful required checks, when merged, then main contains the accepted implementation and tag 0.4.0 points at that commit.
2. Given the release, when downloading its Windows archive, then resources, version and checksums match the release build.

### Edge Cases
- Existing 0.4.0 tag/release: inspect and stop conflicting publication; never move a tag silently.
- Protected main or missing approval: respect rules and report exact blocker.
- Locked local executable: preserve the running process and update safely or report the lock.
- No authorized remote test machines: do not run administrative actions or claim network integration success.

## Requirements

### Functional Requirements
- **FR-001**: Preserve accepted UI, CSV encodings/schema, settings path and backup behavior.
- **FR-002**: Remove exclusive legacy code, prototypes and comparison tools after checking references; retain shared assets and Git history.
- **FR-003**: Make current docs describe one supported implementation; mark historical evidence without falsifying open checks.
- **FR-004**: Set package, executable metadata, UI and distribution version to 0.4.0.
- **FR-005**: Check formatting, lint, tests, Windows release/package, isolated launch and both themes/dialogs.
- **FR-006**: Merge through allowed GitHub rules, verify main CI, then publish stable exact tag 0.4.0 with Windows archive from that commit.
- **FR-007**: Preserve user files outside the repository; exclude personal data and build products from Git.

### Key Entities
- Release: version, immutable tag, verified main commit, notes, archive and checksum.
- Existing profile: unchanged CSV references, settings and backup.

## Success Criteria

### Measurable Outcomes
- **SC-001**: One supported implementation and one current build/release instruction set remain.
- **SC-002**: All required local and main CI checks pass with explicit evidence boundaries.
- **SC-003**: Stable release 0.4.0 and downloadable distribution identify the exact verified main commit.
- **SC-004**: No actual client data or generated executable is committed; upgrade retains settings contract.

## Assumptions
- Approval of Rust is final; exhaustive legacy pixel parity is no longer a release gate.
- No new remote-operation behavior is requested; unauthorized network integration and clean desktop VM tests remain explicitly unperformed.
- Keep executable/archive names for compatibility. No additional feature specification of the application is needed.
