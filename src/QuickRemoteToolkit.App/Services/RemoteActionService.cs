using QuickRemoteToolkit.App.Models;
using System.Diagnostics;
using System.IO;

namespace QuickRemoteToolkit.App.Services;

public sealed class RemoteActionService
{
    public async Task SendMessageAsync(ClientEntry client, string message, int displaySeconds)
    {
        if (string.IsNullOrWhiteSpace(client.Computer))
        {
            throw new ArgumentException("Не указано имя компьютера.");
        }

        if (string.IsNullOrWhiteSpace(message) || message.Contains('\0'))
        {
            throw new ArgumentException("Введите текст сообщения без нулевых символов.");
        }

        if (displaySeconds is < 1 or > 3600)
        {
            throw new ArgumentOutOfRangeException(nameof(displaySeconds), "Время показа: от 1 до 3600 секунд.");
        }

        var startInfo = new ProcessStartInfo
        {
            FileName = Path.Combine(Environment.SystemDirectory, "msg.exe"),
            UseShellExecute = false,
            CreateNoWindow = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true
        };
        startInfo.ArgumentList.Add("*");
        startInfo.ArgumentList.Add($"/server:{client.Computer}");
        startInfo.ArgumentList.Add($"/time:{displaySeconds}");
        startInfo.ArgumentList.Add(message);

        using var process = Process.Start(startInfo)
            ?? throw new InvalidOperationException("Не удалось запустить msg.exe.");
        var outputTask = process.StandardOutput.ReadToEndAsync();
        var errorTask = process.StandardError.ReadToEndAsync();
        using var timeout = new CancellationTokenSource(TimeSpan.FromSeconds(30));
        try
        {
            await process.WaitForExitAsync(timeout.Token);
        }
        catch (OperationCanceledException)
        {
            if (!process.HasExited)
            {
                process.Kill(entireProcessTree: true);
            }

            await process.WaitForExitAsync();
            await Task.WhenAll(outputTask, errorTask);
            throw new TimeoutException("msg.exe не завершилась за 30 секунд. Доставка сообщения не подтверждена.");
        }

        var output = (await outputTask).Trim();
        var error = (await errorTask).Trim();
        if (process.ExitCode != 0)
        {
            var details = string.IsNullOrWhiteSpace(error) ? output : error;
            throw new InvalidOperationException(
                $"msg.exe завершилась с кодом {process.ExitCode}. {details}\nПроверьте доступность ПК, наличие сеанса пользователя и права отправки сообщений.");
        }
    }

    public void OpenRemoteAssistance(ClientEntry client)
    {
        Start("msra.exe", $"/offerra \"{client.Computer}\"");
    }

    public void OpenTracert(ClientEntry client)
    {
        Start(Environment.GetEnvironmentVariable("ComSpec") ?? "cmd.exe", $"/d /c \"tracert {client.Target} & echo. & pause\"");
    }

    public string OpenAdminShare(ClientEntry client, string adminUserName)
    {
        var smbSessionResult = EnsureSmbSession(client, adminUserName);
        Start("explorer.exe", $@"\\{client.Computer}\c$");
        return smbSessionResult;
    }

    public void OpenEventViewer(ClientEntry client)
    {
        Start("eventvwr.msc", $"/computer:{client.Computer}");
    }

    public void OpenComputerManagement(ClientEntry client)
    {
        Start("compmgmt.msc", $@"/computer:\\{client.Computer}");
    }

    public void OpenMstsc(ClientEntry client)
    {
        Start("mstsc.exe", $"/v:{client.Computer}");
    }

    public void OpenWinRsCmd(ClientEntry client)
    {
        Start(Environment.GetEnvironmentVariable("ComSpec") ?? "cmd.exe", $"/d /k \"winrs -r:{client.Computer} cmd\"");
    }

    public void RunGpupdate(ClientEntry client)
    {
        Start(Environment.GetEnvironmentVariable("ComSpec") ?? "cmd.exe", $"/d /c \"winrs -r:{client.Computer} gpupdate /force & echo. & pause\"");
    }

    private static void Start(string fileName, string arguments)
    {
        Process.Start(new ProcessStartInfo
        {
            FileName = fileName,
            Arguments = arguments,
            UseShellExecute = true,
            WorkingDirectory = Directory.GetCurrentDirectory()
        });
    }

    private static string EnsureSmbSession(ClientEntry client, string adminUserName)
    {
        if (string.IsNullOrWhiteSpace(adminUserName))
        {
            return "SMB session skipped: user is empty.";
        }

        var startInfo = new ProcessStartInfo
        {
            FileName = "net.exe",
            CreateNoWindow = true,
            RedirectStandardInput = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            WorkingDirectory = Directory.GetCurrentDirectory()
        };
        startInfo.ArgumentList.Add("use");
        startInfo.ArgumentList.Add($@"\\{client.Computer}\IPC$");
        startInfo.ArgumentList.Add($"/user:{adminUserName}");

        using var process = Process.Start(startInfo);

        if (process is null)
        {
            return "SMB session skipped: net.exe did not start.";
        }

        process.StandardInput.WriteLine();

        if (!process.WaitForExit(5000))
        {
            process.Kill(entireProcessTree: true);
            return "SMB session timeout.";
        }

        var output = process.StandardOutput.ReadToEnd().Trim();
        var error = process.StandardError.ReadToEnd().Trim();
        if (process.ExitCode == 0)
        {
            return "SMB session prepared.";
        }

        var message = string.IsNullOrWhiteSpace(error) ? output : error;
        return string.IsNullOrWhiteSpace(message)
            ? $"SMB session failed: exit code {process.ExitCode}."
            : $"SMB session failed: {message}";
    }

}
