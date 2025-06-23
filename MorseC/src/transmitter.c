#include <stdint.h>
#include <stdbool.h>

/* Hardware Register Structures */
/* SIO (Single-cycle IO) registers for GPIO control */
typedef struct {
    uint32_t cpuid;
    uint32_t gpio_in;
    uint32_t gpio_hi_in;
    uint32_t unused;
    uint32_t gpio_out;
    uint32_t gpio_out_set;
    uint32_t gpio_out_clr;
    uint32_t gpio_out_xor;
    uint32_t gpio_oe;
    uint32_t gpio_oe_set;
    uint32_t gpio_oe_clr;
    uint32_t gpio_oe_xor;
} sio_hw_t;

/* IO Bank 0 registers for GPIO configuration */
typedef struct {
    struct {
        uint32_t status;
        uint32_t ctrl;
    } gpio[30];
    uint32_t intr[4];
    uint32_t proc0_inte[4];
    uint32_t proc0_intf[4];
    uint32_t proc0_ints[4];
} io_bank0_hw_t;

/* Pad control registers */
typedef struct {
    uint32_t voltage_select;
    uint32_t gpio[30];
    uint32_t swclk;
    uint32_t swd;
} pads_bank0_hw_t;

/* Hardware register base addresses */
#define SIO_BASE        0xd0000000
#define IO_BANK0_BASE   0x40014000
#define PADS_BANK0_BASE 0x4001c000
#define RESETS_BASE     0x4000c000

/* Register pointers */
#define sio  ((volatile sio_hw_t*)SIO_BASE)
#define io   ((volatile io_bank0_hw_t*)IO_BANK0_BASE)
#define pads ((volatile pads_bank0_hw_t*)PADS_BANK0_BASE)

/* Pin definitions */
#define BUTTON_PIN   16
#define SPEAKER_PIN  21
#define LED_PIN      25
#define GPIO_FUNC_SIO 5

/* Interrupt configuration */
#define GPIO_INT_EDGE_HIGH  0x8
#define IO_BANK0_IRQ        13
#define NVIC_BASE           0xe000e000
#define NVIC_ISER           (*(volatile uint32_t*)(NVIC_BASE + 0x100))

static void delay(uint32_t count) {
    volatile uint32_t i;
    for (i = 0; i < count; i++) {
        __asm("nop");
    }
}

/* GPIO helper functions */
static inline void gpio_set_function(uint32_t pin, uint32_t function) {
    io->gpio[pin].ctrl = function;
}

static inline void gpio_set_dir(uint32_t pin, bool out) {
    if (out)
        sio->gpio_oe_set = 1U << pin;
    else
        sio->gpio_oe_clr = 1U << pin;
}

static inline void gpio_set_pulls(uint32_t pin, bool up, bool down) {
    pads->gpio[pin] = (up ? (1U << 3) : 0) | (down ? (1U << 2) : 0) | (1U << 6); // Bit 6 is input enable
}

static inline void gpio_put(uint32_t pin, bool value) {
    if (value)
        sio->gpio_out_set = 1U << pin;
    else
        sio->gpio_out_clr = 1U << pin;
}

/* Interrupt handler for IO Bank 0 */
void ioIrqBank0(void) {
    if (io->proc0_ints[BUTTON_PIN / 8] & (GPIO_INT_EDGE_HIGH << (4 * (BUTTON_PIN % 8)))) {
        // Activate LED and speaker when button is pressed
        gpio_put(LED_PIN, true);
        gpio_put(SPEAKER_PIN, true);
        delay(100000);
        
        // Deactivate LED and speaker
        gpio_put(LED_PIN, false);
        gpio_put(SPEAKER_PIN, false);
        
        // Clear the interrupt
        io->intr[BUTTON_PIN / 8] = 0xF << (4 * (BUTTON_PIN % 8));
    }
}

int main(void) {
    // Reset IO Bank 0 peripheral
    volatile uint32_t* resets_reset = (volatile uint32_t*)(RESETS_BASE + 0x0);
    *resets_reset &= ~(1U << 5);
    while ((*resets_reset & (1U << 5)) != 0) {}

    // Configure button (GPIO16)
    gpio_set_function(BUTTON_PIN, GPIO_FUNC_SIO);
    gpio_set_dir(BUTTON_PIN, false);  // Input
    gpio_set_pulls(BUTTON_PIN, true, false);  // Pull-up
    
    // Configure LED (GPIO25)
    gpio_set_function(LED_PIN, GPIO_FUNC_SIO);
    gpio_set_dir(LED_PIN, true);  // Output
    
    // Configure speaker (GPIO21)
    gpio_set_function(SPEAKER_PIN, GPIO_FUNC_SIO);
    gpio_set_dir(SPEAKER_PIN, true);  // Output
    
    // Setup button interrupt
    io->intr[BUTTON_PIN / 8] = 0xF << (4 * (BUTTON_PIN % 8));  // Clear pending interrupts
    io->proc0_inte[BUTTON_PIN / 8] |= GPIO_INT_EDGE_HIGH << (4 * (BUTTON_PIN % 8));  // Enable rising edge
    NVIC_ISER = 1U << IO_BANK0_IRQ;  // Enable interrupt in NVIC

    // Startup test pattern
    for(int i = 0; i < 3; i++) {
        gpio_put(LED_PIN, true);
        gpio_put(SPEAKER_PIN, true);
        delay(100000);
        gpio_put(LED_PIN, false);
        gpio_put(SPEAKER_PIN, false);
        delay(100000);
    }
    
    // Sleep until interrupt
    while (1) {
        __asm volatile("wfi");
    }
}