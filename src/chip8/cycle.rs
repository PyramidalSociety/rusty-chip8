use super::*;

impl Chip8 {
    fn get_opcode(&mut self) -> u16 {
        let opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[self.pc as usize + 1] as u16);

        self.pc += 2;

        opcode
    }

    fn get_vars(&mut self) -> (u8, u8, u8, u8, u8, u8, u16) {
        let opcode = self.get_opcode();

        let n = opcode & 0x000F;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let c = (opcode & 0xF000) >> 12;
        let d = opcode & 0x000F;
        let kk = opcode & 0x00FF;
        let nnn = opcode & 0x0FFF;
        (n as u8, x as u8, y as u8, c as u8, d as u8, kk as u8, nnn)
    }

    pub fn run(&mut self) -> bool {
        let (n, x, y, c, d, kk, nnn) = self.get_vars();

        match (c, x, y, d) {
            (0, 0, 0xE, 0) => self.cls_00e0(),
            (0, 0, 0xE, 0xE) => self.ret_00ee(),
            (1, _, _, _) => self.jp_1nnn(nnn),
            (2, _, _, _) => self.call_2nnn(nnn),
            (3, _, _, _) => self.se_3xkk(x, kk),
            (4, _, _, _) => self.sne_4xkk(x, kk),
            (5, _, _, 0) => self.se_5xy0(x, y),
            (6, _, _, _) => self.ld_6xkk(x, kk),
            (7, _, _, _) => self.add_7xkk(x, kk),
            (8, _, _, 0) => self.ld_8xy0(x, y),
            (8, _, _, 1) => self.or_8xy1(x, y),
            (8, _, _, 2) => self.and_8xy2(x, y),
            (8, _, _, 3) => self.xor_8xy3(x, y),
            (8, _, _, 4) => self.add_8xy4(x, y),
            (8, _, _, 5) => self.sub_8xy5(x, y),
            (8, _, _, 6) => self.shr_8xy6(x),
            (8, _, _, 7) => self.subn_8xy7(x, y),
            (8, _, _, 0xE) => self.shl_8xye(x),
            (9, _, _, 0) => self.sne_9xy0(x, y),
            (0xA, _, _, _) => self.ld_annn(nnn),
            (0xB, _, _, _) => self.jp_v0_bnnn(nnn),
            (0xC, _, _, _) => self.rnd_cxkk(x, kk),
            (0xD, _, _, _) => self.drw_dxyn(x, y, n),
            (0xE, _, 9, 0xE) => self.skp_ex9e(x),
            (0xE, _, 0xA, 1) => self.sknp_exa1(x),
            (0xF, _, 0, 7) => self.ld_fx07(x),
            (0xF, _, 0, 0xA) => self.ld_fx0a(x),
            (0xF, _, 1, 5) => self.ld_fx15(x),
            (0xF, _, 1, 8) => self.ld_fx18(x),
            (0xF, _, 1, 0xE) => self.add_fx1e(x),
            (0xF, _, 2, 9) => self.ld_fx29(x),
            (0xF, _, 3, 3) => self.ld_fx33(x),
            (0xF, _, 5, 5) => self.ld_fx55(x),
            (0xF, _, 6, 5) => self.ld_fx65(x),
            _ => {}
        };

        c == 0xD
    }
}
