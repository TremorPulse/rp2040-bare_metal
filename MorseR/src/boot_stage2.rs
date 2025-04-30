use core::ptr;

// Constants for base addresses
const XIP_BASE: u32 = 0x10000000;
const SSI_BASE: u32 = 0x18000000;
const M0PLUS_BASE: u32 = 0xe0000000;

// Register addresses
const SSI_CTRLR0: *mut u32 = (SSI_BASE + 0x000) as *mut u32;
const SSI_SSIENR: *mut u32 = (SSI_BASE + 0x008) as *mut u32;
const SSI_BAUDR: *mut u32 = (SSI_BASE + 0x014) as *mut u32;
const SSI_SPI_CTRLR0: *mut u32 = (SSI_BASE + 0x0f4) as *mut u32;
const M0PLUS_VTOR: *mut u32 = (M0PLUS_BASE + 0xed08) as *mut u32;

#[link_section = ".boot2"]
#[no_mangle]
pub unsafe extern "C" fn bootStage2() -> ! {
    // Disable SSI to configure it
    ptr::write_volatile(SSI_SSIENR, 0);
    // Set clock divider
    ptr::write_volatile(SSI_BAUDR, 4);
    // EEPROM mode, 32 clocks per data frame
    ptr::write_volatile(SSI_CTRLR0, (3 << 8) | (31 << 16));
    // Read Data (03h) with proper configuration
    ptr::write_volatile(SSI_SPI_CTRLR0, (6 << 2) | (2 << 8) | (0x03 << 24));
    // Enable SSI
    ptr::write_volatile(SSI_SSIENR, 1);
    
    // Calculate vector table address
    let vector_table_addr = XIP_BASE + 0x100;
    
    // Set VTOR value for vector table
    ptr::write_volatile(M0PLUS_VTOR, vector_table_addr);
    
    // Load stack pointer from first word of vector table
    let vector_table = vector_table_addr as *const u32;
    let stack_pointer = ptr::read_volatile(vector_table);
    
    // Set the stack pointer
    core::arch::asm!("msr MSP, {0}", in(reg) stack_pointer);
    
    // Get reset handler address (second entry in vector table)
    let reset_vector = vector_table.add(1);
    let reset_handler = ptr::read_volatile(reset_vector);
    
    // Ensure we're actually going to jump to a valid location in flash
    if reset_handler >= XIP_BASE && reset_handler < (XIP_BASE + 0x1000000) {
        // Jump to reset handler
        core::arch::asm!("bx {0}", in(reg) reset_handler, options(noreturn));
    }
    
    // If we get here, something went wrong - enter default handler behavior
    loop {
        core::arch::asm!("wfi");
    }
}