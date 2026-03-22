Add-Type -AssemblyName System.Drawing

# 创建 256x256 的蓝色背景图片
$bmp = New-Object System.Drawing.Bitmap(256, 256)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.Clear([System.Drawing.Color]::FromArgb(59, 130, 246))

# 保存为 PNG
$bmp.Save("src-tauri\icons\icon.png", [System.Drawing.Imaging.ImageFormat]::Png)

# 保存为 ICO（包含多个尺寸）
$iconSize = New-Object System.Drawing.Size(256, 256)
$iconBmp = New-Object System.Drawing.Bitmap($bmp, $iconSize)
$iconStream = New-Object System.IO.FileStream("src-tauri\icons\icon.ico", [System.IO.FileMode]::Create)
$iconBmp.Save($iconStream, [System.Drawing.Imaging.ImageFormat]::Icon)
$iconStream.Close()

# 创建其他需要的尺寸
$sizes = @("32x32", "128x128")
foreach ($size in $sizes) {
    $dimensions = $size.Split("x") | ForEach-Object { [int]$_ }
    $resized = New-Object System.Drawing.Bitmap($dimensions[0], $dimensions[1])
    $graphics = [System.Drawing.Graphics]::FromImage($resized)
    $graphics.Clear([System.Drawing.Color]::FromArgb(59, 130, 246))
    $graphics.Dispose()
    $resized.Save("src-tauri\icons\$size.png", [System.Drawing.Imaging.ImageFormat]::Png)
    $resized.Dispose()
}

# 创建 2x 版本
$resized2x = New-Object System.Drawing.Bitmap(256, 256)
$graphics2x = [System.Drawing.Graphics]::FromImage($resized2x)
$graphics2x.Clear([System.Drawing.Color]::FromArgb(59, 130, 246))
$graphics2x.Dispose()
$resized2x.Save("src-tauri\icons\128x128@2x.png", [System.Drawing.Imaging.ImageFormat]::Png)
$resized2x.Dispose()

# 创建 ICNS（简单复制 PNG，因为 ICNS 是 macOS 专用）
Copy-Item "src-tauri\icons\icon.png" "src-tauri\icons\icon.icns"

$bmp.Dispose()
$g.Dispose()
