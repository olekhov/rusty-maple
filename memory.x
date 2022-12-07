/* memory.x - Linker script for the STM32F103CBT6 with DFU bootloader */
MEMORY
{
  /* Flash memory begins at 0x80000000 and has a size of 128kB*/
  /* FLASH : ORIGIN = 0x08000000, LENGTH = 64K*/

  /* 20k: DFU bootloader */
  FLASH : ORIGIN = 0x08005000, LENGTH = 108k

  /* RAM begins at 0x20000000 and has a size of 20kB, 3kb for bootloader*/
  RAM : ORIGIN = 0x20000c00, LENGTH = 17K
}

