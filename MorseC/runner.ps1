# profile_morse.ps1

# Set this to your ARM toolchain bin directory
$ARM_TOOLCHAIN = "C:\VSARM\armcc\10 2021.10\bin"

Write-Host "`n==== FLASH SIZE (MAIN) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-size.exe" build/transmitter.elf

Write-Host "`n==== MEMORY SECTIONS (MAIN) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-size.exe" -A build/transmitter.elf | Select-String -Pattern "\.text|\.data|\.bss|\.rodata"

Write-Host "`n==== LARGEST FUNCTIONS (MAIN) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-nm.exe" --print-size --size-sort build/transmitter.elf | Select-String -Pattern " [tT] " | Select-Object -Last 10

Write-Host "`n==== LARGEST STATIC VARIABLES (MAIN) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-nm.exe" --print-size --size-sort build/transmitter.elf | Select-String -Pattern " [bBdD] " | Select-Object -Last 10

Write-Host "`n==== BINARY INFO (MAIN) ===="
& "$ARM_TOOLCHAIN\arm-none-eabi-readelf.exe" -h build/transmitter.elf | Select-String -Pattern "Magic|Class|Data|OS|ABI|Type|Machine|Entry|Flags"

Write-Host "`n==== DETAILED MEMORY SUMMARY (MAIN) ===="
Write-Host "Static data breakdown:"
$textContent = & "$ARM_TOOLCHAIN\arm-none-eabi-size.exe" -A build/transmitter.elf | Out-String

$textSize = if ($textContent -match "\.text\s+(\d+)") { [int]$Matches[1] } else { 0 }
$rodataSize = if ($textContent -match "\.rodata\s+(\d+)") { [int]$Matches[1] } else { 0 }
$dataSize = if ($textContent -match "\.data\s+(\d+)") { [int]$Matches[1] } else { 0 }
$bssSize = if ($textContent -match "\.bss\s+(\d+)") { [int]$Matches[1] } else { 0 }

Write-Host ("{0,-15} {1,8:N0} bytes (code)" -f ".text", $textSize)
Write-Host ("{0,-15} {1,8:N0} bytes (read-only data)" -f ".rodata", $rodataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (initialized data)" -f ".data", $dataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (uninitialized data)" -f ".bss", $bssSize)

# Note: all code and read-only data go to flash
# The .text section in your linker script already includes .rodata
# .data is counted separately as it goes in both flash and RAM
# .boot2 and .vector_table should be included in flash total
$flashTotal = $textSize + $bootSize + $vectorSize + $dataSize

# Note: .data, .bss are actual RAM usage (stack is pre-allocated but not "used")
$RamTotal = $dataSize + $bssSize

Write-Host ("{0,-15} {1,8:N0} bytes" -f "FLASH TOTAL", $flashTotal)
Write-Host ("{0,-15} {1,8:N0} bytes" -f "RAM TOTAL", $ramTotal)


Write-Host "`n==== TOTAL RESOURCE USAGE ===="
Write-Host "Transmitter:"
Write-Host "  Flash usage: $flashTotal bytes of 2,048 KB ($('{0:F2}' -f ($flashTotal / 2097152 * 100))%)"
Write-Host "  Static RAM: $ramTotal bytes"
Write-Host "  Estimated stack: 2,048 bytes"
Write-Host "  Total RAM: $(2048 + $ramTotal) bytes of 264 KB ($('{0:F2}' -f ((2048 + $ramTotal) / 270336 * 100))%)"

