# Release research — 2026-11-17

- Decision: retain accepted Rust tree. origin/main 91689516c96d52877bba3480bf498b7d807ead64 is an ancestor of origin/codex/rust-rewrite a9a6a893bef84747b8fc1a838821e5962dd7de9d; no independent main commits. Working tree was clean. No conflict requires behavioral arbitration.
- Decision: release exact 0.4.0 after main CI. Tags/release stop at v0.3.0; 0.4.0 absent. Rust push CI 44028350760 succeeded. GitHub protection endpoint says Branch not protected; rulesets empty. Still use reviewable PR and wait for checks before merge.
- Decision: remove WPF/Win32 sources, solution, C# reference host/scripts and WPF icon generator. Rust build.rs uses only Rust assets and Windows SDK rc.exe; no old executable is used. Keep common CSV and Rust SVG/ICO. Historical specs retain links to Git history.
- Decision: keep executable/archive names and existing settings location. Renaming provides no user benefit and would break familiar launch paths. User explicitly approved UI; historical exhaustive parity checks remain unperformed, not silently closed.
- Decision: GitHub connector for repository/PR actions; scoped authenticated REST fallback for release/run/rules endpoints absent from connector. No branch-rule bypass.
