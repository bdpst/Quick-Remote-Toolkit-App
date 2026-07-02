namespace QuickRemoteToolkit.App.Models;

public sealed record LogEntry(DateTime Time, string Computer, string Action, string Result)
{
    public string TimeText => Time.ToString("dd.MM.yyyy HH:mm:ss");
}
