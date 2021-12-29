pub struct Opcode(u16);

pub struct CPU {
    registers: [u8; 16],
    memory: [u8; 4086],
    index_register: u16,
    program_counter: u16,
    pixel_state: [u8; 64 * 32],
    delay_timer: u16,
    sound_timer: u16,
    stack: [u16; 16],
    stack_pointer: u16,
    keypad: [u8; 16],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 4086],
            index_register: 0,
            program_counter: 0,
            pixel_state: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            keypad: [0; 16],
        }
    }

    pub fn fetch_opcode(&self) -> Opcode {
        let least_significant_byte = self.memory[self.program_counter as usize];
        let most_significant_byte = self.memory[self.program_counter as usize + 1];
        Opcode((least_significant_byte << 8) as u16 | most_significant_byte as u16)
    }

    pub fn decode_opcode(self, opcode: Opcode) {
        let bytes = (
            (opcode.0 & 0xf000) >> 12 as u8,
            (opcode.0 & 0x0f00) >> 8 as u8,
            (opcode.0 & 0x00f0) >> 4 as u8,
            (opcode.0 & 0x000f) as u8,
        );

        match bytes {
            (0x0, 0x0, 0xe, 0x0) => self.opcode_00e0(),
            (0x0, 0x0, 0xe, 0xe) => self.opcode_00ee(),
            (0x1, _, _, _) => self.opcode_1nnn(),
            (0x2, _, _, _) => self.opcode_2nnn(),
            (0x3, _, _, _) => self.opcode_3xnn(),
            (0x4, _, _, _) => self.opcode_4xnn(),
            (0x5, _, _, 0x0) => self.opcode_5xy0(),
            (0x6, _, _, _) => self.opcode_6xnn(),
            (0x7, _, _, _) => self.opcode_7xnn(),
            (0x8, _, _, 0x0) => self.opcode_8xyo(),
            (0x8, _, _, 0x1) => self.opcode_8xy1(),
            (0x8, _, _, 0x2) => self.opcode_8xy2(),
            (0x8, _, _, 0x3) => self.opcode_8xy3(),
            (0x8, _, _, 0x4) => self.opcode_8xy4(),
            (0x8, _, _, 0x5) => self.opcode_8xy5(),
            (0x8, _, _, 0x6) => self.opcode_8xy6(),
            (0x8, _, _, 0x7) => self.opcode_8xy7(),
            (0x8, _, _, 0xe) => self.opcode_8xye(),
            (0x9, _, _, 0x0) => self.opcode_9xy0(),
            (0xa, _, _, _) => self.opcode_annn(),
            (0xb, _, _, _) => self.opcode_bnnn(),
            (0xc, _, _, _) => self.opcode_cxnn(),
            (0xd, _, _, _) => self.opcode_dxyn(),
            (0xe, _, 0x9, 0xe) => self.opcode_ex9e(),
            (0xe, _, 0xa, 0x1) => self.opcode_exa1(),
            (0xf, _, 0x0, 0x7) => self.opcode_fx07(),
            (0xf, _, 0x0, 0xa) => self.opcode_fx0a(),
            (0xf, _, 0x1, 0x5) => self.opcode_fx15(),
            (0xf, _, 0x1, 0x8) => self.opcode_fx18(),
            (0xf, _, 0x1, 0xe) => self.opcode_fx1e(),
            (0xf, _, 0x2, 0x9) => self.opcode_fx29(),
            (0xf, _, 0x3, 0x3) => self.opcode_fx33(),
            (0xf, _, 0x5, 0x5) => self.opcode_fx55(),
            (0xf, _, 0x6, 0x5) => self.opcode_fx65(),
            (0x0, _, _, _) => self.opcode_0nnn(),
            _ => panic!("opcode not supported"),
        }
    }

    // OpCode functions
    pub fn opcode_0nnn(&self) {
        println!("opcode_0nnn")
    }
    pub fn opcode_00e0(&self) {
        println!("opcode_00e0")
    }
    pub fn opcode_00ee(&self) {
        println!("opcode_00ee")
    }
    pub fn opcode_1nnn(&self) {
        println!("opcode_1nnn")
    }
    pub fn opcode_2nnn(&self) {
        println!("opcode_2nnn")
    }
    pub fn opcode_3xnn(&self) {
        println!("opcode_3xnn")
    }
    pub fn opcode_4xnn(&self) {
        println!("opcode_4xnn")
    }
    pub fn opcode_5xy0(&self) {
        println!("opcode_5xy0")
    }
    pub fn opcode_6xnn(&self) {
        println!("opcode_6xnn")
    }
    pub fn opcode_7xnn(&self) {
        println!("opcode_7xnn")
    }
    pub fn opcode_8xyo(&self) {
        println!("opcode_8xyo")
    }
    pub fn opcode_8xy1(&self) {
        println!("opcode_8xy1")
    }
    pub fn opcode_8xy2(&self) {
        println!("opcode_8xy2")
    }
    pub fn opcode_8xy3(&self) {
        println!("opcode_8xy3")
    }
    pub fn opcode_8xy4(&self) {
        println!("opcode_8xy4")
    }
    pub fn opcode_8xy5(&self) {
        println!("opcode_8xy5")
    }
    pub fn opcode_8xy6(&self) {
        println!("opcode_8xy6")
    }
    pub fn opcode_8xy7(&self) {
        println!("opcode_8xy7")
    }
    pub fn opcode_8xye(&self) {
        println!("opcode_8xye")
    }
    pub fn opcode_9xy0(&self) {
        println!("opcode_9xy0")
    }
    pub fn opcode_annn(&self) {
        println!("opcode_annn")
    }
    pub fn opcode_bnnn(&self) {
        println!("opcode_bnnn")
    }
    pub fn opcode_cxnn(&self) {
        println!("opcode_cxnn")
    }
    pub fn opcode_dxyn(&self) {
        println!("opcode_dxyn")
    }
    pub fn opcode_ex9e(&self) {
        println!("opcode_ex9e")
    }
    pub fn opcode_exa1(&self) {
        println!("opcode_exa1")
    }
    pub fn opcode_fx07(&self) {
        println!("opcode_fx07")
    }
    pub fn opcode_fx0a(&self) {
        println!("opcode_fx0a")
    }
    pub fn opcode_fx15(&self) {
        println!("opcode_fx15")
    }
    pub fn opcode_fx18(&self) {
        println!("opcode_fx18")
    }
    pub fn opcode_fx1e(&self) {
        println!("opcode_fx1e")
    }
    pub fn opcode_fx29(&self) {
        println!("opcode_fx29")
    }
    pub fn opcode_fx33(&self) {
        println!("opcode_fx33")
    }
    pub fn opcode_fx55(&self) {
        println!("opcode_fx55")
    }
    pub fn opcode_fx65(&self) {
        println!("opcode_fx65")
    }
}
