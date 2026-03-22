[System.Reflection.Assembly]::LoadWithPartialName('System.Drawing') | Out-Null
$bmp = [System.Drawing.Bitmap]::FromFile('src-tauri\icons\icon.png')
$bmp.Save('src-tauri\icons\icon.ico', [System.Drawing.Imaging.ImageFormat]::Icon)
$bmp.Dispose()
