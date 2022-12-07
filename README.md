Rusty Maple
===========

Test project to explore capabilities of Leaflabs Maple Mini board.

There are many clones, most close is of BAITE brand.

Do not confuse with "BluePill".

Main differencies (BluePill vs MapleMini):

 * STM32F103C8T6 vs STM32F103CBT6 (64kb Flash vs 128 kb flash)
 * MapleMini has bootloader and different schematics for USB connection (two NPNs)
 * BluePill has pull-up on PA12 (USB DP)
 * MapleMini control these NPNs with GPIO PB9 (labeled DISC, see schematics)
 * BluePill has two leds: Power (always ON) and PC13.
 * MapleMini has one led: PB1

