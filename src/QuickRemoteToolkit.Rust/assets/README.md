# Application icon

`QuickRemoteToolkit.svg` is the editable source: a white monitor with a settings gear on a #2563EB tile. The solid silhouette replaces the old thin, small
computer drawing so it remains readable at 16 pixels.

`QuickRemoteToolkit.ico` contains individually rendered 16, 20, 24, 32, 40, 48,
64, 128 and 256 pixel PNG frames. Cargo embeds the ICO in the executable through
Windows SDK rc.exe; the same artwork supplies the native window/taskbar icon
and custom title bar. The large toolbar tile uses the same texture.
At runtime both tiles and the native window icon use the active theme accent
(#2563EB light, #60A5FA dark), preserving the white artwork. The preserved WPF reference icon is not changed.

To regenerate on Windows:

```powershell
pwsh -STA -NoProfile -File ./src/QuickRemoteToolkit.Rust/assets/generate-icon.ps1
```

The optional design-time generator uses Windows WPF geometry/PNG encoding only
to convert SVG paths to ICO. It is not run by Cargo and adds no .NET dependency
to the application or its portable package.
