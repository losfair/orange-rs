OUTPUT_ARCH(riscv)

ENTRY(_start)

ROM_ADDRESS = 0x20000000;
RAM_ADDRESS = 0x60000200;

SECTIONS
{
    . = ROM_ADDRESS;

    .text : {
        *(.text .text.*)
    }

    . = RAM_ADDRESS;

    .data : {
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .bss : {
        *(.sbss .bss .bss.*)
    }

    /DISCARD/ : { *(.eh_frame) }
}