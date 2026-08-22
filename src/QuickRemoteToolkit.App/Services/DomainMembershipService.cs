using System.ComponentModel;
using System.Runtime.InteropServices;

namespace QuickRemoteToolkit.App.Services;

public sealed class DomainMembershipService
{
    private static readonly TimeSpan QueryTimeout = TimeSpan.FromSeconds(5);

    public async Task<DomainMembershipResult> GetAsync(string computer)
    {
        try
        {
            return await Task.Run(() => Get(computer)).WaitAsync(QueryTimeout);
        }
        catch (TimeoutException)
        {
            return new DomainMembershipResult("Нет ответа", "Превышено время ожидания запроса домена.");
        }
    }

    private static DomainMembershipResult Get(string computer)
    {
        nint nameBuffer = nint.Zero;

        try
        {
            var result = NetGetJoinInformation(computer, out nameBuffer, out var joinStatus);
            if (result != 0)
            {
                var message = new Win32Exception(result).Message;
                return new DomainMembershipResult("Ошибка запроса", message);
            }

            var name = nameBuffer == nint.Zero ? "" : Marshal.PtrToStringUni(nameBuffer) ?? "";
            return joinStatus switch
            {
                NetSetupJoinStatus.DomainName => new DomainMembershipResult(name, null),
                NetSetupJoinStatus.WorkgroupName => new DomainMembershipResult(
                    string.IsNullOrWhiteSpace(name) ? "Рабочая группа" : $"Рабочая группа: {name}",
                    null),
                NetSetupJoinStatus.Unjoined => new DomainMembershipResult("Не присоединён", null),
                _ => new DomainMembershipResult("Неизвестно", null)
            };
        }
        finally
        {
            if (nameBuffer != nint.Zero)
            {
                NetApiBufferFree(nameBuffer);
            }
        }
    }

    [DllImport("Netapi32.dll", CharSet = CharSet.Unicode)]
    private static extern int NetGetJoinInformation(
        string server,
        out nint nameBuffer,
        out NetSetupJoinStatus bufferType);

    [DllImport("Netapi32.dll")]
    private static extern int NetApiBufferFree(nint buffer);

    private enum NetSetupJoinStatus
    {
        UnknownStatus,
        Unjoined,
        WorkgroupName,
        DomainName
    }
}

public sealed record DomainMembershipResult(string DisplayText, string? ErrorMessage);
