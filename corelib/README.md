# Core Implementation
___

This document aims to provide an overview of what the core implementations of this library are like and how they are translated to Rust code. Please note that this project and document is still under development and will be updated with further commits.

## Specifications:

**1. Random Access Memory (RAM)**

The RAM is the primary read/write scope for CHIP8. Original implementations of CHIP8 described the RAM to be 4kb long (4096 bytes) however, since it was a virtual machine and interpreter, this was not a restriction but simply a standard to follow as 4 kilobytes was widely available with standard systems and could fit all requirements of CHIP8 sufficiently. The RAM loads up hexadecimal font sprites at the first 80 bytes (Not a standard but since the first 512 bytes are free, one can use this space). The actual program (Read Only Memory - ROM) is loaded from the 512th byte onwards. This is done to reduce the overhead of accessing the ROM each time to execute a single instruction by simply loading it up into a faster and programatically accessible interface.

**2. Display**

CHIP8 used a 64x32 monochrome display to suit for the Telmac 1800 and COSMIC VIP display. (Section and corresponding code to be updated)

**3. Registers**

A set of 16 Registers named V0-VF is used, each capable of holding a byte of data. The VF register is multipurpose and is used as the carry flag/no-borrow flag, a common register and for checking collisions of pixels. Hence, it should be used accordingly and with caution. Rest of the 15 registers are used to load operands and store temporary values under modification by opcodes. There is also a 2 byte long index register but it should be treated differently from the common registers.

**4. Stack**

The standard stack sepcification is 16 indices long, each capable of holding two bytes of data. The main purpose of the stack is to store return addresses of subroutine calls (Defined by ROMs).

**5. Stack Pointer**

A 2 byte pointer to the next stack index where a value will be pushed (Technically only 4 bits are required to index 16 indices, but emulators use 2 bytes to ensure consistency. Actual implementations had limited memory and hence used one byte for this)

**6. Index Register**

The index register is used to access the RAM for read/write operations. Since the RAM is 4096 bytes long, we need a maximum of 12 bits to address all of them (2^12 = 4096). However, 12 is not a clean power of 2 and represents 1.5 bytes which is not cleanly workable by CPUs as those are built to work with multiples of bytes. Hence, we use 16 bits (or two bytes) to address the RAM.

___

## Fontset For CHIP8

Most ROMs define fontsets to use in association. However, it is a good idea to implement a basic fontset for hexadecimal characters (0-9, A-F) for testing and development. The CHIP8 display treats a sprite as 5 pixels tall and 2 bytes (16 pixels) wide. It renders each scene line by line, pixel by pixel.

Since the first 512 bytes of the RAM are free, we can use this space to load this fontset at the start. It will acquire 5 x 2 x 16 = 160 bytes.

The following is an example of how a sprite for the fontset can be represented:

|          **"0"**         |                     **Binary**                    |          **Hex**         |
|:------------------------:|:-------------------------------------------------:|:------------------------:|
| **** *  * *  * *  * **** | 1111 0000 1001 0000 1001 0000 1001 0000 1111 0000 | 0xF0 0x90 0x90 0x90 0xF0 |
