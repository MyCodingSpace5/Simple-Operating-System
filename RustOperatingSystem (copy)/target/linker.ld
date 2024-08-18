ENTRY(_start);

SECTIONS
{
  . = 0x100000;

  .text :
  {
    *(.text._start)
    *(.text*)
  }

  .rodata :
  {
    *(.rodata*)
  }

  .data : ALIGN(4096)
  {
    *(.data*)
  }

  .bss : ALIGN(4096)
  {
    *(.bss*)
    *(COMMON)
  }
}
