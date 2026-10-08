using System.Windows;

namespace QuickRemoteToolkit.App;

public partial class LocalAdministratorsWindow : Window
{
    public string Member { get; private set; } = "";
    public bool AddMember { get; private set; }

    public LocalAdministratorsWindow(string computer)
    {
        InitializeComponent();
        RecipientText.Text = $"Локальные администраторы — {computer}";
        Loaded += (_, _) => MemberText.Focus();
    }

    private void Apply_Click(object sender, RoutedEventArgs e)
    {
        var member = MemberText.Text.Trim();
        var separator = member.IndexOf('\\');
        if (separator <= 0 || separator == member.Length - 1 || member.LastIndexOf('\\') != separator
            || member.Any(char.IsControl))
        {
            StatusText.Text = "Введите учётную запись в формате ДОМЕН\\логин или ИМЯ-ПК\\логин.";
            MemberText.Focus();
            return;
        }
        Member = member;
        AddMember = AddOption.IsChecked == true;
        DialogResult = true;
    }
}
