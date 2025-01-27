use rand::random;
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
            (0, 0, 0, 0) => (),
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
                let x = n2 as usize;
                let y = n3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }
            (6, _, _, _) => {
                //set reg
                let x = n2 as usize;
                let nn = (op & 0xFF) as u8;

                self.v_reg[x] = nn;
            }
            (7, _, _, _) => {
                //add val
                let x = n2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }
            (8, _, _, _) => {
                match n4 {
                    0 => {
                        //set
                        let x = n2 as usize;
                        let y = n3 as usize;
                        self.v_reg[x] = self.v_reg[y]
                    }
                    1 => {
                        //OR
                        let x = n2 as usize;
                        let y = n3 as usize;

                        self.v_reg[x] |= self.v_reg[y];
                    }
                    2 => {
                        //AND
                        let x = n2 as usize;
                        let y = n3 as usize;

                        self.v_reg[x] &= self.v_reg[y];
                    }
                    3 => {
                        //XOR
                        let x = n2 as usize;
                        let y = n3 as usize;

                        self.v_reg[x] ^= self.v_reg[y];
                    }
                    4 => {
                        //ADD
                        let x = n2 as usize;
                        let y = n3 as usize;

                        let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                        let new_vf = if carry { 1 } else { 0 };

                        self.v_reg[x] = new_vx;
                        self.v_reg[0xF] = new_vf;
                    }
                    5 => {
                        //subtract reg1 from reg2
                        let x = n2 as usize;
                        let y = n3 as usize;

                        let (new_vx, carry) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                        let new_v_f = if carry { 1 } else { 0 };
                        self.v_reg[x] = new_vx;
                        self.v_reg[0xF] = new_v_f;
                    }
                    6 => {
                        //shift 1 to right
                        let x = n2 as usize;
                        let lost_bit = self.v_reg[x] & 1;
                        self.v_reg[x] >>= 1;

                        self.v_reg[0xF] = lost_bit;
                    }
                    7 => {
                        //subtract reg2 from reg1
                        let x = n2 as usize;
                        let y = n3 as usize;

                        let (new_vx, carry) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                        let new_v_f = if carry { 1 } else { 0 };
                        self.v_reg[x] = new_vx;
                        self.v_reg[0xF] = new_v_f;
                    }
                    0xE => {
                        let x = n2 as usize;
                        let lost_bit = (self.v_reg[x] >> 7) & 1;
                        self.v_reg[x] <<= 1;
                        self.v_reg[0xF] = lost_bit;
                    }
                    _ => {
                        unimplemented!("Unimplemented opcode = {}", op);
                    }
                }
            }
            (9, _, _, _) => {
                let x = n2 as usize;
                let y = n3 as usize;

                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2
                }
            }

            (0xA, _, _, _) => {
                //set index reg I
                let nnn = op & 0xFFF;
                self.idx_reg = nnn;
            }
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            }
            (0xC, _, _, _) => {
                //Rand & nn
                let x = n2 as usize;
                let nn = (op & 0xFF) as u8;
                let rand_num: u8 = random();
                self.v_reg[x] = nn & rand_num;
            }
            (0xD, _, _, _) => {
                //display and draw
                let x_coords = self.v_reg[n2 as usize] as u16;
                let y_coords = self.v_reg[n3 as usize] as u16;

                self.v_reg[0xF] = 0;

                let rows = n4;
                let mut flipped = false;

                for curr_y in 0..rows {
                    let address = self.idx_reg + curr_y;
                    let pixels = self.memory[address as usize];

                    for curr_x in 0..8 {
                        if pixels & (0b1000_0000 >> curr_x) != 0 {
                            let x = (curr_x + x_coords) as usize % SCREEN_WIDTH;
                            let y = (curr_y + y_coords) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;

                            flipped |= self.display[idx];
                            self.display[idx] ^= true;
                        }
                    }
                }
                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            }
            (0xE, _, _, _) => match (n3, n4) {
                (9, 0xE) => {
                    let x = n2 as usize;
                    if self.keys[self.v_reg[x] as usize] {
                        self.pc += 2
                    }
                }
                (0xA, 1) => {
                    let x = n2 as usize;
                    if !self.keys[self.v_reg[x] as usize] {
                        self.pc += 2
                    }
                }
                (_, _) => {
                    unimplemented!("Unimplemented opcode = {}", op);
                }
            },

            (0xF, _, _, _) => match (n3, n4) {
                //timers
                (0, 7) => {
                    let x = n2 as usize;
                    self.v_reg[x] = self.delay_timer as u8;
                }
                (1, 5) => {
                    let x = n2 as usize;
                    self.delay_timer = self.v_reg[x] as u16;
                }
                (1, 8) => {
                    let x = n2 as usize;
                    self.sound_timer = self.v_reg[x] as u16;
                }
                (1, 0xE) => {
                    let x = n2 as usize;
                    let vx = self.v_reg[x] as u16;
                    self.idx_reg = self.idx_reg.wrapping_add(vx);
                }

                (0, 0xA) => {
                    let x = n2 as usize;
                    let mut pressed = false;

                    for i in 0..self.keys.len() {
                        if self.keys[i] {
                            self.v_reg[x] = i as u8;
                            pressed = true;
                            break;
                        }
                    }
                    if !pressed {
                        self.pc -= 2;
                    }
                }
                (2, 9) => {
                    let x = n2 as usize;
                    let item = self.v_reg[x];
                    self.idx_reg = (item * 5) as u16;
                }
                (3, 3) => {
                    let x = n2 as usize;
                    let vx = self.v_reg[x] as i32;

                    let hundreds = ((vx / 100) % 10) as u8;

                    let tens = ((vx / 10) % 10) as u8;

                    let ones = (vx % 10) as u8;

                    self.memory[self.idx_reg as usize] = hundreds;
                    self.memory[(self.idx_reg + 1) as usize] = tens;
                    self.memory[(self.idx_reg + 2) as usize] = ones;
                }
                (5, 5) => {
                    let x = n2 as usize;
                    let i = self.idx_reg as usize;

                    for idx in 0..=x {
                        self.memory[i + idx] = self.memory[idx]
                    }
                }
                (6, 5) => {
                    let x = n2 as usize;
                    let i = self.idx_reg as usize;

                    for idx in 0..=x {
                        self.memory[idx] = self.memory[idx + i]
                    }
                }
                (_, _) => {
                    unimplemented!("Unimplemented opcode = {}", op);
                }
            },

            (_, _, _, _) => {
                unimplemented!("Unimplemented opcode = {}", op);
            }
        }
    }
}
