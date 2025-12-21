# Convert PNG to ICO using .NET System.Drawing
param(
    [string]$InputPng = ".\assets\icon.png",
    [string]$OutputIco = ".\assets\icon.ico"
)

Add-Type -AssemblyName System.Drawing

# Load the PNG image
$png = [System.Drawing.Image]::FromFile((Resolve-Path $InputPng))

# Create icon sizes: 16, 32, 48, 64, 128, 256
$sizes = @(16, 32, 48, 64, 128, 256)

# Create a memory stream to hold the ICO data
$memoryStream = New-Object System.IO.MemoryStream

# ICO header
$bw = New-Object System.IO.BinaryWriter($memoryStream)
$bw.Write([UInt16]0)  # Reserved
$bw.Write([UInt16]1)  # Type: 1 for ICO
$bw.Write([UInt16]$sizes.Count)  # Number of images

# Calculate offset for first image
$offset = 6 + ($sizes.Count * 16)

# Array to store image data
$imageData = @()

foreach ($size in $sizes) {
    # Resize image
    $bitmap = New-Object System.Drawing.Bitmap($size, $size)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.DrawImage($png, 0, 0, $size, $size)
    $graphics.Dispose()
    
    # Save to PNG in memory
    $imgStream = New-Object System.IO.MemoryStream
    $bitmap.Save($imgStream, [System.Drawing.Imaging.ImageFormat]::Png)
    $imgBytes = $imgStream.ToArray()
    $imgStream.Dispose()
    $bitmap.Dispose()
    
    # Write directory entry
    $widthByte = if ($size -eq 256) { 0 } else { $size }
    $heightByte = if ($size -eq 256) { 0 } else { $size }
    $bw.Write([Byte]$widthByte)  # Width (0 means 256)
    $bw.Write([Byte]$heightByte)  # Height
    $bw.Write([Byte]0)  # Color palette
    $bw.Write([Byte]0)  # Reserved
    $bw.Write([UInt16]1)  # Color planes
    $bw.Write([UInt16]32)  # Bits per pixel
    $bw.Write([UInt32]$imgBytes.Length)  # Size
    $bw.Write([UInt32]$offset)  # Offset
    
    $imageData += $imgBytes
    $offset += $imgBytes.Length
}

# Write image data
foreach ($data in $imageData) {
    $bw.Write($data)
}

# Save to file
$bw.Flush()
$outputPath = if (Test-Path $OutputIco) { (Resolve-Path $OutputIco).Path } else { $OutputIco }
[System.IO.File]::WriteAllBytes($outputPath, $memoryStream.ToArray())

$bw.Close()
$memoryStream.Close()
$png.Dispose()

Write-Host "Icon created successfully: $OutputIco"
