param([Parameter(Mandatory)][string]$OutputDirectory,[string[]]$Scenes=@('main','message','admin','message-error','admin-error','context'),[int]$Width=1500,[int]$Height=920)
$ErrorActionPreference='Stop'
$taskRepo=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../../..'))
$taskSha='91689516c96d52877bba3480bf498b7d807ead64'
$taskRoot=Join-Path $env:TEMP ('qrt-wpf-reference-'+[Guid]::NewGuid().ToString('N'))
$taskReference=Join-Path $taskRoot 'reference'
$taskHost=Join-Path $taskRoot 'host'
$taskOutput=[IO.Path]::GetFullPath($OutputDirectory)
New-Item -ItemType Directory -Path $taskRoot,$taskHost,$taskOutput -Force | Out-Null
& git -C $taskRepo archive --format=zip "--output=$taskRoot/reference.zip" $taskSha
if ($LASTEXITCODE -ne 0) {throw 'Reference archive failed'}
Expand-Archive -LiteralPath (Join-Path $taskRoot 'reference.zip') -DestinationPath $taskReference
$taskApp=Join-Path $taskReference 'src/QuickRemoteToolkit.App'
$taskSettings=Join-Path $taskApp 'Services/SettingsService.cs'
$taskCode=Get-Content -LiteralPath $taskSettings -Raw
$taskNeedle='Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),'
if (!$taskCode.Contains($taskNeedle)) {throw 'Unexpected SettingsService source'}
$taskCode.Replace($taskNeedle,'Environment.GetEnvironmentVariable("QRT_QA_APPDATA") ?? throw new InvalidOperationException("QA profile required"),') | Set-Content -LiteralPath $taskSettings -Encoding utf8
$taskAppXaml=Join-Path $taskApp 'App.xaml'
(Get-Content -LiteralPath $taskAppXaml -Raw).Replace('StartupUri="MainWindow.xaml"','') | Set-Content -LiteralPath $taskAppXaml -Encoding utf8
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'reference-host.cs') -Destination (Join-Path $taskHost 'Program.cs')
$taskProjectReference=[Security.SecurityElement]::Escape((Join-Path $taskApp 'QuickRemoteToolkit.App.csproj'))
@"
<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>WinExe</OutputType><TargetFramework>net10.0-windows</TargetFramework><UseWPF>true</UseWPF><ImplicitUsings>enable</ImplicitUsings><EnableDefaultCompileItems>false</EnableDefaultCompileItems></PropertyGroup><ItemGroup><Compile Include="Program.cs"/><ProjectReference Include="$taskProjectReference"/></ItemGroup></Project>
"@ | Set-Content -LiteralPath (Join-Path $taskHost 'host.csproj') -Encoding utf8
$taskDotnet=Join-Path $env:ProgramW6432 'dotnet/dotnet.exe'
& $taskDotnet build (Join-Path $taskHost 'host.csproj') -c Release --nologo
if ($LASTEXITCODE -ne 0) {throw 'Reference host build failed'}
$taskProfile=Join-Path $taskRoot 'profile'
New-Item -ItemType Directory -Path (Join-Path $taskProfile 'QuickRemoteToolkitApp') -Force | Out-Null
$taskCsv=Join-Path $taskRoot 'demo.csv'
[IO.File]::WriteAllText($taskCsv,"number;computer;ip;person`r`n1;PC-DEMO;192.0.2.1;Тестовый сотрудник`r`n2;PC-DEMO-02;192.0.2.2;Сотрудник с длинным именем`r`n",[Text.UTF8Encoding]::new($false))
foreach($taskDark in @($false,$true)) {
    $taskTheme=if($taskDark){'dark'}else{'light'}
    @{ClientsCsvPath=$taskCsv;PingTimeoutMs=1500;ParallelPingLimit=8;FavoriteComputers=@('computer:PC-DEMO');ShowFavoritesOnly=$false;IsDarkTheme=$taskDark} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $taskProfile 'QuickRemoteToolkitApp/settings.json') -Encoding utf8
    foreach($taskScene in $Scenes) {
        $taskImage=Join-Path $taskOutput "$taskTheme-$taskScene.png"
        if(Test-Path -LiteralPath $taskImage){throw "Evidence already exists: $taskImage"}
        $taskExe=Join-Path $taskHost 'bin/Release/net10.0-windows/host.exe'
        $taskStart=[Diagnostics.ProcessStartInfo]::new($taskExe)
        $taskStart.UseShellExecute=$false
        $taskStart.Environment['QRT_QA_APPDATA']=$taskProfile
        $taskStart.Environment['QRT_QA_IMAGE']=$taskImage
        $taskStart.Environment['QRT_QA_SCENE']=$taskScene
        $taskStart.Environment['QRT_QA_WIDTH']=$Width.ToString()
        $taskStart.Environment['QRT_QA_HEIGHT']=$Height.ToString()
        $taskStart.Environment['QRT_QA_DARK']=if($taskDark){'1'}else{'0'}
        $taskProcess=[Diagnostics.Process]::Start($taskStart)
        if(!$taskProcess.WaitForExit(8000)){if($taskProcess.MainModule.FileName -eq $taskExe){$taskProcess.Kill()};throw "Reference timeout: $taskScene"}
        if($taskProcess.ExitCode -ne 0){throw "Reference exit $($taskProcess.ExitCode)"}
        Write-Output "PASS WPF $taskTheme $taskScene"
    }
}
Write-Output "Reference host and isolated profile retained: $taskRoot"
