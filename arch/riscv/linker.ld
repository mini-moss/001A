OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS
{
    . = 0x80200000;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    .rodata ALIGN(4K) : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    .data ALIGN(4K) : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .stack ALIGN(4K) : {
        *(.bss.stack)
    }

    .bss ALIGN(4K) : {
        *(.bss.bootstack)

        . = ALIGN(4K);
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    PROVIDE(end = .);
}
