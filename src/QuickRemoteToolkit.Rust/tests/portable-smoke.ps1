param(
    [Parameter(Mandatory)][string]$Archive,
    [Parameter(Mandatory)][string]$OutputDirectory
)
$ErrorActionPreference = 'Stop'
$taskArchive = (Resolve-Path -LiteralPath $Archive).Path
$taskOutput = [IO.Path]::GetFullPath($OutputDirectory)
if (Test-Path -LiteralPath $taskOutput) { throw 'Use a new isolated smoke directory' }
New-Item -ItemType Directory -Path $taskOutput | Out-Null
$taskPackage = Join-Path $taskOutput 'package'
Expand-Archive -LiteralPath $taskArchive -DestinationPath $taskPackage
$taskInfo = Get-Content -LiteralPath (Join-Path $taskPackage 'BUILD-INFO.json') -Raw | ConvertFrom-Json
$taskExe = Join-Path $taskPackage 'QuickRemoteToolkitRust.exe'
if ((Get-FileHash -LiteralPath $taskExe).Hash.ToLowerInvariant() -ne $taskInfo.exe_sha256) { throw 'Extracted EXE hash mismatch' }
$taskProfile = Join-Path $taskOutput 'profile'
$taskSettingsDir = Join-Path $taskProfile 'QuickRemoteToolkitApp'
New-Item -ItemType Directory -Path $taskSettingsDir -Force | Out-Null
$taskCsv = Join-Path $taskOutput 'demo.csv'
[IO.File]::WriteAllText($taskCsv, "number;computer;ip;person`n1;PC-DEMO;192.0.2.1;Тестовый сотрудник`n", [Text.UTF8Encoding]::new($false))
$taskSettings = @{ClientsCsvPath=$taskCsv;PingTimeoutMs=1500;ParallelPingLimit=8;FavoriteComputers=@('computer:PC-DEMO');ShowFavoritesOnly=$false;IsDarkTheme=$true;ReleaseSmokeUnknown='preserved'}
$taskSettingsFile = Join-Path $taskSettingsDir 'settings.json'
$taskSettings | ConvertTo-Json | Set-Content -LiteralPath $taskSettingsFile -Encoding utf8
$taskStart = [Diagnostics.ProcessStartInfo]::new($taskExe)
$taskStart.UseShellExecute = $false
$taskStart.WorkingDirectory = $taskPackage
$taskStart.Environment['APPDATA'] = $taskProfile
$taskStart.Environment['PATH'] = Join-Path $env:WINDIR 'System32'
foreach ($taskEnvName in @('DOTNET_ROOT','CARGO_HOME','RUSTUP_HOME')) { $taskStart.Environment.Remove($taskEnvName) | Out-Null }
$taskProcess = [Diagnostics.Process]::Start($taskStart)
try {
    $taskReady = $false
    for ($taskAttempt=0; $taskAttempt -lt 80; $taskAttempt++) {
        if ($taskProcess.HasExited) { throw "Package exited early: $($taskProcess.ExitCode)" }
        $taskProcess.Refresh()
        if ($taskProcess.MainWindowHandle -ne 0 -and $taskProcess.MainWindowTitle -like "*$($taskInfo.version)*") { $taskReady=$true; break }
        Start-Sleep -Milliseconds 100
    }
    if (!$taskReady) { throw 'No package window registered' }
    $taskTitle = $taskProcess.MainWindowTitle
    if ($taskTitle -notlike "*$($taskInfo.version)*") { throw "Version absent in window title: $taskTitle" }
    if (!(Test-Path -LiteralPath $taskSettingsFile)) { throw 'Settings disappeared' }
    Start-Sleep -Milliseconds 500
    if (!$taskProcess.CloseMainWindow()) { throw 'Could not request normal window close' }
    if (!$taskProcess.WaitForExit(8000)) { throw 'Normal close timed out' }
    if ($taskProcess.ExitCode -ne 0) { throw "Package exit code: $($taskProcess.ExitCode)" }
    $taskSaved = Get-Content -LiteralPath $taskSettingsFile -Raw | ConvertFrom-Json
    if ($taskSaved.ReleaseSmokeUnknown -ne 'preserved' -or $taskSaved.ClientsCsvPath -ne $taskCsv -or !$taskSaved.IsDarkTheme -or $taskSaved.FavoriteComputers[0] -ne 'computer:PC-DEMO') { throw 'Settings compatibility failed' }
    if (!(Test-Path -LiteralPath (Join-Path $taskSettingsDir 'settings.before-rust.json'))) { throw 'Settings backup missing' }
    @{version=$taskInfo.version;commit=$taskInfo.commit;window_title=$taskTitle;normal_close=$true;settings_preserved=$true;isolated_appdata=$true;remote_actions_run=$false;clean_machine=$false;archive_sha256=(Get-FileHash -LiteralPath $taskArchive).Hash.ToLowerInvariant()} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $taskOutput 'result.json') -Encoding utf8
    Write-Output 'PASS extracted package launch, title version, normal close and settings compatibility'
} finally {
    if (!$taskProcess.HasExited) { $taskProcess.Kill(); $taskProcess.WaitForExit() }
    $taskProcess.Dispose()
}
