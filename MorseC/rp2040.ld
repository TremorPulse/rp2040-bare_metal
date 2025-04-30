ENTRY(bootStage2);
MEMORY
{
    flash(rx) : ORIGIN = 0x10000000, LENGTH = 2048k
    sram(rwx) : ORIGIN = 0x20000000, LENGTH = 264k  
}
SECTIONS
{
    /* Boot2 section needs to be exactly 256 bytes */
    .boot2 :
    {
        _sboot2 = .;
        *(.boot2*)
        _eboot2 = .;
        . = . + (252 - (_eboot2 - _sboot2));
        *(.crc*)
    } > flash
    
    /* Vector table and code */
    .text :
    {
        *(.vector*)
        *(.text*)
        *(.rodata*)
        _etext = .;            /* Define end of text section */
    } > flash
    
    /* Data section - initialized variables */
    .data :
    {
        _sdata = .;            /* Define start of data section */
        *(.data*)
        _edata = .;            /* Define end of data section */
    } > sram AT > flash
    
    /* BSS section - uninitialized variables */
    .bss :
    {
        _sbss = .;             /* Define start of bss section */
        *(.bss*)
        *(COMMON)
        _ebss = .;             /* Define end of bss section */
    } > sram
    
    /* Stack section */
    .stack (NOLOAD) :
    {
        . = ORIGIN(sram) + LENGTH(sram);
        _sstack = .;
    } > sram
}