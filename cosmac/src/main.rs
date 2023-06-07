struct CPU {
    position_in_memory: usize,
    memory: [u8; 0x1000],
    registers: [u8; 16],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        // loop {
            // let opcode = self.read_opcode();

            // let c = ((opcode & 0x0F00) >> 12) as u8;
            // let x = ((opcode & 0x00F0) >> 8) as u8;
            // let y = ((opcode & 0x000F) >> 4) as u8;
            // let d = ((opcode & 0x000F) >> 0) as u8;

            // match (c, x, y, d) {
            //     (0x8, _, _, 0x4) => self.add_xy(x, y),
            //     _ => todo!('opcode {:04x}', opcode),
            // }
        // }
    }
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014;
    cpu.registers[0] = 1;
    cpu.registers[1] = 2;
}