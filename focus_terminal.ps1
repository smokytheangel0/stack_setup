Add-Type @"
  using System;
  using System.Runtime.InteropServices;
  public class SFW {
     [DllImport("user32.dll")]
     [return: MarshalAs(UnmanagedType.Bool)]
     public static extern bool SetForegroundWindow(IntPtr hWnd);
  }
"@
$count = 0
do {
  $windowHandle = (get-process -name powershell)[$count].MainWindowHandle
  $count = $count + 1
} while ($windowHandle -eq 0 -or !$windowHandle)
echo $windowHandle
[SFW]::SetForegroundWindow($windowHandle)