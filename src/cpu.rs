use std::path::Path;

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
            program_counter: 0x200,
            pixel_state: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            keypad: [0; 16],
        }
    }

    pub fn load_program(&mut self, path: impl AsRef<Path>) {
        let file = std::fs::read(path).expect("Could not read file");
        assert!(file.len() < 4086 - 0x200);
        // TODO: This can be done faster
        for i in 0..file.len() {
            self.memory[i + 0x200] = file[i];
        }
    }

    pub fn fetch_opcode(&self) -> Opcode {
        let least_significant_byte = self.memory[self.program_counter as usize];
        let most_significant_byte = self.memory[self.program_counter as usize + 1];
        Opcode((least_significant_byte as u16) << 8 | most_significant_byte as u16)
    }

    pub fn decode_opcode(&mut self, opcode: Opcode) {
        let bytes = (
            (opcode.0 & 0xf000) >> 12 as u8,
            (opcode.0 & 0x0f00) >> 8 as u8,
            (opcode.0 & 0x00f0) >> 4 as u8,
            (opcode.0 & 0x000f) as u8,
        );
        println!("{:X}{:X}{:X}{:x}", bytes.0, bytes.1, bytes.2, bytes.3);
        match bytes {
            (0x0, 0x0, 0xe, 0x0) => self.opcode_00e0(),
            (0x0, 0x0, 0xe, 0xe) => self.opcode_00ee(),
            (0x1, _, _, _) => self.opcode_1nnn(opcode),
            (0x2, _, _, _) => self.opcode_2nnn(),
            (0x3, _, _, _) => self.opcode_3xnn(opcode),
            (0x4, _, _, _) => self.opcode_4xnn(opcode),
            (0x5, _, _, 0x0) => self.opcode_5xy0(opcode),
            (0x6, _, _, _) => self.opcode_6xnn(opcode),
            (0x7, _, _, _) => self.opcode_7xnn(opcode),
            (0x8, _, _, 0x0) => self.opcode_8xyo(opcode),
            (0x8, _, _, 0x1) => self.opcode_8xy1(opcode),
            (0x8, _, _, 0x2) => self.opcode_8xy2(opcode),
            (0x8, _, _, 0x3) => self.opcode_8xy3(opcode),
            (0x8, _, _, 0x4) => self.opcode_8xy4(opcode),
            (0x8, _, _, 0x5) => self.opcode_8xy5(opcode),
            (0x8, _, _, 0x6) => self.opcode_8xy6(opcode),
            (0x8, _, _, 0x7) => self.opcode_8xy7(opcode),
            (0x8, _, _, 0xe) => self.opcode_8xye(),
            (0x9, _, _, 0x0) => self.opcode_9xy0(opcode),
            (0xa, _, _, _) => self.opcode_annn(opcode),
            (0xb, _, _, _) => self.opcode_bnnn(opcode),
            (0xc, _, _, _) => self.opcode_cxnn(),
            (0xd, _, _, _) => self.opcode_dxyn(),
            (0xe, _, 0x9, 0xe) => self.opcode_ex9e(),
            (0xe, _, 0xa, 0x1) => self.opcode_exa1(),
            (0xf, _, 0x0, 0x7) => self.opcode_fx07(),
            (0xf, _, 0x0, 0xa) => self.opcode_fx0a(),
            (0xf, _, 0x1, 0x5) => self.opcode_fx15(),
            (0xf, _, 0x1, 0x8) => self.opcode_fx18(),
            (0xf, _, 0x1, 0xe) => self.opcode_fx1e(opcode),
            (0xf, _, 0x2, 0x9) => self.opcode_fx29(),
            (0xf, _, 0x3, 0x3) => self.opcode_fx33(),
            (0xf, _, 0x5, 0x5) => self.opcode_fx55(opcode),
            (0xf, _, 0x6, 0x5) => self.opcode_fx65(opcode),
            (0x0, _, _, _) => self.opcode_0nnn(),
            _ => panic!(
                "opcode not supported: {:X}{:X}{:X}{:x}",
                bytes.0, bytes.1, bytes.2, bytes.3
            ),
        }

        self.program_counter += 2;
    }

    // OpCode functions
    pub fn opcode_0nnn(&self) {
        println!("opcode_0nnn")
    }

    pub fn opcode_00e0(&mut self) {
        self.pixel_state = [0; 64 * 32];
    }

    pub fn opcode_00ee(&mut self) {
        println!("opcode_00ee")
    }

    pub fn opcode_1nnn(&mut self, opcode: Opcode) {
        let nnn = opcode.0 & 0x0fff;
        self.program_counter = nnn;
        println!("opcode_1nnn")
    }

    pub fn opcode_2nnn(&self) {
        println!("opcode_2nnn")
    }

    pub fn opcode_3xnn(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let nn = ((opcode.0 & 0x00ff) >> 4) as u8;
        if self.registers[x as usize] == nn {
            self.program_counter += 1;
        }
    }

    pub fn opcode_4xnn(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let nn = (opcode.0 & 0x00ff) as u8;
        if self.registers[x as usize] != nn {
            self.program_counter += 1;
        }
    }

    pub fn opcode_5xy0(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = ((opcode.0 & 0x00f0) >> 4) as u8;
        if self.registers[x as usize] == self.registers[y as usize] {
            self.program_counter += 1;
        }
    }

    pub fn opcode_6xnn(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let nn = (opcode.0 & 0x00ff) as u8;
        self.registers[x as usize] = nn;
    }

    pub fn opcode_7xnn(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let nn = (opcode.0 & 0x00ff) as u8;
        self.registers[x as usize] += nn;
    }

    pub fn opcode_8xyo(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = (opcode.0 & 0x00f0) as u8 >> 4;
        self.registers[x as usize] = self.registers[y as usize];
    }

    pub fn opcode_8xy1(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = (opcode.0 & 0x00f0) >> 4 as u8;
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
    }

    pub fn opcode_8xy2(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = (opcode.0 & 0x00f0) >> 4 as u8;
        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
    }

    pub fn opcode_8xy3(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = (opcode.0 & 0x00f0) >> 4 as u8;
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
    }

    pub fn opcode_8xy4(&self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = (opcode.0 & 0x00f0) >> 4 as u8;
        println!("opcode_8xy4")
    }

    pub fn opcode_8xy5(&self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = (opcode.0 & 0x00f0) >> 4 as u8;
        println!("opcode_8xy5")
    }

    pub fn opcode_8xy6(&self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = (opcode.0 & 0x00f0) >> 4 as u8;
        println!("opcode_8xy6")
    }

    pub fn opcode_8xy7(&self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = (opcode.0 & 0x00f0) >> 4 as u8;
        println!("opcode_8xy7")
    }

    pub fn opcode_8xye(&self) {
        println!("opcode_8xye")
    }

    pub fn opcode_9xy0(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8;
        let y = (opcode.0 & 0x00f0) >> 4 as u8;
        if self.registers[x as usize] != self.registers[y as usize] {
            self.program_counter += 1;
        }
    }

    pub fn opcode_annn(&mut self, opcode: Opcode) {
        let nnn = opcode.0 & 0x0fff;
        self.index_register = self.memory[nnn as usize] as u16;
    }

    pub fn opcode_bnnn(&mut self, opcode: Opcode) {
        let nnn = opcode.0 & 0x0fff;
        self.program_counter = self.registers[0] as u16 + nnn as u16;
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
    pub fn opcode_fx1e(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8 as u8;
        self.index_register += x;
    }
    pub fn opcode_fx29(&self) {
        println!("opcode_fx29")
    }
    pub fn opcode_fx33(&self) {
        println!("opcode_fx33")
    }

    pub fn opcode_fx55(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8 as u8;
        for i in 0..x + 1 {
            self.memory[self.index_register as usize] = self.registers[i as usize];
            self.index_register += 1;
        }
    }

    pub fn opcode_fx65(&mut self, opcode: Opcode) {
        let x = opcode.0 & 0x0f00 >> 8 as u8;
        for i in 0..x + 1 {
            self.registers[i as usize] = self.memory[self.index_register as usize];
            self.index_register += 1;
        }
    }
}
