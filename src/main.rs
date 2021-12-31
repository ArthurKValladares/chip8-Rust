mod cpu;
mod fontset;

use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.load_program("./test_opcode.ch8");
    loop {
        let opcode = cpu.fetch_opcode();
        cpu.decode_opcode(opcode);

        // TODO: Actual timer
        std::thread::sleep(std::time::Duration::new(0, 1));
    }
}
