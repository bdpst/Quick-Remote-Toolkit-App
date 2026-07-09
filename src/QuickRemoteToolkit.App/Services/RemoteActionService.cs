using QuickRemoteToolkit.App.Models;
using System.Diagnostics;
using System.IO;

namespace QuickRemoteToolkit.App.Services;

public sealed class RemoteActionService
{
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
