/* Vector Table
 * This file defines the interrupt vector table and handlers for the RP2040
 * The vector table is placed at a specific memory location and contains
 * pointers to interrupt service routines */

#include <stdint.h>
#include <stdbool.h>

/* Function pointer type for vector table entries */
typedef void (*vectFunc) (void);

/* External stack pointer symbol defined by the linker script */
extern uint32_t _sstack;

/* Core handler function declarations */
__attribute__((noreturn)) void defaultHandler();
__attribute__((noreturn)) void resetHandler();

/* Core exception handler declarations - weak aliases to defaultHandler */
void nmiHandler         () __attribute__((weak, alias("defaultHandler")));
void hardFaultHandler   () __attribute__((weak, alias("defaultHandler")));
void svCallHandler      () __attribute__((weak, alias("defaultHandler")));
void pendSvHandler      () __attribute__((weak, alias("defaultHandler")));
void sysTickHandler     () __attribute__((weak, alias("defaultHandler")));
void ioIrqBank0         () __attribute__((weak, alias("defaultHandler")));

/* Main program entry point declaration */
extern int main(void);

/* Vector table definition */
const vectFunc vector[] __attribute__((section(".vector"))) = 
{
    /* Core System Handler Vectors */
    (vectFunc)(&_sstack),   /* Initial Stack Pointer value */
    resetHandler,           /* Reset Handler */
    nmiHandler,             /* Non-Maskable Interrupt Handler */
    hardFaultHandler,       /* Hard Fault Handler */
    0,                      /* Reserved */
    0,                      /* Reserved */
    0,                      /* Reserved */
    0,                      /* Reserved */
    0,                      /* Reserved */
    0,                      /* Reserved */
    0,                      /* Reserved */
    svCallHandler,          /* SVCall Handler */
    0,                      /* Reserved */
    0,                      /* Reserved */
    pendSvHandler,          /* PendSV Handler */
    sysTickHandler,         /* SysTick Handler */

    /* RP2040 Specific Interrupts - Only include ioIrqBank0 for GPIO interrupts */
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  /* 0-12: Unused */
    ioIrqBank0,             /* 13: IO Bank 0 - For button interrupts */
};

void resetHandler()
{
    /* Symbols created by the linker script */
    extern uint32_t _etext;     /*  End of .text section (in flash) */
    extern uint32_t _sdata;     /* Start of .data section (in RAM) */
    extern uint32_t _edata;     /* End of .data section (in RAM) */
    extern uint32_t _sbss;      /* Start of .bss section (in RAM) */
    extern uint32_t _ebss;      /* End of .bss section (in RAM) */

    /* Copy initialized data from flash to RAM */
    uint32_t *src = &_etext;    /* Source is end of code in flash */
    uint32_t *dst = &_sdata;    /* Destination is start of data in RAM */
    
    while (dst < &_edata) {
        *dst++ = *src++;       
    }

    /* Zero out the BSS section */
    dst = &_sbss;
    while (dst < &_ebss) {
        *dst++ = 0;             
    }

    /*  Call the main program */
    main();
    
    /* If main ever returns, stay in infinite loop */
    while(true);
}

/* Default interrupt handler */
void defaultHandler()
{
    while (true) {
        __asm volatile("wfi");  /* Wait for interrupt - low power state */
    }
}