using QuickRemoteToolkit.App.Models;
using System.IO;
using System.Text.Json;

namespace QuickRemoteToolkit.App.Services;

public sealed class SettingsService
{
    private readonly string _settingsDirectory = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
        "QuickRemoteToolkitApp");

    private string SettingsPath => Path.Combine(_settingsDirectory, "settings.json");

    public AppSettings Load()
    {
        Directory.CreateDirectory(_settingsDirectory);

        if (!File.Exists(SettingsPath))
        {
            var settings = new AppSettings
            {
                ClientsCsvPath = TryFindDefaultCsv()
            };
            Save(settings);
            return settings;
        }

        var json = File.ReadAllText(SettingsPath);
        return JsonSerializer.Deserialize<AppSettings>(json) ?? new AppSettings();
    }

    public void Save(AppSettings settings)
    {
        Directory.CreateDirectory(_settingsDirectory);
        var json = JsonSerializer.Serialize(settings, new JsonSerializerOptions { WriteIndented = true });
        File.WriteAllText(SettingsPath, json);
    }

    private static string TryFindDefaultCsv()
    {
        var documents = Environment.GetFolderPath(Environment.SpecialFolder.MyDocuments);
        var candidate = Path.Combine(documents, "GitHub", "Quick-Remote-Toolkit", "QuickRemoteToolkit.clients.csv");
        return File.Exists(candidate) ? candidate : "";
    }
}
