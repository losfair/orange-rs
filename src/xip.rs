use crate::mmio;

const XIP_OFFSET_REG: u8 = 0x31;
const XIP_MASK_REG: u8 = 0x32;

pub fn set_xip_offset(new_offset: u32) {
    unsafe {
        mmio::write(XIP_OFFSET_REG, new_offset);
    }
}

pub fn set_xip_mask(new_mask: u32) {
    unsafe {
        mmio::write(XIP_MASK_REG, new_mask);
    }
}
