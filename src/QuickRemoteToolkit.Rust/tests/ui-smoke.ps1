param(
    [Parameter(Mandatory)][string]$OutputDirectory,
    [string[]]$Scenes = @('main','message','admin','message-error','admin-error','search','empty','busy','collapsed','reordered','logs','long','context'),
    [int]$Width=1500,
    [int]$Height=920
)
$ErrorActionPreference = 'Stop'
$taskOutputRoot = [IO.Path]::GetFullPath($OutputDirectory)
New-Item -ItemType Directory -Path $taskOutputRoot -Force | Out-Null
$taskProfile = Join-Path $taskOutputRoot 'profile'
New-Item -ItemType Directory -Path (Join-Path $taskProfile 'QuickRemoteToolkitApp') -Force | Out-Null
$taskCsv = Join-Path $taskOutputRoot 'demo.csv'
[IO.File]::WriteAllText($taskCsv, "number;computer;ip;person`r`n1;PC-DEMO;192.0.2.1;Тестовый сотрудник`r`n2;PC-DEMO-02;192.0.2.2;Сотрудник с длинным именем`r`n", [Text.UTF8Encoding]::new($false))
& cargo build --locked --manifest-path (Join-Path $PSScriptRoot '../Cargo.toml') --features ui-qa --bin qrt-ui-qa
if ($LASTEXITCODE -ne 0) { throw 'QA host build failed' }
$taskExe = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../target/debug/qrt-ui-qa.exe'))
$taskResults = @()
foreach ($taskDark in @($false,$true)) {
    $taskTheme = if ($taskDark) {'dark'} else {'light'}
    $taskSettings = @{ClientsCsvPath=$taskCsv;PingTimeoutMs=1500;ParallelPingLimit=8;FavoriteComputers=@('computer:PC-DEMO');ShowFavoritesOnly=$false;IsDarkTheme=$taskDark}
    $taskSettings | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $taskProfile 'QuickRemoteToolkitApp/settings.json') -Encoding utf8
    foreach ($taskScene in $Scenes) {
        $taskSceneOutput = Join-Path $taskOutputRoot "$taskTheme-$taskScene"
        if (Test-Path -LiteralPath (Join-Path $taskSceneOutput 'main.png')) { throw "Evidence already exists: $taskSceneOutput" }
        New-Item -ItemType Directory -Path $taskSceneOutput -Force | Out-Null
        $taskStart = [Diagnostics.ProcessStartInfo]::new($taskExe)
        $taskStart.UseShellExecute = $false
        $taskStart.Environment['APPDATA'] = $taskProfile
        $taskStart.Environment['QRT_QA_PROFILE'] = $taskProfile
        $taskStart.Environment['QRT_QA_OUTPUT'] = $taskSceneOutput
        $taskStart.Environment['QRT_QA_SCENE'] = $taskScene
        $taskStart.Environment['QRT_QA_WIDTH'] = $Width.ToString()
        $taskStart.Environment['QRT_QA_HEIGHT'] = $Height.ToString()
        $taskProcess = [Diagnostics.Process]::Start($taskStart)
        if (!$taskProcess.WaitForExit(8000)) {
            if ($taskProcess.MainModule.FileName -eq $taskExe) { $taskProcess.Kill() }
            throw "QA timeout: $taskTheme $taskScene"
        }
        if ($taskProcess.ExitCode -ne 0) { throw "QA exit $($taskProcess.ExitCode): $taskTheme $taskScene" }
        $taskState = Get-Content -LiteralPath (Join-Path $taskSceneOutput 'state.json') -Raw | ConvertFrom-Json
        if ($taskState.messaging -or $taskState.administering) { throw 'QA unexpectedly started a remote operation' }
        if ($taskScene -eq 'message-error' -and $taskState.dialog_error -ne 'Укажите время показа от 1 до 3600 секунд.') { throw 'Ctrl+Enter validation failed' }
        if ($taskScene -eq 'admin-error' -and $taskState.dialog_error -ne 'Введите учётную запись в формате ДОМЕН\логин или ИМЯ-ПК\логин.') { throw 'Enter validation failed' }
        $taskResults += @{theme=$taskTheme;scene=$taskScene;exit=0;state=$taskState}
        Write-Output "PASS $taskTheme $taskScene"
    }
}
$taskResults | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $taskOutputRoot 'results.json') -Encoding utf8
