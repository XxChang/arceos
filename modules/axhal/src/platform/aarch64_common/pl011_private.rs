use core::ptr::NonNull;

use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::registers::{ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};

register_bitfields![u32,
    UARTDR [
        DATA OFFSET(0) NUMBITS(8) [],
        FE OFFSET(8) NUMBITS(1) [],
        PE OFFSET(9) NUMBITS(1) [],
        BE OFFSET(10) NUMBITS(1) [],
        OE OFFSET(11) NUMBITS(1) []
    ],

    UARTFR [
        CTS OFFSET(0) NUMBITS(1) [],
        DSR OFFSET(1) NUMBITS(1) [],
        DCD OFFSET(2) NUMBITS(1) [],
        BUSY OFFSET(3) NUMBITS(1) [],
        RXFE OFFSET(4) NUMBITS(1) [],
        TXFF OFFSET(5) NUMBITS(1) [],
        RXFF OFFSET(6) NUMBITS(1) [],
        TXFE OFFSET(7) NUMBITS(1) [],
        RI OFFSET(8) NUMBITS(1) [],
    ],

    UARTIBRD [
        DIVINT OFFSET(0) NUMBITS(16) [],
    ],

    UARTFBRD [
        DIVFRAC OFFSET(0) NUMBITS(6) [],
    ],

    UARTCR [
        UARTEN OFFSET(0) NUMBITS(1) [],
        SIREN OFFSET(1) NUMBITS(1) [],
        SIRLP OFFSET(2) NUMBITS(1) [],
        LBE OFFSET(7) NUMBITS(1) [],
        TXE OFFSET(8) NUMBITS(1) [],
        RXE OFFSET(9) NUMBITS(1) [],
        DTR OFFSET(10) NUMBITS(1) [],
        RTS OFFSET(11) NUMBITS(1) [],
    ],

    pub UARTICR [
        RIRMIS OFFSET(0) NUMBITS(1) [],
        CTSMIC OFFSET(1) NUMBITS(1) [],
        DCDMIC OFFSET(2) NUMBITS(1) [],
        DSRMIC OFFSET(3) NUMBITS(1) [],
        RXIC OFFSET(4) NUMBITS(1) [],
        TXIC OFFSET(5) NUMBITS(1) [],
        RTIC OFFSET(6) NUMBITS(1) [],
        FEIC OFFSET(7) NUMBITS(1) [],
        PEIC OFFSET(8) NUMBITS(1) [],
        BEIC OFFSET(9) NUMBITS(1) [],
        OEIC OFFSET(10) NUMBITS(1) [],
    ],
];

register_structs! {
    PL011Registers {
        (0x00 => dr: ReadWrite<u32, UARTDR::Register>),
        (0x04 => _reserved0),
        (0x18 => fr: ReadWrite<u32, UARTFR::Register>),
        (0x1c => _reserved1),
        (0x30 => cr: ReadWrite<u32, UARTCR::Register>),
        (0x34 => _reserved2),
        (0x44 => pub icr: WriteOnly<u32, UARTICR::Register>),
        (0x48 => @END),
    }
}

struct PL011RegPtr(pub NonNull<PL011Registers>);

pub struct PL011 {
    base: PL011RegPtr,
}

unsafe impl Send for PL011RegPtr {}

impl PL011 {
    pub const fn new(base: *mut u8) -> Self {
        PL011 {
            base: PL011RegPtr(NonNull::new(base).unwrap().cast()),
        }
    }

    pub fn init(&self) {
    }

    pub fn putchar(&self, c: u8) {
        while unsafe { self.base.0.as_ref().fr.is_set(UARTFR::TXFF) } {}
        unsafe {
            self.base.0.as_ref().dr.modify(UARTDR::DATA.val(c.into()));
        }
    }

    pub fn getchar(&self) -> u8 {
        while unsafe { self.base.0.as_ref().fr.is_set(UARTFR::RXFE) } {}
        unsafe { self.base.0.as_ref().dr.read(UARTDR::DATA) as u8 }
    }

    pub fn is_receive_interrupt(&self) -> bool {
        !unsafe { self.base.0.as_ref().fr.is_set(UARTFR::RXFE) }
    }

    pub fn ack_interrupts(&self) {
        unsafe {
            self.base.0.as_ref().icr.write(
                UARTICR::RXIC::SET
                    + UARTICR::TXIC::SET
                    + UARTICR::FEIC::SET
                    + UARTICR::PEIC::SET
                    + UARTICR::BEIC::SET
                    + UARTICR::OEIC::SET,
            );
        }
    }
}
