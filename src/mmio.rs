pub const BASE: usize = 0xfe000000;

pub unsafe fn write(index: u8, value: u32) {
    write_aux(index, 0, value)
}

pub unsafe fn write_aux(index: u8, aux: u16, value: u32) {
    let addr = BASE + ((index as usize + ((aux as usize) << 8)) << 2);
    core::ptr::write_volatile(addr as *mut u32, value);
}

pub unsafe fn read(index: u8) -> u32 {
    read_aux(index, 0)
}

pub unsafe fn read_aux(index: u8, aux: u16) -> u32 {
    let addr = BASE + ((index as usize + ((aux as usize) << 8)) << 2);
    core::ptr::read_volatile(addr as *mut u32)
}
