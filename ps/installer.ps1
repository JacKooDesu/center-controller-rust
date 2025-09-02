$bitness = if ([System.Runtime.InteropServices.RuntimeInformation, mscorlib]::ProcessArchitecture -eq "X64") {
    "x86_64"
}
elseif ([System.Runtime.InteropServices.RuntimeInformation, mscorlib]::ProcessArchitecture -eq "Arm64") {
    "aarch64"
}
else {
    "i686"
}
# https://github.com/JacKooDesu/center-controller-rust/releases/download/v1.0.0/center-controller-rust.exe
$url = "https://github.com/JacKooDesu/center-controller-rust/releases/latest/download/$($bitness)_center-controller-rust.exe"
$outFile = "$Env:UserProfile\Desktop\center-controller-rust.exe"

Write-Output "$($PSStyle.Bold)$($PSStyle.Foreground.Green)[@]$($PSStyle.Reset) downloading center controller rust..."

$oldProgressPreference = $ProgressPreference
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri $url -OutFile $outFile
$ProgressPreference = $oldProgressPreference

Start-Process -FilePath $outFile
