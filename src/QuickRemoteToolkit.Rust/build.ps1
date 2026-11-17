param([string]$OutputDirectory = (Join-Path $PSScriptRoot '../../publish-rust'))
$ErrorActionPreference = 'Stop'
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($cargo) { $cargoPath = $cargo.Source } else { $cargoPath = Join-Path $env:USERPROFILE '.cargo/bin/cargo.exe' }
# Build release with static CRT and deterministic MSVC PE linking.
& $cargoPath rustc --release --locked --bin QuickRemoteToolkitRust --manifest-path (Join-Path $PSScriptRoot 'Cargo.toml') -- -C target-feature=+crt-static -C link-arg=/Brepro
if ($LASTEXITCODE -ne 0) { throw 'Deterministic Rust link failed' }

New-Item -ItemType Directory -Force $OutputDirectory | Out-Null
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'target/release/QuickRemoteToolkitRust.exe') -Destination $OutputDirectory -Force
Get-Item (Join-Path $OutputDirectory 'QuickRemoteToolkitRust.exe') | Select-Object FullName,Length,LastWriteTime

$runtimeText = @'
Quick Remote Toolkit Rust — Windows x64
Запуск: QuickRemoteToolkitRust.exe
.NET Desktop Runtime и средства разработки для запуска не нужны.
Нужны Windows 10/11 x64, поддерживающий OpenGL драйвер и штатные Windows DLL.
WinRM/WinRS/SMB и права текущего пользователя нужны соответствующим удалённым действиям.
Настройки: %AppData%\QuickRemoteToolkitApp\settings.json
Резервная копия до первой записи: settings.before-rust.json
Не запускать WPF/Rust одновременно при изменении настроек.
Паритет UI и реальных удалённых действий проверяется отдельно: specs/005-rust-parity.
'@
[System.IO.File]::WriteAllText((Join-Path $OutputDirectory 'RUNTIME.txt'), $runtimeText, [System.Text.UTF8Encoding]::new($false))
$archivePath = Join-Path ([System.IO.Path]::GetFullPath($OutputDirectory)) 'QuickRemoteToolkit.Rust-win-x64.zip'
if (Test-Path -LiteralPath $archivePath) { Remove-Item -LiteralPath $archivePath }
$archive = [System.IO.Compression.ZipFile]::Open($archivePath, [System.IO.Compression.ZipArchiveMode]::Create)
try {
    foreach ($fileName in @('QuickRemoteToolkitRust.exe', 'RUNTIME.txt')) {
        $entry = $archive.CreateEntry($fileName, [System.IO.Compression.CompressionLevel]::Optimal)
        $entry.LastWriteTime = [System.DateTimeOffset]::new(1980, 1, 1, 0, 0, 0, [System.TimeSpan]::Zero)
        $entryStream = $entry.Open()
        $inputStream = [System.IO.File]::OpenRead((Join-Path $OutputDirectory $fileName))
        try { $inputStream.CopyTo($entryStream) } finally { $inputStream.Dispose(); $entryStream.Dispose() }
    }
} finally { $archive.Dispose() }
Get-FileHash -LiteralPath $archivePath -Algorithm SHA256
