#![allow(dead_code)]

pub mod hw {
    use core::ptr::{read_volatile, write_volatile};

    macro_rules! hw_reg {
        (rw $addr: expr, $read:ident, $write: ident, $data_type: ty) => {
            #[allow(dead_code)]
            pub fn $read() -> $data_type {
                unsafe { read_volatile($addr as *const $data_type) }
            }

            #[allow(dead_code)]
            pub fn $write(value: $data_type) {
                unsafe { write_volatile($addr as *mut $data_type, value); }
            }
        };
        (r $addr: expr, $read: ident, $data_type: ty) => {
            #[allow(dead_code)]
            pub fn $read() ->  $data_type {
                unsafe { read_volatile($addr as *const $data_type) }
            }
        };
        (w $addr: expr, $write: ident, $data_type: ty) => {
            #[allow(dead_code)]
            pub fn $write(value:  $data_type) {
                unsafe { write_volatile($addr as *mut $data_type, value); }
            }
        };
    }

    hw_reg!(rw 0x4000000, read_dispcnt, write_dispcnt, u16);
    hw_reg!(rw 0x4000004, read_dispstat, write_dispstat, u16);
    hw_reg!(rw 0x4000008, read_bg0cnt, write_bg0cnt, u16);
    hw_reg!(rw 0x400000a, read_bg1cnt, write_bg1cnt, u16);
    hw_reg!(rw 0x400000c, read_bg2cnt, write_bg2cnt, u16);
    hw_reg!(rw 0x400000e, read_bg3cnt, write_bg3cnt, u16);
    hw_reg!(w 0x4000010, write_bg0hofs, u16);
    hw_reg!(w 0x4000012, write_bg0vofs, u16);
    hw_reg!(w 0x4000014, write_bg1hofs, u16);
    hw_reg!(w 0x4000016, write_bg1vofs, u16);
    hw_reg!(w 0x4000018, write_bg2hofs, u16);
    hw_reg!(w 0x400001a, write_bg2vofs, u16);
    hw_reg!(w 0x400001c, write_bg3hofs, u16);
    hw_reg!(w 0x400001e, write_bg3vofs, u16);
    hw_reg!(r 0x4000130, read_keyinput, u16);

    pub enum VideoMode {
        Mode0 = 0b000,//tile mode, 4 bg layers, no rotate/scale, all support scroll
        Mode1 = 0b001,//tile mode, 3 bg layers, only BG 2 can rotate/scale, BGs 0,1 support scroll
        Mode2 = 0b010,//tile mode, 2 bg layers, BGs 2,3  can rotate/scale, no scroll
        Mode3 = 0b0000010000000011,//bitmap mode 16bit color // BG 2 only, rotate/scale
        Mode4 = 0b0000010000000100,//bitmap mode 8bit references to a palette // BG 2 only, rotate/scale
        Mode5 = 0b0000010000000101,//bitmap mode 16 bit color backbufferd (160x128) // BG 2 only, rotate/scale
    }

    pub fn set_video_mode(mode : VideoMode) {
        write_dispcnt(mode as u16);
    }

    pub fn write_pal(index: u32, col: u16) {
        if index < 512 {
            unsafe {
                write_volatile((0x5000000u32 + (index * 2) as u32) as *mut u16, col);
            }
        }
    }

    pub fn write_vram16(offset: u32, data: u16) {
        if offset < 0xc000 {
            unsafe { write_volatile((0x6000000u32 + offset * 2) as *mut u16, data) }
        }
    }

    // color for gba [unused bit] BBB BBGG GGGR RRRR
    // downscales from 8bit color ( 255  -> 31 )
    pub fn make_color(red: u8, green: u8, blue: u8) -> u16
    {
        (red / 8) as u16 | ((green / 8) as u16) << 5 | ((blue / 8) as u16) << 10
    }
}

pub struct KeyState {
    state: u32,
}

pub enum Key {
    A = 1,
    B = 2,
    Select = 4,
    Start = 8,
    Right = 16,
    Left = 32,
    Up = 64,
    Down = 128,
    R = 256,
    L = 512,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState { state: 0 }
    }
    pub fn update(&mut self) {
        let pressed = hw::read_keyinput() ^ 0xffffu16;
        let triggered = pressed & !self.get_pressed();
        self.state = (pressed as u32) | ((triggered as u32) << 16);
    }
    fn get_pressed(&self) -> u16 {
        self.state as u16
    }
    fn get_triggered(&self) -> u16 {
        (self.state >> 16) as u16
    }
    #[allow(dead_code)]
    pub fn is_pressed(&self, key: Key) -> bool {
        self.get_pressed() & (key as u16) != 0
    }
    #[allow(dead_code)]
    pub fn is_triggered(&self, key: Key) -> bool {
        self.get_triggered() & (key as u16) != 0
    }
}

pub fn wait_vblank() {
    while hw::read_dispstat() & 1 != 0 {}
    while hw::read_dispstat() & 1 == 0 {}
}
