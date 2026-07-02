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

    public void OpenAdminShare(ClientEntry client)
    {
        Start("explorer.exe", $@"\\{client.Computer}\c$");
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
}
