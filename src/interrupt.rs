pub fn without_interrupts<F: FnOnce() -> R, R>(f: F) -> R {
    let prev_mstatus: usize;
    unsafe {
        llvm_asm!("csrrc $0, mstatus, $1" : "=r" (prev_mstatus) : "r" (1 << 3) :: "volatile"); // mie
    }
    let ret = f();
    if prev_mstatus & (1 << 3) != 0 {
        unsafe {
            llvm_asm!("csrs mstatus, $0" :: "r" (1 << 3) :: "volatile"); // mie
        }
    }
    ret
}