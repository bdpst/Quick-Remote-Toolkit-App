# Design-time SVG path rasterizer. Cargo consumes the committed ICO, not this script.
# Run on Windows: pwsh -STA -NoProfile -File ./assets/generate-icon.ps1
param([string]$PreviewDirectory)
$ErrorActionPreference='Stop'
Add-Type -AssemblyName PresentationCore,WindowsBase
[xml]$taskSvg=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'QuickRemoteToolkit.svg') -Raw
$taskPaths=$taskSvg.SelectNodes('//*[local-name()="path"]')
$taskSizes=@(16,20,24,32,40,48,64,128,256)
$taskFrames=[Collections.Generic.List[byte[]]]::new()
foreach($taskSize in $taskSizes) {
    $taskVisual=[Windows.Media.DrawingVisual]::new()
    $taskDrawing=$taskVisual.RenderOpen()
    $taskDrawing.PushTransform([Windows.Media.ScaleTransform]::new($taskSize/16.0,$taskSize/16.0))
    foreach($taskPath in $taskPaths) {
        $taskBrush=[Windows.Media.BrushConverter]::new().ConvertFromInvariantString($taskPath.fill)
        $taskGeometry=[Windows.Media.Geometry]::Parse($taskPath.d)
        $taskDrawing.DrawGeometry($taskBrush,$null,$taskGeometry)
    }
    $taskDrawing.Pop()
    $taskDrawing.Close()
    $taskBitmap=[Windows.Media.Imaging.RenderTargetBitmap]::new($taskSize,$taskSize,96,96,[Windows.Media.PixelFormats]::Pbgra32)
    $taskBitmap.Render($taskVisual)
    $taskEncoder=[Windows.Media.Imaging.PngBitmapEncoder]::new()
    $taskEncoder.Frames.Add([Windows.Media.Imaging.BitmapFrame]::Create($taskBitmap))
    $taskStream=[IO.MemoryStream]::new()
    try {$taskEncoder.Save($taskStream);$taskPng=$taskStream.ToArray()} finally {$taskStream.Dispose()}
    $taskFrames.Add($taskPng)
    if($PreviewDirectory) {
        New-Item -ItemType Directory -Path $PreviewDirectory -Force | Out-Null
        [IO.File]::WriteAllBytes((Join-Path $PreviewDirectory "icon-$taskSize.png"),$taskPng)
    }
}
$taskOutput=[IO.MemoryStream]::new()
$taskWriter=[IO.BinaryWriter]::new($taskOutput)
try {
    $taskWriter.Write([ushort]0);$taskWriter.Write([ushort]1);$taskWriter.Write([ushort]$taskSizes.Count)
    $taskOffset=6+16*$taskSizes.Count
    for($taskIndex=0;$taskIndex -lt $taskSizes.Count;$taskIndex++) {
        $taskDimension=if($taskSizes[$taskIndex] -eq 256){0}else{$taskSizes[$taskIndex]}
        $taskWriter.Write([byte]$taskDimension);$taskWriter.Write([byte]$taskDimension)
        $taskWriter.Write([byte]0);$taskWriter.Write([byte]0)
        $taskWriter.Write([ushort]1);$taskWriter.Write([ushort]32)
        $taskWriter.Write([uint]$taskFrames[$taskIndex].Length);$taskWriter.Write([uint]$taskOffset)
        $taskOffset+=$taskFrames[$taskIndex].Length
    }
    foreach($taskFrame in $taskFrames){$taskWriter.Write([byte[]]$taskFrame)}
    [IO.File]::WriteAllBytes((Join-Path $PSScriptRoot 'QuickRemoteToolkit.ico'),$taskOutput.ToArray())
} finally {$taskWriter.Dispose();$taskOutput.Dispose()}
Write-Output "Generated ICO frames: $($taskSizes -join ', ')"
