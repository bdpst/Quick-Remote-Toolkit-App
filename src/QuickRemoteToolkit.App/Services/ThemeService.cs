using System.Windows;
using System.Windows.Media;

namespace QuickRemoteToolkit.App.Services;

public static class ThemeService
{
    private static readonly IReadOnlyDictionary<string, string> LightPalette =
        new Dictionary<string, string>
        {
            ["WindowBackgroundBrush"] = "#F5F7FB",
            ["CardBackgroundBrush"] = "#FFFFFF",
            ["CardMutedBrush"] = "#F8FAFD",
            ["TextPrimaryBrush"] = "#0F172A",
            ["TextSecondaryBrush"] = "#475569",
            ["TextMutedBrush"] = "#7C8798",
            ["BorderBrushSoft"] = "#D8E0EC",
            ["DividerBrush"] = "#E5EAF3",
            ["AccentBrush"] = "#2563EB",
            ["AccentHoverBrush"] = "#1D4ED8",
            ["AccentSoftBrush"] = "#E8F1FF",
            ["SelectedRowBrush"] = "#DDEBFF",
            ["AlternatingRowBrush"] = "#FBFCFE",
            ["ButtonHoverBrush"] = "#F1F6FF",
            ["ButtonHoverBorderBrush"] = "#BFD0EA",
            ["ButtonPressedBrush"] = "#E8F1FF",
            ["PrimaryPressedBrush"] = "#1E40AF",
            ["HeaderHoverBrush"] = "#F1F5FB"
        };

    private static readonly IReadOnlyDictionary<string, string> DarkPalette =
        new Dictionary<string, string>
        {
            ["WindowBackgroundBrush"] = "#111827",
            ["CardBackgroundBrush"] = "#1F2937",
            ["CardMutedBrush"] = "#273449",
            ["TextPrimaryBrush"] = "#F1F5F9",
            ["TextSecondaryBrush"] = "#CBD5E1",
            ["TextMutedBrush"] = "#94A3B8",
            ["BorderBrushSoft"] = "#475569",
            ["DividerBrush"] = "#344256",
            ["AccentBrush"] = "#60A5FA",
            ["AccentHoverBrush"] = "#3B82F6",
            ["AccentSoftBrush"] = "#1E3A5F",
            ["SelectedRowBrush"] = "#1E4976",
            ["AlternatingRowBrush"] = "#243044",
            ["ButtonHoverBrush"] = "#30415A",
            ["ButtonHoverBorderBrush"] = "#5B708D",
            ["ButtonPressedBrush"] = "#1E3A5F",
            ["PrimaryPressedBrush"] = "#1D4ED8",
            ["HeaderHoverBrush"] = "#30415A"
        };

    public static void Apply(bool isDarkTheme)
    {
        var palette = isDarkTheme ? DarkPalette : LightPalette;

        foreach (var (key, colorValue) in palette)
        {
            var color = (Color)ColorConverter.ConvertFromString(colorValue);
            Application.Current.Resources[key] = new SolidColorBrush(color);
        }
    }
}
