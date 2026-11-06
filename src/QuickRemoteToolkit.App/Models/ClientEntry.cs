using System.ComponentModel;
using System.Runtime.CompilerServices;

namespace QuickRemoteToolkit.App.Models;

public sealed class ClientEntry : INotifyPropertyChanged
{
    private ClientStatus _status = ClientStatus.Unknown;
    private DateTime? _lastChecked;
    private string _domain = "—";

    private bool _isFavorite;
    public string FavoriteKey => !string.IsNullOrWhiteSpace(Computer) && Computer.Trim() != "-"
        ? "computer:" + Computer.Trim().ToUpperInvariant()
        : !string.IsNullOrWhiteSpace(Ip) && Ip.Trim() != "-"
            ? "ip:" + Ip.Trim().ToUpperInvariant() : "";
    public bool CanFavorite => FavoriteKey.Length > 0;
    public bool IsFavorite
    {
        get => _isFavorite;
        set
        {
            if (_isFavorite == value) return;
            _isFavorite = value;
            OnPropertyChanged();
        }
    }

    public int Number { get; init; }
    public string Computer { get; init; } = "";
    public string Ip { get; init; } = "";
    public string Person { get; init; } = "";
    public string Target => string.IsNullOrWhiteSpace(Ip) || Ip == "-" ? Computer : Ip;

    public string Domain
    {
        get => _domain;
        set
        {
            if (_domain == value)
            {
                return;
            }

            _domain = value;
            OnPropertyChanged();
        }
    }

    public ClientStatus Status
    {
        get => _status;
        set
        {
            if (_status == value)
            {
                return;
            }

            _status = value;
            OnPropertyChanged();
            OnPropertyChanged(nameof(StatusText));
            OnPropertyChanged(nameof(StatusBrush));
        }
    }

    public DateTime? LastChecked
    {
        get => _lastChecked;
        set
        {
            if (_lastChecked == value)
            {
                return;
            }

            _lastChecked = value;
            OnPropertyChanged();
            OnPropertyChanged(nameof(LastCheckedText));
        }
    }

    public string StatusText => Status switch
    {
        ClientStatus.Online => "Доступен",
        ClientStatus.Offline => "Не отвечает",
        ClientStatus.Checking => "Проверка",
        _ => "Неизвестно"
    };

    public string StatusBrush => Status switch
    {
        ClientStatus.Online => "#16A34A",
        ClientStatus.Offline => "#DC2626",
        ClientStatus.Checking => "#2563EB",
        _ => "#8A94A3"
    };

    public string LastCheckedText => LastChecked?.ToString("dd.MM.yyyy HH:mm:ss") ?? "—";

    public event PropertyChangedEventHandler? PropertyChanged;

    private void OnPropertyChanged([CallerMemberName] string? propertyName = null)
    {
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
    }
}
