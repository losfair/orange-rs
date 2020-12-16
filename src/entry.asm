.section .text.entry

.globl _start
_start:
    la sp, _stack
    li t0, 8192 # len(SYS_STACK)
    add sp, sp, t0
    j rust_main

.section .bss.stack
.align 8
_stack:
.space 8192
