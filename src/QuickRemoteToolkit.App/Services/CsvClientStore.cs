using QuickRemoteToolkit.App.Models;
using System.IO;
using System.Text;

namespace QuickRemoteToolkit.App.Services;

public sealed class CsvClientStore
{
    public IReadOnlyList<ClientEntry> Load(string path)
    {
        var lines = ReadLines(path);
        var clients = new List<ClientEntry>();

        foreach (var line in lines.Skip(1))
        {
            if (string.IsNullOrWhiteSpace(line))
            {
                continue;
            }

            var parts = line.Split(';');
            if (parts.Length < 3)
            {
                continue;
            }

            clients.Add(new ClientEntry
            {
                Number = int.TryParse(parts[0], out var number) ? number : clients.Count + 1,
                Computer = parts[1].Trim(),
                Ip = parts[2].Trim(),
                Person = parts.Length > 3 ? parts[3].Trim() : ""
            });
        }

        return clients;
    }

    private static string[] ReadLines(string path)
    {
        var bytes = File.ReadAllBytes(path);
        var utf8Strict = new UTF8Encoding(false, true);

        try
        {
            return utf8Strict.GetString(bytes).Split(["\r\n", "\n"], StringSplitOptions.None);
        }
        catch (DecoderFallbackException)
        {
            return Encoding.GetEncoding(1251).GetString(bytes).Split(["\r\n", "\n"], StringSplitOptions.None);
        }
    }
}
