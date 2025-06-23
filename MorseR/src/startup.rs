use core::ptr;

// External references
extern "C" {
    // External stack pointer symbol defined by the linker script
    static _sstack: u32;
    // External symbols for the vector table
    static _etext: u32;     // End of .text section (in flash)
    static _sdata: u32;     // Start of .data section (in RAM)
    static _edata: u32;     // End of .data section (in RAM)
    static _sbss: u32;      // Start of .bss section (in RAM)
    static _ebss: u32;      // End of .bss section (in RAM)
    // External main function
    fn main();
    // External interrupt handler
    fn ioIrqBank0();
}

// Default handler - never returns
#[no_mangle]
pub extern "C" fn defaultHandler() -> ! {
    loop {
        unsafe { core::arch::asm!("wfi"); }
    }
}

// Reset handler - called on system startup
#[no_mangle]
#[link_section = ".text"]
pub extern "C" fn resetHandler() -> ! {
    unsafe {
        // Copy initialized data from flash to RAM
        let mut src = ptr::addr_of!(_etext) as *const u32;
        let mut dst = ptr::addr_of!(_sdata) as *mut u32;
        let end = ptr::addr_of!(_edata) as *mut u32;
       
        while dst < end {
            ptr::write_volatile(dst, ptr::read_volatile(src));
            dst = dst.add(1);
            src = src.add(1);
        }
       
        // Zero out the BSS section
        let mut bss = ptr::addr_of!(_sbss) as *mut u32;
        let bss_end = ptr::addr_of!(_ebss) as *mut u32;
       
        while bss < bss_end {
            ptr::write_volatile(bss, 0);
            bss = bss.add(1);
        }

        // Set Vector Table Offset Register (VTOR)
        let vtor = 0xE000ED08 as *mut u32;
        ptr::write_volatile(vtor, 0x10000100);  

        // Call main function
        main();
       
        // If main returns, enter an infinite loop
        loop {
            core::arch::asm!("wfi");
        }
    }
}

// Exception handlers - all marked as diverging (-> !) to match the vector table entry type
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn nmiHandler() -> ! {
    defaultHandler()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn hardFaultHandler() -> ! {
    defaultHandler()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn svCallHandler() -> ! {
    defaultHandler()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn pendSvHandler() -> ! {
    defaultHandler()
}

#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn sysTickHandler() -> ! {
    defaultHandler()
}

// Wrapper for GPIO IRQ handler
#[no_mangle]
pub extern "C" fn ioIrqBank0Handler() -> ! {
    unsafe {
        ioIrqBank0();
    }
    defaultHandler()
}

// Define the type for vector table entries
#[repr(C)]
#[derive(Copy, Clone)]
union VectorTableEntry {
    handler: unsafe extern "C" fn() -> !,
    reserved: u32,
    stack_top: *const u32,
}

// Make it safe to share between threads (required for static)
unsafe impl Sync for VectorTableEntry {}

// Define the vector table - explicitly list all entries rather than using a loop
#[link_section = ".vector_table"]
#[no_mangle]
pub static VECTOR_TABLE: [VectorTableEntry; 48] = [
    // Initial Stack Pointer
    VectorTableEntry { stack_top: { ptr::addr_of!(_sstack) } },
    
    // Core exception handlers
    VectorTableEntry { handler: resetHandler },
    VectorTableEntry { handler: nmiHandler },
    VectorTableEntry { handler: hardFaultHandler },
    VectorTableEntry { handler: defaultHandler }, // MemManage
    VectorTableEntry { handler: defaultHandler }, // BusFault
    VectorTableEntry { handler: defaultHandler }, // UsageFault
    
    // Reserved entries 7-10
    VectorTableEntry { reserved: 0 },
    VectorTableEntry { reserved: 0 },
    VectorTableEntry { reserved: 0 },
    VectorTableEntry { reserved: 0 },
    
    VectorTableEntry { handler: svCallHandler },
    VectorTableEntry { handler: defaultHandler }, // Debug Monitor
    VectorTableEntry { reserved: 0 }, // Reserved entry 13
    VectorTableEntry { handler: pendSvHandler },
    VectorTableEntry { handler: sysTickHandler },
    
    // Peripheral IRQs - RP2040 has 32 IRQs
    VectorTableEntry { handler: defaultHandler }, // IRQ0
    VectorTableEntry { handler: defaultHandler }, // IRQ1
    VectorTableEntry { handler: defaultHandler }, // IRQ2
    VectorTableEntry { handler: defaultHandler }, // IRQ3
    VectorTableEntry { handler: defaultHandler }, // IRQ4
    VectorTableEntry { handler: defaultHandler }, // IRQ5
    VectorTableEntry { handler: defaultHandler }, // IRQ6
    VectorTableEntry { handler: defaultHandler }, // IRQ7
    VectorTableEntry { handler: defaultHandler }, // IRQ8
    VectorTableEntry { handler: defaultHandler }, // IRQ9
    VectorTableEntry { handler: defaultHandler }, // IRQ10
    VectorTableEntry { handler: defaultHandler }, // IRQ11
    VectorTableEntry { handler: defaultHandler }, // IRQ12
    VectorTableEntry { handler: ioIrqBank0Handler }, // IRQ13 (IO_BANK0)
    VectorTableEntry { handler: defaultHandler }, // IRQ14
    VectorTableEntry { handler: defaultHandler }, // IRQ15
    VectorTableEntry { handler: defaultHandler }, // IRQ16
    VectorTableEntry { handler: defaultHandler }, // IRQ17
    VectorTableEntry { handler: defaultHandler }, // IRQ18
    VectorTableEntry { handler: defaultHandler }, // IRQ19
    VectorTableEntry { handler: defaultHandler }, // IRQ20
    VectorTableEntry { handler: defaultHandler }, // IRQ21
    VectorTableEntry { handler: defaultHandler }, // IRQ22
    VectorTableEntry { handler: defaultHandler }, // IRQ23
    VectorTableEntry { handler: defaultHandler }, // IRQ24
    VectorTableEntry { handler: defaultHandler }, // IRQ25
    VectorTableEntry { handler: defaultHandler }, // IRQ26
    VectorTableEntry { handler: defaultHandler }, // IRQ27
    VectorTableEntry { handler: defaultHandler }, // IRQ28
    VectorTableEntry { handler: defaultHandler }, // IRQ29
    VectorTableEntry { handler: defaultHandler }, // IRQ30
    VectorTableEntry { handler: defaultHandler }, // IRQ31
];