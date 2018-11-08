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
$len = (get-process -name setup).Length
if ($len -gt 1) {
  do {
    $windowHandle = (get-process -name setup)[$count].MainWindowHandle
    $count = $count + 1
    if ($count -gt $len) {
      Write-Output "there were $len processes associated with setup"
      break
    } else {
      continue
    }
  } while ($windowHandle -eq 0 -or !$windowHandle)  
} else {
  $windowHandle = (get-process -name setup).MainWindowHandle
  "there was only one process associated with setup"
}

$out = 0
do {
  $out = [SFW]::SetForegroundWindow($windowHandle)
  Write-Output "SetForegroundWindow returned $out"
  Start-Sleep -s 1
} while (!$out)