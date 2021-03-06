#[rustfmt::skip]
pub mod mmio {
    pub const BASE:            usize =        0x3F00_0000;

    pub const CLOCK_BASE:      usize = BASE + 0x0010_1000;
    pub const GPIO_BASE:       usize = BASE + 0x0020_0000;
    pub const PL011_UART_BASE: usize = BASE + 0x0020_1000;
    pub const PWM_BASE:        usize = BASE + 0x0020_C000;
    pub const END_INCLUSIVE:   usize =        0x3FFF_FFFF;
}
