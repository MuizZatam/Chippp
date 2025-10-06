const RAM_SIZE: usize = 4096;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDRESS: u16 = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emulator {
    program_counter: u16,
    ram: [u8; RAM_SIZE],
    screen_buffer: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_registers: [u8; NUM_REGISTERS],
    index_register: u16,
    stack: [u16; STACK_SIZE],
    stack_pointer: u16,
    keys: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

impl Emulator {
    pub fn new() -> Self {
        let mut new_emulator = Self {
            program_counter: START_ADDRESS,
            ram: [0; RAM_SIZE],
            screen_buffer: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_registers: [0; NUM_REGISTERS],
            index_register: 0,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };
        new_emulator.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emulator
    }

    fn push(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    pub fn reset(&mut self) {
        self.program_counter = START_ADDRESS;
        self.ram = [0; RAM_SIZE];
        self.screen_buffer = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_registers = [0; NUM_REGISTERS];
        self.index_register = 0;
        self.stack = [0; STACK_SIZE];
        self.stack_pointer = 0;
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // fetch
        let opcode = self.fetch();
        // decode
        // execute
        self.execute(opcode);
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.program_counter as usize] as u16;
        let lower_byte = self.ram[(self.program_counter + 1) as usize] as u16;

        let opcode = (higher_byte << 8) | lower_byte;
        self.program_counter += 2;
        opcode
    }

    fn execute(&mut self, opcode: u16) {
        let digit1 = (opcode & 0xF000) >> 12;
        let digit2 = (opcode & 0x0F00) >> 8;
        let digit3 = (opcode & 0x00F0) >> 4;
        let digit4 = opcode & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // NOP (0000) - Does nothing, returns
            // Not described in CHIP8 standard but needed for timings and alignments
            (0, 0, 0, 0) => return,

            // CLS (00E0) - Clears the Screen Buffer
            // CHIP8 clears the screen every frame and redraws sprites line by line
            (0, 0, 0xE, 0) => {
                self.screen_buffer = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }

            // RET (00EE) - Returns from a subroutine
            // Behaves like a normal return from a function
            // This returns control flow to the entry point from which the subroutine was called
            // Achieved by pushing the current value of program counter to the stack before entering a subroutine
            // It is expected that during the execution of the called subroutine, the program counter updates as usual
            // Once the subroutine finishes execution, the value from the stack is popped and re-assigned to the
            // program counter to resume control flow from where the subroutine was called
            (0, 0, 0xE, 0xE) => {
                let return_address = self.pop();
                self.program_counter = return_address;
            }

            // JMP NNN (1NNN) - Jumps to address NNN
            // Shifts the program counter to NNN
            (1, _, _, _) => {
                let nnn = opcode & 0xFFF;
                self.program_counter = nnn;
            }

            // CALL NNN (2NNN) - Calls subroutine at NNN
            (2, _, _, _) => {
                let nnn = opcode & 0xFFF;
                self.push(self.program_counter);
                self.program_counter = nnn;
            }

            // SKIP VX == NN (3XNN) - Skips next instruction if VX is equal to NN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (opcode & 0xFF) as u8;
                if self.v_registers[x] == nn {
                    self.program_counter += 2;
                }
            }

            // SKIP VX != NN (4XNN) - Skips next instruction if VX is not equal to NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (opcode & 0xFF) as u8;
                if self.v_registers[x] != nn {
                    self.program_counter += 2;
                }
            }

            // SKIP VX == VY (5XY0) - Skips next instruction if VX is equal to VY
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_registers[x] == self.v_registers[y] {
                    self.program_counter += 2;
                }
            }

            // SET VX = NN - Sets the value of VX to NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (opcode & 0xFF) as u8;
                self.v_registers[x] = nn;
            }

            // INCREMENT VX, NN - Adds NN to VX
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (opcode & 0xFF) as u8;
                // Use wrapping addition to prevent panic due to overflow
                // It is expected that ROMs won't let this situation arise
                self.v_registers[x] = self.v_registers[x].wrapping_add(nn);
            }

            // SET VX = VY - Sets the value of VX to be that of VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_registers[x] = self.v_registers[y];
            }

            (_, _, _, _) => unimplemented!("Unimplemented Opcode: {opcode}"),
        }
    }

    pub fn tick_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // BEEP
            }
            self.sound_timer -= 1;
        }
    }
}
