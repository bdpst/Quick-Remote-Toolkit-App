# Application icon

Editable source: QuickRemoteToolkit.svg, white monitor with gear on a #2563EB tile.
QuickRemoteToolkit.ico contains 16,20,24,32,40,48,64,128,256 pixel PNG frames.
Cargo embeds it with Windows SDK rc.exe. Window/taskbar, custom titlebar and
large toolbar use the same artwork; runtime accent is #2563EB light / #60A5FA dark.
The committed ICO is a build input; no external converter or managed runtime is needed.
When editing artwork, regenerate all frames with a suitable SVG-to-ICO design tool
and inspect 16px readability and matching runtime icons.
