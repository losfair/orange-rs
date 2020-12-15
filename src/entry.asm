.section .text.entry

.globl _start
_start:
    j _start_2

# _intr_entry at 0x02000010
.space 16-(.-_start)
.globl _intr_entry
_intr_entry:
    j _intr_entry

_start_2:
    li t0, 0x02000010
    csrw mtvec, t0

_copy_ram_preload:
    la t0, ROM_END
    la t1, RAM_START
    la t2, RAM_PRELOAD_END

_copy_ram_preload.loop:
    bge t1, t2, _copy_ram_preload.loop_end
    lw t3, (t0)
    sw t3, (t1)
    addi t0, t0, 4
    addi t1, t1, 4
    j _copy_ram_preload.loop

_copy_ram_preload.loop_end:
    la sp, _stack
    li t0, 8192 # len(SYS_STACK)
    add sp, sp, t0
    j rust_main

.section .bss.stack
.align 8
_stack:
.space 8192
