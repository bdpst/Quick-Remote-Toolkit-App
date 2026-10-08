using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;

namespace QuickRemoteToolkit.App;

public partial class SendMessageWindow : Window
{
    public string MessageTextValue { get; private set; } = "";
    public int DisplaySeconds { get; private set; } = 300;

    public SendMessageWindow(string computer)
    {
        InitializeComponent();
        RecipientText.Text = $"Сообщение на {computer}";
        Loaded += (_, _) => MessageText.Focus();
        PreviewKeyDown += (_, e) =>
        {
            if (e.Key == Key.Enter && Keyboard.Modifiers == ModifierKeys.Control)
            {
                Send_Click(this, new RoutedEventArgs());
                e.Handled = true;
            }
        };
    }

    private void MessageText_TextChanged(object sender, TextChangedEventArgs e)
    {
        if (SendButton is not null)
        {
            SendButton.IsEnabled = !string.IsNullOrWhiteSpace(MessageText.Text);
        }
    }

    private void Send_Click(object sender, RoutedEventArgs e)
    {
        if (string.IsNullOrWhiteSpace(MessageText.Text))
        {
            StatusText.Text = "Введите текст сообщения.";
            MessageText.Focus();
            return;
        }

        if (!int.TryParse(DisplaySecondsText.Text, out var seconds) || seconds is < 1 or > 3600)
        {
            StatusText.Text = "Укажите время показа от 1 до 3600 секунд.";
            DisplaySecondsText.Focus();
            return;
        }

        MessageTextValue = MessageText.Text.Trim();
        DisplaySeconds = seconds;
        DialogResult = true;
    }
}
