using System.ComponentModel;
using System.Runtime.InteropServices;

namespace QuickRemoteToolkit.App.Services;

public sealed class RemoteMessageService
{
    public Task SendAsync(string computer, string message, int seconds, IProgress<string> progress)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(computer);
        ArgumentException.ThrowIfNullOrWhiteSpace(message);
        if (message.Contains('\0'))
            throw new ArgumentException("Текст содержит нулевой символ.", nameof(message));
        if (seconds is < 1 or > 3600)
            throw new ArgumentOutOfRangeException(nameof(seconds));

        return Task.Run(async () =>
        {
            var server = WTSOpenServerW(computer);
            if (server == nint.Zero)
                throw NativeError("Не удалось подключиться к ПК");
            try
            {
                var sessions = GetSessions(server);
                if (sessions.Count == 0)
                    throw new InvalidOperationException("На ПК не найдено пользовательских сеансов.");

                // Send to every user session concurrently, so one user's wait does not delay another.
                await Task.WhenAll(sessions.Select(session => Task.Run(() =>
                {
                    var recipient = $"{session.User} (сеанс {session.Id})";
                    progress.Report($"{recipient}: отправка и ожидание ответа, до {seconds} сек.");
                    const string title = "Сообщение от службы поддержки";
                    // OK/Cancel gives the recipient an explicit way to dismiss without acknowledging.
                    const uint style = 0x00000001 | 0x00000040; // MB_OKCANCEL | MB_ICONINFORMATION
                    if (!WTSSendMessageW(server, session.Id, title, title.Length * 2,
                            message, message.Length * 2, style, seconds, out var response, true))
                    {
                        progress.Report($"{recipient}: {NativeError("Ошибка отправки").Message}");
                        return;
                    }

                    var result = response switch
                    {
                        1 => "Пользователь нажал ОК.",
                        2 => "Пользователь закрыл сообщение без подтверждения (Отмена).",
                        32000 => "Время ожидания истекло.",
                        _ => $"Получен ответ Windows: {response}."
                    };
                    progress.Report($"{recipient}: {result}");
                })));
            }
            finally
            {
                WTSCloseServer(server);
            }
        });
    }

    private static List<(int Id, string User)> GetSessions(nint server)
    {
        if (!WTSEnumerateSessionsW(server, 0, 1, out var buffer, out var count))
            throw NativeError("Не удалось получить сеансы пользователей");
        try
        {
            var sessions = new List<(int, string)>();
            var size = Marshal.SizeOf<SessionInfo>();
            for (var i = 0; i < count; i++)
            {
                var session = Marshal.PtrToStructure<SessionInfo>(buffer + i * size);
                // Include active and disconnected user sessions, excluding listeners and services.
                if (session.State is not (0 or 4))
                    continue;
                if (!WTSQuerySessionInformationW(server, session.Id, 5, out var userBuffer, out _))
                    throw NativeError($"Не удалось определить пользователя сеанса {session.Id}");
                try
                {
                    var user = Marshal.PtrToStringUni(userBuffer);
                    if (!string.IsNullOrWhiteSpace(user))
                        sessions.Add((session.Id, user));
                }
                finally
                {
                    WTSFreeMemory(userBuffer);
                }
            }
            return sessions;
        }
        finally
        {
            WTSFreeMemory(buffer);
        }
    }

    private static Win32Exception NativeError(string context)
    {
        var code = Marshal.GetLastWin32Error();
        return new Win32Exception(code, $"{context}: {new Win32Exception(code).Message} (код {code}).");
    }

    [StructLayout(LayoutKind.Sequential)]
    private struct SessionInfo
    {
        public int Id;
        public nint StationName;
        public int State;
    }

    [DllImport("wtsapi32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
    private static extern nint WTSOpenServerW(string serverName);

    [DllImport("wtsapi32.dll", ExactSpelling = true)]
    private static extern void WTSCloseServer(nint server);

    [DllImport("wtsapi32.dll", ExactSpelling = true)]
    private static extern void WTSFreeMemory(nint memory);

    [DllImport("wtsapi32.dll", ExactSpelling = true, SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool WTSEnumerateSessionsW(nint server, int reserved, int version, out nint sessions, out int count);

    [DllImport("wtsapi32.dll", ExactSpelling = true, SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool WTSQuerySessionInformationW(nint server, int sessionId, int infoClass, out nint buffer, out int bytes);

    [DllImport("wtsapi32.dll", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool WTSSendMessageW(nint server, int sessionId, string title, int titleLength,
        string message, int messageLength, uint style, int timeout, out int response,
        [MarshalAs(UnmanagedType.Bool)] bool wait);
}
