# Validation quickstart

Follow README build/check commands on Windows. Run tests/ui-smoke.ps1 with an isolated output folder for both themes/main/message/admin. Verify the ZIP with tests/verify-artifact.py and run tests/portable-smoke.ps1 on the extracted package using synthetic CSV only. After PR/main CI, tag checked main 0.4.0; wait for workflow, download release assets, compare commit/version/checksums. Record actual results in validation.md. No remote administrative smoke targets are authorized.
