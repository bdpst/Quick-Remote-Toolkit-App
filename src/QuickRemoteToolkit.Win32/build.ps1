param([string]$Compiler = 'clang++')
$ErrorActionPreference = 'Stop'
$taskRepo = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\..'))
$taskOutput = Join-Path $taskRepo 'win32-publish'
New-Item -ItemType Directory -Path $taskOutput -Force | Out-Null
$taskCompilerDir = Split-Path (Get-Command $Compiler).Source
$taskResourceCompiler = Join-Path $taskCompilerDir 'llvm-windres.exe'
if (!(Test-Path $taskResourceCompiler)) { $taskResourceCompiler = Join-Path $taskCompilerDir 'windres.exe' }
$taskResource = Join-Path $taskOutput 'app.res.o'
& $taskResourceCompiler '-I' $PSScriptRoot (Join-Path $PSScriptRoot 'app.rc') '-o' $taskResource
if ($LASTEXITCODE -ne 0) { throw 'Resource build failed.' }
& $Compiler '-std=c++17' '-O2' '-Wall' '-Wextra' '-DUNICODE' '-D_UNICODE' '-municode' '-mwindows' '-static' (Join-Path $PSScriptRoot 'main.cpp') $taskResource '-o' (Join-Path $taskOutput 'QuickRemoteToolkitWin32.exe') '-lcomctl32' '-lcomdlg32' '-lshell32'
if ($LASTEXITCODE -ne 0) { throw 'C++ build failed.' }
Write-Output (Join-Path $taskOutput 'QuickRemoteToolkitWin32.exe')