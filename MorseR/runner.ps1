# Set target directory for Rust builds
$TARGET_DIR = "target\thumbv6m-none-eabi\release\build\MorseR-a373f093b25cbd75\out"
$TRANSMITTER_ELF = "$TARGET_DIR\transmitter.elf"

Write-Host "Analyzing existing transmitter.elf at $TRANSMITTER_ELF"

# Display binary info
Write-Host "`n==== BINARY INFO (TRANSMITTER) ===="
arm-none-eabi-readelf -h $TRANSMITTER_ELF | Select-String -Pattern "File|Type|Machine|Entry|Flags"

# Display and collect section sizes
Write-Host "`n==== SECTION SIZES (TRANSMITTER) ===="
arm-none-eabi-size $TRANSMITTER_ELF -A

# Extract memory section sizes
$sizeOutput = arm-none-eabi-size $TRANSMITTER_ELF -A

# Handle missing sections gracefully
$textMatch = $sizeOutput | Select-String -Pattern "\.text "
$textSize = if ($textMatch) { 
    $textMatch.ToString() -match "\.text\s+(\d+)" | Out-Null
    if ($Matches) { [int]$Matches[1] } else { 0 }
} else { 0 }

$rodataMatch = $sizeOutput | Select-String -Pattern "\.rodata"
$rodataSize = if ($rodataMatch) { 
    $rodataMatch.ToString() -match "\.rodata\s+(\d+)" | Out-Null
    if ($Matches) { [int]$Matches[1] } else { 0 }
} else { 0 }

$dataMatch = $sizeOutput | Select-String -Pattern "\.data "
$dataSize = if ($dataMatch) { 
    $dataMatch.ToString() -match "\.data\s+(\d+)" | Out-Null
    if ($Matches) { [int]$Matches[1] } else { 0 }
} else { 0 }

$bssMatch = $sizeOutput | Select-String -Pattern "\.bss "
$bssSize = if ($bssMatch) { 
    $bssMatch.ToString() -match "\.bss\s+(\d+)" | Out-Null
    if ($Matches) { [int]$Matches[1] } else { 0 }
} else { 0 }

# Display largest functions
Write-Host "`n==== LARGEST FUNCTIONS (TRANSMITTER) ===="
arm-none-eabi-nm $TRANSMITTER_ELF --print-size --size-sort | Select-String -Pattern " [tT] " | Select-Object -Last 10

# Display largest static variables
Write-Host "`n==== LARGEST STATIC VARIABLES (TRANSMITTER) ===="
arm-none-eabi-nm $TRANSMITTER_ELF --print-size --size-sort | Select-String -Pattern " [bBdD] " | Select-Object -Last 10

# Look for other named sections that might contain code or data
$bootMatch = $sizeOutput | Select-String -Pattern "\.boot2"
$bootSize = if ($bootMatch) {
    $bootMatch.ToString() -match "\.boot2\s+(\d+)" | Out-Null
    if ($Matches) { [int]$Matches[1] } else { 0 }
} else { 0 }

$vectorMatch = $sizeOutput | Select-String -Pattern "\.vector_table"
$vectorSize = if ($vectorMatch) {
    $vectorMatch.ToString() -match "\.vector_table\s+(\d+)" | Out-Null
    if ($Matches) { [int]$Matches[1] } else { 0 }
} else { 0 }

$stackMatch = $sizeOutput | Select-String -Pattern "\.stack"
$stackSize = if ($stackMatch) {
    $stackMatch.ToString() -match "\.stack\s+(\d+)" | Out-Null
    if ($Matches) { [int]$Matches[1] } else { 0 }
} else { 0 }

# Detailed memory summary
Write-Host "`n==== DETAILED MEMORY SUMMARY (TRANSMITTER) ===="
Write-Host "Static data breakdown:"
Write-Host ("{0,-15} {1,8:N0} bytes (code)" -f ".text", $textSize)
Write-Host ("{0,-15} {1,8:N0} bytes (code)" -f ".boot2", $bootSize)
Write-Host ("{0,-15} {1,8:N0} bytes (data)" -f ".vector_table", $vectorSize) 
Write-Host ("{0,-15} {1,8:N0} bytes (read-only data)" -f ".rodata", $rodataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (initialized data)" -f ".data", $dataSize)
Write-Host ("{0,-15} {1,8:N0} bytes (uninitialized data)" -f ".bss", $bssSize)
Write-Host ("{0,-15} {1,8:N0} bytes (pre-allocated)" -f ".stack", $stackSize)

# Note: all code and read-only data go to flash
$flashTotal = $textSize + $rodataSize + $dataSize + $bootSize + $vectorSize
# Note: .data, .bss are actual RAM usage (stack is pre-allocated but not "used")
$staticRamTotal = $dataSize + $bssSize
$totalRamWithStack = $staticRamTotal + $stackSize

Write-Host ("{0,-15} {1,8:N0} bytes" -f "FLASH TOTAL", $flashTotal)
Write-Host ("{0,-15} {1,8:N0} bytes" -f "STATIC RAM", $staticRamTotal)
Write-Host ("{0,-15} {1,8:N0} bytes" -f "RAM WITH STACK", $totalRamWithStack)

# Stack size estimation
Write-Host "`n==== STACK SIZE ESTIMATION ===="
Write-Host "Pre-allocated stack size: $stackSize bytes ($('{0:F2}' -f ($stackSize / 1024)) KB)"
Write-Host "Note: This is reserved space in the linker script (.stack section)"
Write-Host "      Actual stack usage at runtime will be between 0 and this maximum"
Write-Host "      The stack grows downward from the top of this reserved region"

# Total resource usage
Write-Host "`n==== TOTAL RESOURCE USAGE ===="
Write-Host "Transmitter:"
Write-Host "  Flash usage: $flashTotal bytes of 2,048 KB ($('{0:F2}' -f ($flashTotal / 2097152 * 100))%)"
Write-Host "  Static RAM usage: $staticRamTotal bytes of 264 KB ($('{0:F2}' -f ($staticRamTotal / 270336 * 100))%)"
Write-Host "  Reserved stack space: $stackSize bytes"
Write-Host "  Total RAM reserved: $totalRamWithStack bytes of 264 KB ($('{0:F2}' -f ($totalRamWithStack / 270336 * 100))%)"