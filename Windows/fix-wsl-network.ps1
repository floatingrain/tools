# 修复 WSL mirrored 网络
wsl --shutdown
Get-HnsNetwork | Where-Object {$_.Name -like '*FSE*' -or $_.Name -like '*WSL*'} | Remove-HnsNetwork
Restart-Service hns
Write-Host "WSL mirrored networking reset complete"