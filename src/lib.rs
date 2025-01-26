use std::collections::VecDeque;
const FONT_SET_SIZE: usize = 80;
const FONT_SET: [u8; FONT_SET_SIZE] = [
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
const RAM_SIZE: usize = 4096;
const NUM_REG: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADR: u16 = 0x200;

pub const SCREEN_HEIGHT: usize = 64;
pub const SCREEN_WIDTH: usize = 32;

pub struct Emulator {
    pc: u16,
    memory: [u8; RAM_SIZE],
    display: [bool; SCREEN_HEIGHT * SCREEN_WIDTH],
    idx_reg: u16,
    v_reg: [u8; NUM_REG],
    stack: VecDeque<i32>,
    sound_timer: u16,
    delay_timer: u16,
    keys: [bool; NUM_KEYS],
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    pub fn new() -> Self {
        let mut emu = Self {
            pc: START_ADR,
            memory: [0; RAM_SIZE],
            display: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            idx_reg: 0,
            v_reg: [0; NUM_REG],
            stack: VecDeque::from(vec![0; 16]),
            sound_timer: 0,
            delay_timer: 0,
            keys: [false; NUM_KEYS],
        };
        emu.memory[..FONT_SET_SIZE].copy_from_slice(&FONT_SET);

        emu
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.decode_and_execute(op);
    }

    pub fn tick_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                //sound
            }
            self.sound_timer -= 1;
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let top_byte = self.memory[self.pc as usize] as u16;
        let bot_byte = self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;

        (top_byte << 8) | (bot_byte) //returns opcode
    }
    pub fn decode_and_execute(&mut self, op: u16) {
        let n1 = (op >> 12) & 0x0F; //first nibble = bits 12-15
        let n2 = (op >> 8) & 0x0F; //second nibble = bits 8-11
        let n3 = (op >> 4) & 0x0F; //third nibble = bits 4-7
        let n4 = op & 0x0F; //fourth nibble = bits 0-3

        match (n1, n2, n3, n4) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => {
                self.display = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
            }
            (1, _, _, _) => {
                //jump
                let nnn = op & 0xFFF;
                self.pc += nnn;
            }
            (2, _, _, _) => {
                //call NNN
                let nnn = op & 0xFFF;
                self.stack.push_front(self.pc.into());
                self.pc = nnn
            }
            (3, _, _, _) => {
                let reg_to_use = n2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[reg_to_use] == nn {
                    self.pc += 2
                }
            }
            (4, _, _, _) => {
                let reg_to_use = n2 as usize;
                let nn = (op & 0xFF) as u8;

                if self.v_reg[reg_to_use] != nn {
                    self.pc += 2;
                }
            }
            (5, _, _, 0) => {
                let reg1 = n2 as usize;
                let reg2 = n3 as usize;
                if self.v_reg[reg1] == self.v_reg[reg2] {
                    self.pc += 2;
                }
            }
            (6, _, _, _) => {
                //set reg
                let reg = n2 as usize;
                let nn = (op & 0xFF) as u8;

                self.v_reg[reg] = nn;
            }
            (7, _, _, _) => {
                //add val
            }
            (0xA, _, _, _) => {
                //set index reg I
            }
            (0xD, n2, n3, _) => {
                //display and draw
            }

            (_, _, _, _) => {
                unimplemented!("Unimplemented opcode = {}", op)
            }
        }
    }
}
