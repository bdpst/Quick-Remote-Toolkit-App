using Microsoft.Win32;
using QuickRemoteToolkit.App.Models;
using QuickRemoteToolkit.App.Services;
using System.Collections.ObjectModel;
using System.ComponentModel;
using System.Diagnostics;
using System.IO;
using System.Net.NetworkInformation;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;

namespace QuickRemoteToolkit.App;

public partial class MainWindow : Window
{
    private readonly ObservableCollection<ClientEntry> _clients = [];
    private readonly ObservableCollection<LogEntry> _logs = [];
    private readonly SettingsService _settingsService = new();
    private readonly CsvClientStore _clientStore = new();
    private readonly RemoteActionService _actions = new();
    private readonly AppSettings _settings;
    private readonly ICollectionView _clientsView;

    public ICommand FocusSearchCommand { get; }

    public MainWindow()
    {
        FocusSearchCommand = new RelayCommand(FocusSearch);
        InitializeComponent();
        SetWindowIcon();

        _settings = _settingsService.Load();
        ClientsGrid.ItemsSource = _clients;
        LogGrid.ItemsSource = _logs;
        _clientsView = CollectionViewSource.GetDefaultView(_clients);
        _clientsView.Filter = FilterClient;

        CsvPathText.Text = _settings.ClientsCsvPath;
        LoadClients();
    }

    private ClientEntry? SelectedClient => ClientsGrid.SelectedItem as ClientEntry;
    private static string CurrentAdminUserName => $@"{Environment.UserDomainName}\{Environment.UserName}";

    private void SetWindowIcon()
    {
        var iconPath = Path.Combine(AppContext.BaseDirectory, "Assets", "QuickRemoteToolkit.ico");
        if (!File.Exists(iconPath))
        {
            return;
        }

        Icon = BitmapFrame.Create(new Uri(iconPath, UriKind.Absolute));
    }

    private void FocusSearch()
    {
        SearchBox.Focus();
        SearchBox.SelectAll();
    }

    private bool FilterClient(object item)
    {
        if (item is not ClientEntry client)
        {
            return false;
        }

        var query = SearchBox.Text.Trim();
        if (string.IsNullOrWhiteSpace(query))
        {
            return true;
        }

        return client.Computer.Contains(query, StringComparison.OrdinalIgnoreCase)
            || client.Ip.Contains(query, StringComparison.OrdinalIgnoreCase)
            || client.Person.Contains(query, StringComparison.OrdinalIgnoreCase);
    }

    private void LoadClients()
    {
        _clients.Clear();

        if (string.IsNullOrWhiteSpace(_settings.ClientsCsvPath) || !File.Exists(_settings.ClientsCsvPath))
        {
            CsvPathText.Text = "CSV не выбран или файл не найден.";
            CsvInfoText.Text = "";
            AddLog("", "Load CSV", "CSV не выбран или файл не найден.");
            return;
        }

        try
        {
            foreach (var client in _clientStore.Load(_settings.ClientsCsvPath))
            {
                _clients.Add(client);
            }

            AddLog("", "Load CSV", $"Загружено клиентов: {_clients.Count}");
            CsvPathText.Text = _settings.ClientsCsvPath;
            CsvInfoText.Text = $"Загружено: {DateTime.Now:dd.MM.yyyy HH:mm}   Клиентов: {_clients.Count}";
        }
        catch (Exception ex)
        {
            AddLog("", "Load CSV", ex.Message);
            MessageBox.Show(ex.Message, "Ошибка чтения CSV", MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    private void AddLog(string computer, string action, string result)
    {
        _logs.Insert(0, new LogEntry(DateTime.Now, computer, action, result));
    }

    private void RunForSelected(string actionName, Action<ClientEntry> action)
    {
        var client = SelectedClient;
        if (client is null)
        {
            MessageBox.Show("Выберите клиента.", "Quick Remote Toolkit", MessageBoxButton.OK, MessageBoxImage.Information);
            return;
        }

        try
        {
            action(client);
            AddLog(client.Computer, actionName, "Запущено.");
        }
        catch (Exception ex)
        {
            AddLog(client.Computer, actionName, ex.Message);
            MessageBox.Show(ex.Message, actionName, MessageBoxButton.OK, MessageBoxImage.Error);
        }
    }

    private async Task PingClientAsync(ClientEntry client)
    {
        client.Status = ClientStatus.Checking;
        client.LastChecked = DateTime.Now;

        try
        {
            using var ping = new Ping();
            var reply = await ping.SendPingAsync(client.Target, _settings.PingTimeoutMs);
            client.Status = reply.Status == IPStatus.Success ? ClientStatus.Online : ClientStatus.Offline;
            AddLog(client.Computer, "Ping", reply.Status == IPStatus.Success ? $"{reply.RoundtripTime} ms" : reply.Status.ToString());
        }
        catch (Exception ex)
        {
            client.Status = ClientStatus.Offline;
            AddLog(client.Computer, "Ping", ex.Message);
        }
    }

    private void ChooseCsv_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new OpenFileDialog
        {
            Filter = "CSV files (*.csv)|*.csv|All files (*.*)|*.*",
            Title = "Выберите CSV клиентов"
        };

        if (dialog.ShowDialog(this) != true)
        {
            return;
        }

        _settings.ClientsCsvPath = dialog.FileName;
        _settingsService.Save(_settings);
        CsvPathText.Text = _settings.ClientsCsvPath;
        LoadClients();
    }

    private void ReloadClients_Click(object sender, RoutedEventArgs e) => LoadClients();

    private void SearchBox_TextChanged(object sender, System.Windows.Controls.TextChangedEventArgs e) => _clientsView.Refresh();

    private void ClientsGrid_SelectionChanged(object sender, System.Windows.Controls.SelectionChangedEventArgs e)
    {
    }

    private void ClientsGrid_MouseDoubleClick(object sender, System.Windows.Input.MouseButtonEventArgs e)
    {
        RunForSelected("Remote Assistance", _actions.OpenRemoteAssistance);
    }

    private void ClientsGrid_PreviewMouseRightButtonDown(object sender, System.Windows.Input.MouseButtonEventArgs e)
    {
        var row = FindParent<DataGridRow>((DependencyObject)e.OriginalSource);
        if (row is null)
        {
            return;
        }

        row.IsSelected = true;
        ClientsGrid.SelectedItem = row.Item;
        row.Focus();
    }

    private static T? FindParent<T>(DependencyObject child) where T : DependencyObject
    {
        var current = child;
        while (current is not null)
        {
            if (current is T target)
            {
                return target;
            }

            current = VisualTreeHelper.GetParent(current);
        }

        return null;
    }

    private void RemoteAssistance_Click(object sender, RoutedEventArgs e) => RunForSelected("Remote Assistance", _actions.OpenRemoteAssistance);
    private void Tracert_Click(object sender, RoutedEventArgs e) => RunForSelected("Tracert", _actions.OpenTracert);
    private void AdminShare_Click(object sender, RoutedEventArgs e)
    {
        RunForSelected("Open C$", client => _actions.OpenAdminShare(client, CurrentAdminUserName));
    }
    private void EventViewer_Click(object sender, RoutedEventArgs e) => RunForSelected("Event Viewer", _actions.OpenEventViewer);
    private void ComputerManagement_Click(object sender, RoutedEventArgs e) => RunForSelected("Computer Management", _actions.OpenComputerManagement);
    private void Mstsc_Click(object sender, RoutedEventArgs e) => RunForSelected("MSTSC", _actions.OpenMstsc);
    private void WinRsCmd_Click(object sender, RoutedEventArgs e) => RunForSelected("WinRS cmd", _actions.OpenWinRsCmd);
    private void Gpupdate_Click(object sender, RoutedEventArgs e) => RunForSelected("gpupdate", _actions.RunGpupdate);

    private async void PingSelected_Click(object sender, RoutedEventArgs e)
    {
        if (SelectedClient is null)
        {
            MessageBox.Show("Выберите клиента.", "Quick Remote Toolkit", MessageBoxButton.OK, MessageBoxImage.Information);
            return;
        }

        await PingClientAsync(SelectedClient);
    }

    private async void PingAll_Click(object sender, RoutedEventArgs e)
    {
        var semaphore = new SemaphoreSlim(_settings.ParallelPingLimit);
        var tasks = _clients.Select(async client =>
        {
            await semaphore.WaitAsync();
            try
            {
                await PingClientAsync(client);
            }
            finally
            {
                semaphore.Release();
            }
        });

        await Task.WhenAll(tasks);
    }

    private void CopyComputer_Click(object sender, RoutedEventArgs e)
    {
        RunForSelected("Copy computer", client => Clipboard.SetText(client.Computer));
    }

    private void CopyIp_Click(object sender, RoutedEventArgs e)
    {
        RunForSelected("Copy IP", client => Clipboard.SetText(client.Target));
    }

    private void ExportLogs_Click(object sender, RoutedEventArgs e)
    {
        var dialog = new SaveFileDialog
        {
            Filter = "Text files (*.txt)|*.txt|CSV files (*.csv)|*.csv|All files (*.*)|*.*",
            DefaultExt = "txt",
            FilterIndex = 1,
            FileName = $"quick-remote-toolkit-log-{DateTime.Now:yyyyMMdd-HHmmss}.txt"
        };

        if (dialog.ShowDialog(this) != true)
        {
            return;
        }

        var isCsv = string.Equals(Path.GetExtension(dialog.FileName), ".csv", StringComparison.OrdinalIgnoreCase);
        var lines = isCsv
            ? _logs.Select(log => $"{log.Time:yyyy-MM-dd HH:mm:ss};{log.Computer};{log.Action};{log.Result}")
            : _logs.Select(log => $"[{log.Time:yyyy-MM-dd HH:mm:ss}] {log.Computer} | {log.Action} | {log.Result}");

        File.WriteAllLines(dialog.FileName, lines);
        AddLog("", "Export logs", dialog.FileName);
    }

    private sealed class RelayCommand(Action execute) : ICommand
    {
        public event EventHandler? CanExecuteChanged
        {
            add { }
            remove { }
        }

        public bool CanExecute(object? parameter) => true;

        public void Execute(object? parameter) => execute();
    }
}
