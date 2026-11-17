param([string]$OutputDirectory = (Join-Path $PSScriptRoot '../../publish-rust'))
$ErrorActionPreference = 'Stop'
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($cargo) { $cargoPath = $cargo.Source } else { $cargoPath = Join-Path $env:USERPROFILE '.cargo/bin/cargo.exe' }
# Build release with static CRT and deterministic MSVC PE linking.
& $cargoPath rustc --release --locked --bin QuickRemoteToolkitRust --manifest-path (Join-Path $PSScriptRoot 'Cargo.toml') -- -C target-feature=+crt-static -C link-arg=/Brepro
if ($LASTEXITCODE -ne 0) { throw 'Deterministic Rust link failed' }

$manifest = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'Cargo.toml') -Raw
$version = [regex]::Match($manifest, '(?m)^version = "([0-9]+\.[0-9]+\.[0-9]+)"').Groups[1].Value
if (!$version) { throw 'Cargo package version missing' }
$sourceCommit = (& git -C $PSScriptRoot rev-parse HEAD).Trim()
if ($LASTEXITCODE -ne 0) { throw 'Source commit unavailable' }
if ($env:GITHUB_REF_TYPE -eq 'tag' -and $env:GITHUB_REF_NAME.TrimStart('v') -ne $version) { throw 'Tag does not match package version' }
New-Item -ItemType Directory -Force $OutputDirectory | Out-Null
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'target/release/QuickRemoteToolkitRust.exe') -Destination $OutputDirectory -Force
Get-Item (Join-Path $OutputDirectory 'QuickRemoteToolkitRust.exe') | Select-Object FullName,Length,LastWriteTime

$runtimeText = @"
Quick Remote Toolkit $version — Windows x64
Запуск: QuickRemoteToolkitRust.exe
.NET Desktop Runtime и средства разработки для запуска не нужны.
Нужны Windows 10/11 x64, поддерживающий OpenGL драйвер и штатные Windows DLL.
WinRM/WinRS/SMB и права текущего пользователя нужны соответствующим удалённым действиям.
Настройки: %AppData%\QuickRemoteToolkitApp\settings.json
Резервная копия до первой записи: settings.before-rust.json
Журнал хранится в памяти до закрытия; сохранение через экспорт TXT/CSV.
Пароли не хранятся. Не запускайте несколько экземпляров для изменения одних настроек.
Исходный коммит: $sourceCommit
Документация: https://github.com/bdpst/Quick-Remote-Toolkit-App
"@
[System.IO.File]::WriteAllText((Join-Path $OutputDirectory 'RUNTIME.txt'), $runtimeText, [System.Text.UTF8Encoding]::new($false))
$buildInfo = [ordered]@{
    version = $version
    commit = $sourceCommit
    source_dirty = [bool](& git -C $PSScriptRoot status --porcelain --untracked-files=no)
    target = 'x86_64-pc-windows-msvc'
    exe_sha256 = (Get-FileHash -LiteralPath (Join-Path $OutputDirectory 'QuickRemoteToolkitRust.exe') -Algorithm SHA256).Hash.ToLowerInvariant()
}
[IO.File]::WriteAllText((Join-Path $OutputDirectory 'BUILD-INFO.json'), ($buildInfo | ConvertTo-Json) + "`n", [Text.UTF8Encoding]::new($false))
$archivePath = Join-Path ([System.IO.Path]::GetFullPath($OutputDirectory)) 'QuickRemoteToolkit.Rust-win-x64.zip'
if (Test-Path -LiteralPath $archivePath) { Remove-Item -LiteralPath $archivePath }
$archive = [System.IO.Compression.ZipFile]::Open($archivePath, [System.IO.Compression.ZipArchiveMode]::Create)
try {
    foreach ($fileName in @('QuickRemoteToolkitRust.exe', 'RUNTIME.txt', 'BUILD-INFO.json')) {
        $entry = $archive.CreateEntry($fileName, [System.IO.Compression.CompressionLevel]::Optimal)
        $entry.LastWriteTime = [System.DateTimeOffset]::new(1980, 1, 1, 0, 0, 0, [System.TimeSpan]::Zero)
        $entryStream = $entry.Open()
        $inputStream = [System.IO.File]::OpenRead((Join-Path $OutputDirectory $fileName))
        try { $inputStream.CopyTo($entryStream) } finally { $inputStream.Dispose(); $entryStream.Dispose() }
    }
} finally { $archive.Dispose() }
$archiveHash = (Get-FileHash -LiteralPath $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
[IO.File]::WriteAllText((Join-Path $OutputDirectory 'SHA256SUMS'), "$archiveHash  QuickRemoteToolkit.Rust-win-x64.zip`n", [Text.UTF8Encoding]::new($false))
Get-FileHash -LiteralPath $archivePath -Algorithm SHA256
