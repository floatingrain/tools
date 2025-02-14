# 获取所有隐藏设备
$hiddenDevices = Get-PnpDevice | Where-Object { $_.Status -eq "Unknown" -or $_.Status -eq "Error" }

# 获取所有驱动程序包列表（缓存到变量中）
$driverPackages = pnputil /enum-drivers

foreach ($device in $hiddenDevices) {
	# 获取设备的实例ID
	$instanceId = $device.InstanceId

	# 获取设备的驱动程序信息
	$driverInfo = Get-PnpDeviceProperty -InstanceId $instanceId -KeyName "DEVPKEY_Device_DriverInfPath"

	# 使用 pnputil 卸载设备
	Write-Host "Removing device: $($device.FriendlyName) ($instanceId)"
	pnputil /remove-device $instanceId

	if ($driverInfo.Data) {
		# 获取驱动程序的 INF 文件名
		$infFileName = [System.IO.Path]::GetFileName($driverInfo.Data)

		# 从缓存的驱动程序包列表中查找匹配的驱动程序
		$driverPackage = $driverPackages | Select-String -Pattern $infFileName

		if ($driverPackage) {
			# 提取 OEM 名称（例如：oem123.inf）
			$oemName = ($driverPackage -split ' ')[-1]

			# 使用 pnputil 删除驱动程序包
			Write-Host "Deleting driver package: $oemName"
			pnputil /delete-driver $oemName /force
		}
	}
}

Write-Host "Hidden devices and their drivers have been removed."