using System.Diagnostics;
using System.IO;
using System.Text;

namespace QuickRemoteToolkit.App.Services;

public sealed class LocalAdministratorsService
{
    internal static string CreateScript(string computer, string member, bool add)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(computer);
        ArgumentException.ThrowIfNullOrWhiteSpace(member);
        if (computer.Contains('\0') || member.Contains('\0'))
            throw new ArgumentException("Имя содержит нулевой символ.");
        static string Literal(string value) => "'" + value.Replace("'", "''") + "'";
        var command = add ? "Add-LocalGroupMember" : "Remove-LocalGroupMember";
        return $$"""
            $ErrorActionPreference = 'Stop'
            $ProgressPreference = 'SilentlyContinue'
            [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
            try {
                Invoke-Command -ComputerName {{Literal(computer)}} -ArgumentList {{Literal(member)}} -ErrorAction Stop -ScriptBlock {
                    param([string]$Member)
                    $ErrorActionPreference = 'Stop'
                    $Administrators = Get-LocalGroup -SID 'S-1-5-32-544' -ErrorAction Stop
                    {{command}} -Group $Administrators.Name -Member $Member -ErrorAction Stop
                }
                exit 0
            } catch {
                [Console]::Error.WriteLine($_.Exception.Message)
                exit 1
            }
            """;
    }

    public async Task ChangeAsync(string computer, string member, bool add)
    {
        var script = CreateScript(computer, member, add);
        var systemFolder = Environment.Is64BitOperatingSystem && !Environment.Is64BitProcess
            ? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.Windows), "Sysnative")
            : Environment.SystemDirectory;
        var startInfo = new ProcessStartInfo
        {
            FileName = Path.Combine(systemFolder, "WindowsPowerShell", "v1.0", "powershell.exe"),
            UseShellExecute = false,
            CreateNoWindow = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            StandardOutputEncoding = Encoding.UTF8,
            StandardErrorEncoding = Encoding.UTF8
        };
        foreach (var argument in new[] { "-NoProfile", "-NonInteractive", "-EncodedCommand", Convert.ToBase64String(Encoding.Unicode.GetBytes(script)) })
            startInfo.ArgumentList.Add(argument);
        using var process = Process.Start(startInfo) ?? throw new InvalidOperationException("Не удалось запустить PowerShell.");
        var output = process.StandardOutput.ReadToEndAsync();
        var error = process.StandardError.ReadToEndAsync();
        using var timeout = new CancellationTokenSource(TimeSpan.FromSeconds(60));
        try
        {
            await process.WaitForExitAsync(timeout.Token);
        }
        catch (OperationCanceledException)
        {
            if (!process.HasExited)
                process.Kill(entireProcessTree: true);
            await process.WaitForExitAsync();
            await Task.WhenAll(output, error);
            throw new TimeoutException("Ожидание превысило 60 секунд. Результат операции неизвестен: проверьте состав группы на ПК перед повтором.");
        }
        var details = (await error).Trim();
        var standardOutput = (await output).Trim();
        if (process.ExitCode != 0)
            throw new InvalidOperationException(string.IsNullOrWhiteSpace(details)
                ? $"PowerShell завершилась с кодом {process.ExitCode}. {standardOutput}"
                : details);
    }
}
