MEMORY
{
    flash(rx) : ORIGIN = 0x10000000, LENGTH = 2048k
    sram(rwx) : ORIGIN = 0x20000000, LENGTH = 264k  
}

SECTIONS
{
    .boot2 : {
        _sboot2 = .;
        *(.boot2*)
        . = ALIGN(256);  /* Pad to 256 bytes */
        _eboot2 = .;
    } > flash

    .vector_table : {
        *(.vector_table)
    } > flash

    .text : {
        *(.text*)
        *(.rodata*)
        _etext = .;
    } > flash

    .data : {
        _sdata = .;
        *(.data*)
        _edata = .;
    } > sram AT > flash

    .bss : {
        _sbss = .;
        *(.bss*)
        *(COMMON)
        _ebss = .;
    } > sram

    .stack (NOLOAD) : {
        . = ALIGN(8);
        _sstack = .;
        . = . + 0x2000;  /* 8 KB stack */
        _estack = .;
    } > sram
}