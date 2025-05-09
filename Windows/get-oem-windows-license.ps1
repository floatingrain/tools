# 这段代码利用 WMI 提供的接口，直接从系统的许可管理服务中提取原始的 OEM 产品密钥。这种方法适用于预装 Windows 的设备，但如果系统是通过零售密钥或批量许可激活的，可能无法获取到相关密钥。
(Get-CimInstance -Query "SELECT * FROM SoftwareLicensingService").OA3xOriginalProductKey