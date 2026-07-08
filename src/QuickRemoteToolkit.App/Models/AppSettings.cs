namespace QuickRemoteToolkit.App.Models;

public sealed class AppSettings
{
    public string ClientsCsvPath { get; set; } = "";
    public int PingTimeoutMs { get; set; } = 1000;
    public int ParallelPingLimit { get; set; } = 24;
}
