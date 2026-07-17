using System;
using System.Runtime.InteropServices;

class SecureMemory
{
    static void Main()
    {
        byte[] secretToken = { 0x41, 0x42, 0x43, 0x44 }; // Geheime Daten "ABCD"
        
        // Pinne das Array im RAM, um zu verhindern, dass der GC es verschiebt
        GCHandle handle = GCHandle.Alloc(secretToken, GCHandleType.Pinned);
        try
        {
            Console.WriteLine("[*] Sensible Operationen im RAM...");
            
            // Verwende das Geheimnis...
        }
        finally
        {
            // Sicheres Überschreiben (Zeroing) vor der Freigabe
            CryptographicZeroMemory(handle.AddrOfPinnedObject(), secretToken.Length);
            handle.Free();
            Console.WriteLine("[+] Speicher erfolgreich genullt (Zeroized).");
        }
    }

    [DllImport("kernel32.dll", EntryPoint = "RtlZeroMemory", SetLastError = false)]
    private static extern void CryptographicZeroMemory(IntPtr dest, int length);
}
