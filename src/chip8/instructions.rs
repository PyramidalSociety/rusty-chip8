use super::*;

impl Chip8 {
    fn cls_00e0(&mut self) {
        self.video[..].fill(false);
    }

    fn ret_00ee(&mut self) {
        if self.sp == 0 {
            panic!("Stack pointer underflow");
        }

        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jp_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn call_2nnn(&mut self, nnn: u16) {
        if self.sp as usize == STACK_SIZE {
            panic!("Stack overflow");
        }

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn se_3xkk(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] == kk {
            self.pc += 2;
        }
    }

    fn sne_4xkk(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] != kk {
            self.pc += 2;
        }
    }

    fn se_5xy0(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.pc += 2;
        }
    }

    fn ld_6xkk(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = kk;
    }

    fn add_7xkk(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = self.registers[x as usize].overflowing_add(kk).0;
    }

    fn ld_8xy0(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn or_8xy1(&mut self, x: u8, y: u8) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    fn and_8xy2(&mut self, x: u8, y: u8) {
        self.registers[x as usize] &= self.registers[y as usize];
    }

    fn xor_8xy3(&mut self, x: u8, y: u8) {
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    fn add_8xy4(&mut self, x: u8, y: u8) {
        let sum = self.registers[x as usize].overflowing_add(self.registers[y as usize]);

        self.registers[0xF] = sum.1 as u8;
        self.registers[x as usize] = sum.0;
    }

    fn sub_8xy5(&mut self, x: u8, y: u8) {
        let diff = self.registers[x as usize].overflowing_sub(self.registers[y as usize]);

        self.registers[0xF] = diff.1 as u8;
        self.registers[x as usize] = diff.0;
    }

    fn shr_8xy6(&mut self, x: u8) {
        self.registers[0xF] = self.registers[x as usize] & 1;

        self.registers[x as usize] >>= 1;
    }

    fn subn_8xy7(&mut self, x: u8, y: u8) {
        let diff = self.registers[y as usize].overflowing_sub(self.registers[x as usize]);

        self.registers[0xF] = diff.1 as u8;
        self.registers[x as usize] = diff.0;
    }

    fn shl_8xye(&mut self, x: u8) {
        self.registers[0xF] = (self.registers[x as usize] & 0x80) >> 0x7;

        self.registers[x as usize] <<= 1;
    }

    fn ld_annn(&mut self, nnn: u16) {
        self.index = nnn;
    }

    fn jp_v0_bnnn(&mut self, nnn: u16) {
        self.pc = self.registers[0] as u16 + nnn;
    }

    fn rnd_cxkk(&mut self, x: u8, kk: u8) {
        let rnd: u8 = rand::random();

        self.registers[x as usize] = rnd & kk;
    }

    fn drw_dxyn(&mut self, x: u8, y: u8, n: u8) {
        let pos_x = self.registers[x as usize] as usize % VIDEO_WIDTH;
        let pos_y = self.registers[y as usize] as usize % VIDEO_HEIGHT;
        let height = n as usize;

        self.registers[0xF] = 0;

        for row in 0..height {
            if self.index as usize + row >= MEM_SIZE {
                break;
            }

            if row + pos_y >= VIDEO_HEIGHT {
                break;
            }

            let sprite = self.memory[self.index as usize + row];

            for col in 0..8 {
                if col + pos_x >= VIDEO_WIDTH {
                    break;
                }

                let sprite_px = (sprite & (0x80 >> col)) != 0;
                let screen_px = &mut self.video[(pos_y + row) * VIDEO_WIDTH + pos_x + col];

                if sprite_px {
                    continue;
                }

                if *screen_px {
                    self.registers[0xF] = 1;
                }

                *screen_px ^= sprite_px;
            }
        }
    }

    fn skp_ex9e(&mut self, x: u8) {
        let key = self.registers[x as usize];

        if key > 0xF {
            return;
        }

        if self.keypad[key as usize] {
            self.pc += 2;
        }
    }

    fn sknp_exa1(&mut self, x: u8) {
        let key = self.registers[x as usize];

        if key > 0xF {
            return;
        }

        if !self.keypad[key as usize] {
            self.pc += 2;
        }
    }

    fn ld_fx07(&mut self, x: u8) {
        self.registers[x as usize] = self.delay_timer;
    }

    fn ld_fx0a(&mut self, x: u8) {
        for i in 0..=16 {
            if i == 16 {
                self.pc -= 2;
            }

            if self.keypad[i] {
                self.registers[x as usize] = i as u8;
                break;
            }
        }
    }

    fn ld_fx15(&mut self, x: u8) {
        self.delay_timer = self.registers[x as usize];
    }

    fn ld_fx18(&mut self, x: u8) {
        self.sound_timer = self.registers[x as usize];
    }

    fn add_fx1e(&mut self, x: u8) {
        self.index += self.registers[x as usize] as u16;
    }

    fn ld_fx29(&mut self, x: u8) {
        let digit = self.registers[x as usize];

        self.index = FONTSET_START_ADDRESS as u16 + 5 * digit as u16;
    }

    fn ld_fx33(&mut self, x: u8) {
        let mut val = self.registers[x as usize];

        self.memory[self.index as usize + 2] = val % 10;
        val /= 10;

        self.memory[self.index as usize + 1] = val % 10;
        val /= 10;

        self.memory[self.index as usize] = val;
    }

    fn ld_fx55(&mut self, x: u8) {
        for i in 0..=self.registers[x as usize] {
            if self.index + i as u16 >= MEM_SIZE as u16 {
                break;
            }

            self.memory[self.index as usize + i as usize] = self.registers[i as usize];
        }
    }

    fn ld_fx65(&mut self, x: u8) {
        for i in 0..=self.registers[x as usize] {
            if self.index + i as u16 >= MEM_SIZE as u16 || i as usize >= NUM_REGISTERS {
                break;
            }

            self.registers[i as usize] = self.memory[self.index as usize + i as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cls_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        for i in 0..2048 {
            dummy.video[i] = true;
        }
        dummy.cls_00e0();

        for i in 0..2048 {
            assert_eq!(dummy.video[i], false);
        }
    }

    #[test]
    fn assignation_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_6xkk(2, 254);
        assert_eq!(dummy.registers[2], 254);

        dummy.add_7xkk(2, 1);
        assert_eq!(dummy.registers[2], 255);

        dummy.add_7xkk(2, 10);
        assert_eq!(dummy.registers[2], 9);

        assert_eq!(dummy.registers[0], 0);
    }

    #[test]
    fn instr_8xyd() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_6xkk(0, 10);
        dummy.ld_6xkk(1, 11);
        dummy.ld_8xy0(0, 1);
        assert_eq!(dummy.registers[0], dummy.registers[1]);
        assert_eq!(dummy.registers[0], 11);

        dummy.ld_6xkk(0, 8);
        dummy.ld_6xkk(1, 7);
        dummy.or_8xy1(0, 1);
        assert_eq!(dummy.registers[0], 15);

        dummy.ld_6xkk(0, 14);
        dummy.ld_6xkk(1, 7);
        dummy.and_8xy2(0, 1);
        assert_eq!(dummy.registers[0], 6);

        dummy.ld_6xkk(0, 14);
        dummy.ld_6xkk(1, 7);
        dummy.xor_8xy3(0, 1);
        assert_eq!(dummy.registers[0], 9);

        dummy.ld_6xkk(0, 255);
        dummy.ld_6xkk(1, 10);
        dummy.add_8xy4(0, 1);
        assert_eq!(dummy.registers[0], 9);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 32);
        dummy.ld_6xkk(1, 64);
        dummy.add_8xy4(0, 1);
        assert_eq!(dummy.registers[0], 96);
        assert_eq!(dummy.registers[0xF], 0);

        dummy.ld_6xkk(0, 10);
        dummy.ld_6xkk(1, 9);
        dummy.sub_8xy5(0, 1);
        assert_eq!(dummy.registers[0], 1);
        assert_eq!(dummy.registers[0xF], 0);

        dummy.ld_6xkk(0, 9);
        dummy.ld_6xkk(1, 10);
        dummy.sub_8xy5(0, 1);
        assert_eq!(dummy.registers[0], 255);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 7);
        dummy.shr_8xy6(0);
        assert_eq!(dummy.registers[0], 3);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 6);
        dummy.shr_8xy6(0);
        assert_eq!(dummy.registers[0], 3);
        assert_eq!(dummy.registers[0xF], 0);

        dummy.ld_6xkk(0, 9);
        dummy.ld_6xkk(1, 10);
        dummy.subn_8xy7(0, 1);
        assert_eq!(dummy.registers[0], 1);
        assert_eq!(dummy.registers[0xF], 0);

        dummy.ld_6xkk(0, 10);
        dummy.ld_6xkk(1, 9);
        dummy.subn_8xy7(0, 1);
        assert_eq!(dummy.registers[0], 255);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 255);
        dummy.shl_8xye(0);
        assert_eq!(dummy.registers[0], 254);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 127);
        dummy.shl_8xye(0);
        assert_eq!(dummy.registers[0], 254);
        assert_eq!(dummy.registers[0xF], 0);
    }

    #[test]
    fn ld_add_index_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_annn(1024);
        assert_eq!(dummy.index, 1024);

        dummy.ld_6xkk(0, 10);
        dummy.add_fx1e(0);
        assert_eq!(dummy.index, 1034);

        dummy.ld_fx29(0);
        assert_eq!(dummy.index, 130);
    }

    #[test]
    fn jp_v0_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_6xkk(0, 0xFF);
        dummy.jp_v0_bnnn(0x400);
        assert_eq!(dummy.pc, 0x4FF);
    }

    #[test]
    fn ld_dt_st_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_6xkk(0, 15);
        dummy.ld_fx15(0);
        dummy.ld_fx07(1);
        dummy.ld_fx18(1);

        assert_eq!(dummy.registers[0], 15);
        assert_eq!(dummy.registers[1], 15);
        assert_eq!(dummy.delay_timer, 15);
        assert_eq!(dummy.sound_timer, 15);
    }

    #[test]
    fn ld_bcd_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_6xkk(0, 128);
        dummy.ld_fx33(0);

        assert_eq!(dummy.memory[0], 1);
        assert_eq!(dummy.memory[1], 2);
        assert_eq!(dummy.memory[2], 8);
    }

    #[test]
    fn ld_reg_mem() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_6xkk(0, 1);
        dummy.ld_6xkk(1, 2);
        dummy.ld_6xkk(2, 3);
        dummy.ld_6xkk(3, 4);
        dummy.ld_6xkk(4, 5);

        dummy.ld_fx55(5);
        assert_eq!(dummy.registers[0], 1);
        assert_eq!(dummy.registers[1], 2);
        assert_eq!(dummy.registers[2], 3);
        assert_eq!(dummy.registers[3], 4);
        assert_eq!(dummy.registers[4], 5);
        assert_eq!(dummy.registers[5], 0);

        dummy.ld_6xkk(0, 128);
        dummy.ld_fx33(0);
        dummy.ld_fx65(2);
        assert_eq!(dummy.registers[0], 1);
        assert_eq!(dummy.registers[1], 2);
        assert_eq!(dummy.registers[2], 8);
    }
}
