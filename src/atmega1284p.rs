#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum DataRegister {
    PinA    = 0x20,
    DDRA    = 0x21,
    PortA   = 0x22,
    PinB    = 0x23,
    DDRB    = 0x24,
    PortB   = 0x25,
    PinC    = 0x26,
    DDRC    = 0x27,
    PortC   = 0x28,
    PinD    = 0x29,
    DDRD    = 0x2a,
    PortD   = 0x2b,
}