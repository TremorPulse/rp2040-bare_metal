#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::addr_of_mut;

/* Hardware Register Structures */
/* SIO (Single-cycle IO) registers for fast GPIO access */
#[repr(C)]
struct SioHw {
    cpuid: u32,          /* Processor core identifier */
    gpio_in: u32,        /* Input values for GPIO 0-29 */
    gpio_hi_in: u32,     /* Input values for GPIO 30-35 */
    unused: u32,         /* Reserved */
    gpio_out: u32,       /* GPIO output values */
    gpio_out_set: u32,   /* Set GPIO output bits */
    gpio_out_clr: u32,   /* Clear GPIO output bits */
    gpio_out_xor: u32,   /* XOR GPIO output bits */
    gpio_oe: u32,        /* GPIO output enable */
    gpio_oe_set: u32,    /* Set GPIO output enable bits */
    gpio_oe_clr: u32,    /* Clear GPIO output enable bits */
    gpio_oe_xor: u32,    /* XOR GPIO output enable bits */
}

/* IO Bank 0 registers for GPIO configuration and interrupts */
/* Each structure has two 32-bit values: status and ctrl */
#[repr(C)]
struct IoBank0Hw {
    gpio: [GpioCtrl; 30],    /* We repeated this 30 times for each GPIO */
    intr: [u32; 4],          /* Raw interrupts */
    proc0_inte: [u32; 4],    /* Interrupt enable for processor 0 */
    proc0_intf: [u32; 4],    /* Interrupt force for processor 0 */
    proc0_ints: [u32; 4],    /* Interrupt status for processor 0 */
}

#[repr(C)]
struct GpioCtrl {
    status: u32,     /* GPIO status */
    ctrl: u32,       /* GPIO control including function selection */
}

/* Pad control registers for GPIO electrical properties */
#[repr(C)]
struct PadsBank0Hw {
    voltage_select: u32,  /* Voltage select */
    gpio: [u32; 30],      /* Pad control register for each GPIO */
    swclk: u32,           /* Pad control register for SWCLK */
    swd: u32,             /* Pad control register for SWD */
}

/* Base addresses for hardware registers */
const SIO_BASE: u32 = 0xd0000000;
const IO_BANK0_BASE: u32 = 0x40014000;
const PADS_BANK0_BASE: u32 = 0x4001c000;
const RESETS_BASE: u32 = 0x4000c000;

/* Register access pointers */
/* This means:
   1. Take base address
   2. Cast it to a pointer to the struct
   3. Access registers through this pointer */
#[inline(always)]
fn sio() -> *mut SioHw {
    SIO_BASE as *mut SioHw
}

#[inline(always)]
fn io() -> *mut IoBank0Hw {
    IO_BANK0_BASE as *mut IoBank0Hw
}

#[inline(always)]
fn pads() -> *mut PadsBank0Hw {
    PADS_BANK0_BASE as *mut PadsBank0Hw
}

/* Pin definitions */
const BUTTON_PIN: u32 = 16;    /* Push button input */
const SPEAKER_PIN: u32 = 21;   /* Speaker output */
const LED_PIN: u32 = 25;       /* Onboard LED */
const GPIO_FUNC_SIO: u32 = 5;  /* SIO function for GPIO */

/* Interrupt configuration */
const GPIO_INT_EDGE_HIGH: u32 = 0x8;
const IO_BANK0_IRQ: u32 = 13;      /* IO Bank 0 interrupt number */
const NVIC_BASE: u32 = 0xe000e000;
const NVIC_ISER: *mut u32 = (NVIC_BASE + 0x100) as *mut u32;

/* Simple delay function using NOP instructions */
#[inline(always)]
fn delay(count: u32) {
    /* Loop to create delay with NOP instructions */
    for i in 0..count {
        unsafe {
            asm!("nop");
        }
    }
}

/* Interrupt handler for IO Bank 0 */
#[no_mangle]
pub extern "C" fn ioIrqBank0() {
    unsafe {
        /* This checks if button caused interrupt:
           1. proc0_ints[BUTTON_PIN / 8] gets the right interrupt status register
           2. BUTTON_PIN / 8 divides pin number by 8 to get right register (pins 0-7 use 0, 8-15 use 1, etc)
           3. BUTTON_PIN % 8 gets remainder to find position within register
           4. << (4 * (BUTTON_PIN % 8)) shifts bits to right position (each pin uses 4 bits) */
        let pin_index = BUTTON_PIN / 8;
        let pin_offset = 4 * (BUTTON_PIN % 8);
        
        if (*io()).proc0_ints[pin_index as usize] & (GPIO_INT_EDGE_HIGH << pin_offset) != 0 {
            /* Set bits using OR operation:
               1. 1U creates unsigned integer 1
               2. << LED_PIN shifts 1 left by LED_PIN positions 
               3. Writing to gpio_out_set sets those pins high */
            (*sio()).gpio_out_set = (1u32 << LED_PIN) | (1u32 << SPEAKER_PIN);
            delay(100000);
            
            /* Deactivate LED and speaker - same but writes to clear register */
            (*sio()).gpio_out_clr = (1u32 << LED_PIN) | (1u32 << SPEAKER_PIN);
            
            /* Clear the interrupt */
            (*io()).intr[pin_index as usize] = 0xF << pin_offset;
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        /* Reset IO Bank 0 peripheral:
           1. Create pointer to reset register
           2. Clear bit 5 using AND with inverted bit pattern
           3. Wait while bit is still set */
        let resets_reset = (RESETS_BASE + 0x0) as *mut u32;
        *resets_reset &= !(1u32 << 5);
        
        let resets_reset_done = (RESETS_BASE + 0x8) as *mut u32;
        while (*resets_reset_done & (1u32 << 5)) == 0 {}

        /* Configure button (GPIO16)
           1. Set GPIO function using direct register write
           2. Clear output enable bit for input mode
           3. Set pad control (pull-up and input enable) */
        (*io()).gpio[BUTTON_PIN as usize].ctrl = GPIO_FUNC_SIO;    /* Set to SIO function */
        (*sio()).gpio_oe_clr = 1u32 << BUTTON_PIN;                 /* Set as input */
        (*pads()).gpio[BUTTON_PIN as usize] = (1u32 << 3) | (1u32 << 6); /* Enable pull-up and input */
        
        /* Configure LED (GPIO25) */
        (*io()).gpio[LED_PIN as usize].ctrl = GPIO_FUNC_SIO;       /* Set to SIO function */
        (*sio()).gpio_oe_set = 1u32 << LED_PIN;                    /* Set as output */
        
        /* Configure speaker (GPIO21) */
        (*io()).gpio[SPEAKER_PIN as usize].ctrl = GPIO_FUNC_SIO;   /* Set to SIO function */
        (*sio()).gpio_oe_set = 1u32 << SPEAKER_PIN;                /* Set as output */
        
        /* Setup button interrupt 
           1. Clear existing interrupts
           2. Enable rising edge interrupt for button
           3. Enable IO Bank 0 interrupt in NVIC (Nested Vectored Interrupt Controller) */
        let pin_index = BUTTON_PIN / 8;
        let pin_offset = 4 * (BUTTON_PIN % 8);
        
        (*io()).intr[pin_index as usize] = 0xF << pin_offset;  /* Clear pending interrupts */
        (*io()).proc0_inte[pin_index as usize] |= GPIO_INT_EDGE_HIGH << pin_offset;  /* Enable rising edge interrupt */
        *NVIC_ISER = 1u32 << IO_BANK0_IRQ;  /* Enable interrupt in NVIC */

        /* Startup test pattern */
        for _ in 0..3 {
            /* Turn on LED and speaker */
            (*sio()).gpio_out_set = (1u32 << LED_PIN) | (1u32 << SPEAKER_PIN);
            delay(100000);
            /* Turn off LED and speaker */
            (*sio()).gpio_out_clr = (1u32 << LED_PIN) | (1u32 << SPEAKER_PIN);
            delay(100000);
        }
        
        /* Debug LED flash */
        (*sio()).gpio_out_set = 1u32 << LED_PIN;
        delay(500000);
        (*sio()).gpio_out_clr = 1u32 << LED_PIN;
        
        /* 1. CPU sleeps until interrupt occurs
           2. wfi = Wait For Interrupt instruction
           3. loop prevents function from returning */
        loop {
            asm!("wfi");
        }
    }
}