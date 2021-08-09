use std::fs;
use std::path::Path;

use crate::opcodes;
use crate::opcodes::Mnemonic;
use crate::opcodes::AddressMode;

//--------------------------------------------------------------------------------

const PALETTE: [[u8; 4]; 64] = [
    [ 84,  84,  84, 255],  
    [  0,  30, 116, 255],  
    [  8,  16, 144, 255],  
    [ 48,   0, 136, 255],  
    [ 68,   0, 100, 255],  
    [ 92,   0,  48, 255],  
    [ 84,   4,   0, 255],  
    [ 60,  24,   0, 255],  
    [ 32,  42,   0, 255],  
    [  8,  58,   0, 255],  
    [  0,  64,   0, 255],  
    [  0,  60,   0, 255],  
    [  0,  50,  60, 255],  
    [  0,   0,   0, 255],  
    [  0,   0,   0, 255],  
    [  0,   0,   0, 255],
    [152, 150, 152, 255],  
    [  8,  76, 196, 255],  
    [ 48,  50, 236, 255],  
    [ 92,  30, 228, 255],  
    [136,  20, 176, 255],  
    [160,  20, 100, 255],  
    [152,  34,  32, 255],  
    [120,  60,   0, 255],  
    [ 84,  90,   0, 255],  
    [ 40, 114,   0, 255],  
    [  8, 124,   0, 255],  
    [  0, 118,  40, 255],  
    [  0, 102, 120, 255],  
    [  0,   0,   0, 255],  
    [  0,   0,   0, 255],  
    [  0,   0,   0, 255],
    [236, 238, 236, 255],  
    [ 76, 154, 236, 255],  
    [120, 124, 236, 255],  
    [176,  98, 236, 255],  
    [228,  84, 236, 255],  
    [236,  88, 180, 255],  
    [236, 106, 100, 255],  
    [212, 136,  32, 255],  
    [160, 170,   0, 255],  
    [116, 196,   0, 255],  
    [ 76, 208,  32, 255],  
    [ 56, 204, 108, 255],  
    [ 56, 180, 204, 255],  
    [ 60,  60,  60, 255],  
    [  0,   0,   0, 255],  
    [  0,   0,   0, 255],
    [236, 238, 236, 255],  
    [168, 204, 236, 255],  
    [188, 188, 236, 255],  
    [212, 178, 236, 255],  
    [236, 174, 236, 255],  
    [236, 174, 212, 255],  
    [236, 180, 176, 255],  
    [228, 196, 144, 255],  
    [204, 210, 120, 255],  
    [180, 222, 120, 255],  
    [168, 226, 144, 255],  
    [152, 226, 180, 255],  
    [160, 214, 228, 255],  
    [160, 162, 160, 255],  
    [  0,   0,   0, 255],  
    [  0,   0,   0, 255],
];

pub enum NametableMirrorMode {
    Horizontal, Vertical
}

pub struct RomState {
    prg_rom : Vec<u8>,
    chr_rom : Vec<u8>,
    nametable_mirror_mode : NametableMirrorMode
}

impl RomState {
    pub fn load(path: &Path) -> RomState {
        let content = fs::read(path).expect("Failed to read file");

        // TODO: parse more of the header
        let prg_size = (content[4] as usize) * 16384;
        let chr_size = (content[5] as usize) * 8192;

        let nametable_mirror_mode;
        if content[6] & 1 != 0 {
            nametable_mirror_mode = NametableMirrorMode::Horizontal;
        }
        else { //if content[6] & 2 != 0 {
            nametable_mirror_mode = NametableMirrorMode::Vertical;
        }
        /*else {
            panic!("Unknown mirror mode flags");
        }*/

        return RomState {
            prg_rom: content[16 .. 16+prg_size].to_vec(),
            chr_rom: content[16+prg_size .. 16+prg_size+chr_size].to_vec(),
            nametable_mirror_mode
        };
    }
}

//--------------------------------------------------------------------------------

#[derive(Default)]
pub struct CpuFlags {
    carry : bool, 
    zero : bool, 
    interrupt_disable : bool, 
    decimal : bool, 
    overflow : bool, 
    negative : bool,
}

struct SpriteData {
    index: usize,
    y: u8,
    tile: u8,
    attributes: u8,
    x: u8
}

#[derive(Default)]
pub struct JoypadState {
    pub buttons: [bool; 8],
    pub next_button: usize
}

pub struct EmuState {
    pub rom_state : RomState,
    ram : [u8; 2048],
    cycle_count : u64,
    last_ppu_cycle : u64,
    pub program_counter: u16,
    pub stack_pointer: u8,
    cpu_flags: CpuFlags,
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,

    pub joypad1: JoypadState,

    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    pub ppu_y: i32,
    pub ppu_x: i32,
    ppu_odd_frame: bool,
    ppu_address: u16,
    ppu_data_read_buffer: u8,
    ppu_nametable_ram: [[u8; 1024]; 2],
    ppu_palette_ram: [u8; 32],
    ppu_oam_ram: [u8; 256],
    ppu_scroll_x: u8,
    ppu_scroll_y: u8,
    ppu_scroll_latch: bool,
    ppu_nmi_flag: bool,

    pub frame_buffer: Vec<u8>,
    sprites_this_scanline: Vec<SpriteData>
}

impl EmuState {
    pub fn new(rom_path: &Path) -> EmuState {
        let mut result = EmuState {
            rom_state: RomState::load(rom_path),
            ram: [0; 2048],
            cycle_count: 0,
            last_ppu_cycle: 0,
            program_counter: 0x8000,
            stack_pointer: 0xFD,
            cpu_flags: Default::default(),
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            joypad1: Default::default(),
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            ppu_x: 0,
            ppu_y: -1,
            ppu_odd_frame: false,
            ppu_address: 0,
            ppu_data_read_buffer: 0,
            ppu_nametable_ram: [[0; 1024]; 2],
            ppu_palette_ram: [0; 32],
            ppu_oam_ram: [0; 256],
            ppu_scroll_x: 0,
            ppu_scroll_y: 0,
            ppu_scroll_latch: false,
            ppu_nmi_flag: false,
            frame_buffer: Vec::new(),
            sprites_this_scanline: Vec::new()
        };
        result.cpu_flags.interrupt_disable = true;
        result.frame_buffer.resize(256 * 240 * 4, 0);
        return result;
    }

    fn read_byte(&mut self, address: u16) -> Result<u8, &'static str> {
        match address {
            // internal RAM
            0x0000 ..= 0x07FF => Ok(self.ram[address as usize]),

            // wrapping for internal RAM
            0x0800 ..= 0x1FFF => self.read_byte(address & 0x7FF),
            
            // PPU_STATUS
            0x2002 => { 
                self.update_ppu();
                let result = self.ppu_status;
                self.ppu_status &= 0x7F; // clear VBLANK latch
                self.ppu_address = 0; // clear address latch
                self.ppu_scroll_latch = false;
                return Ok(result);
            }

            // PPU_DATA
            0x2007 => {
                self.update_ppu();

                // https://wiki.nesdev.com/w/index.php?title=PPU_registers#The_PPUDATA_read_buffer_.28post-fetch.29
                // Palette data is not buffered, all other data is
                if self.ppu_address >= 0x3F00 && self.ppu_address <= 0x3FFF {
                    self.ppu_data_read_buffer = self.read_ppu_byte(self.ppu_address);
                }

                let result = self.ppu_data_read_buffer;
                self.ppu_data_read_buffer = self.read_ppu_byte(self.ppu_address);

                if self.ppu_ctrl & 4 != 0 {
                    self.ppu_address += 32;
                } else {
                    self.ppu_address += 1;
                }

                return Ok(result);
            }

            // Joypad registers
            0x4016 => {
                let result = if self.joypad1.buttons[self.joypad1.next_button] { 1u8 } else { 0u8 };
                self.joypad1.next_button = (self.joypad1.next_button + 1) % 8;
                return Ok(result);
            }
            
            0x4017 => {
                // TODO
                Ok(0)
            }

            // TODO: registers

            // wrapping for PPU registers
            0x2008 ..= 0x3FFF => self.read_byte(address & 0x2007),

            // program ROM
            0x8000 ..= 0xFFFF => {
                let index = (address - 0x8000) as usize;
                Ok(self.rom_state.prg_rom[index % self.rom_state.prg_rom.len()])
            },

            // catch-all
            _ => Err("Invalid memory address")
        }
    }

    fn read_next_program_byte(&mut self) -> u8 {
        let result = self.read_byte(self.program_counter);
        self.program_counter += 1;
        return result.expect("Failed to read next program byte");
    }

    fn write_byte(&mut self, address: u16, value: u8) -> Result<(), &'static str> {
        match address {
            // internal RAM
            0x0000 ..= 0x07FF => {
                self.ram[address as usize] = value;
                Ok(())
            },

            // wrapping for internal RAM
            0x0800 ..= 0x1FFF => self.write_byte(address & 0x7FF, value),
            
            // PPU_CTRL
            0x2000 => {
                self.update_ppu();
                self.ppu_ctrl = value;
                Ok(())
            }

            // PPU_MASK
            0x2001 => {
                self.update_ppu();
                self.ppu_mask = value;
                Ok(())
            }

            // PPU_OAM_ADDR
            0x2003 => {
                self.update_ppu();
                if value != 0 {
                    Err("Nonzero OAM address not implemented")
                } else {
                    Ok(())
                }
            }

            // PPU_SCROLL
            0x2005 => {
                self.update_ppu();
                if !self.ppu_scroll_latch {
                    self.ppu_scroll_x = value;
                } else {
                    self.ppu_scroll_y = value;
                }
                self.ppu_scroll_latch = !self.ppu_scroll_latch;
                Ok(())
            }

            // PPU_ADDR
            0x2006 => {
                self.update_ppu();
                self.ppu_address = self.ppu_address << 8 | (value as u16);
                Ok(())
            }

            // PPU_DATA
            0x2007 => {
                self.update_ppu();
                self.write_ppu_byte(self.ppu_address, value);
                if self.ppu_ctrl & 4 != 0 {
                    self.ppu_address += 32;
                }
                else {
                    self.ppu_address += 1;
                }
                Ok(())
            }

            // OAM_DMA
            0x4014 => {
                let base_address = (value as u16) << 8;
                for index in 0..256 {
                    self.ppu_oam_ram[index] = self.read_byte(base_address + index as u16).expect("Bad memory access");
                }

                if self.cycle_count % 2 == 1 {
                    self.cycle_count += 1;
                }
                self.cycle_count += 513;
                self.update_ppu();

                Ok(())
            }

            // APU registers
            0x4000 ..= 0x4013 | 0x4015 => {
                // TODO
                Ok(())
            }

            // Joypad registers
            0x4016 => {
                self.joypad1.next_button = 0;
                Ok(())
            }
            
            0x4017 => {
                // TODO
                Ok(())
            }

            // TODO: registers

            // wrapping for PPU registers
            0x2008 ..= 0x3FFF => self.write_byte(address & 0x2007, value),

            // catch-all
            _ => Err("Invalid memory address")
        }
    }

    fn set_zero_negative_flags(&mut self, value: u8) {
        self.cpu_flags.zero = value == 0;
        self.cpu_flags.negative = value & 0x80 != 0;
    }

    fn push_to_stack(&mut self, value: u8) {
        self.ram[0x100 | self.stack_pointer as usize] = value;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn pull_from_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        return self.ram[0x100 | self.stack_pointer as usize];
    }

    fn get_operand_address(&mut self, address_mode: &AddressMode) -> u16 {
        match address_mode {
            AddressMode::ABS | AddressMode::ABX | AddressMode::ABY => {
                self.cycle_count += 2;
                let lo_byte = self.read_next_program_byte();
                let hi_byte = self.read_next_program_byte();
                let base_address = ((hi_byte as u16) << 8) | (lo_byte as u16);
                let offset = match address_mode {
                    AddressMode::ABS => 0,
                    AddressMode::ABX => self.reg_x,
                    AddressMode::ABY => self.reg_y,
                    _ => unreachable!()
                };
                let address = base_address.wrapping_add(offset as u16);
                if (base_address >> 8) != (address >> 8) {
                    self.cycle_count += 1; // page boundary crossed
                }
                return address;
            }

            AddressMode::IDX => {
                self.cycle_count += 4;
                let zp_address = self.read_next_program_byte().wrapping_add(self.reg_x) as u16;
                let lo_byte = self.read_byte( zp_address         ).expect("Invalid memory read");
                let hi_byte = self.read_byte((zp_address+1) % 256).expect("Invalid memory read");
                return ((hi_byte as u16) << 8) | (lo_byte as u16);
            },

            AddressMode::IDY => {
                let zp_address = self.read_next_program_byte() as u16;
                let lo_byte = self.read_byte( zp_address         ).expect("Invalid memory read");
                let hi_byte = self.read_byte((zp_address+1) % 256).expect("Invalid memory read");
                let base_address = ((hi_byte as u16) << 8) | (lo_byte as u16);
                let address = base_address.wrapping_add(self.reg_y as u16);

                self.cycle_count += 3;
                if (base_address >> 8) != (address >> 8) {
                    self.cycle_count += 1; // page boundary crossed
                }
                return address;
            },

            AddressMode::IMM => {
                self.program_counter += 1;
                return self.program_counter - 1;
            },

            AddressMode::IMP => 0, // not used

            AddressMode::IND => {
                self.cycle_count += 2;
                let lo_byte = self.read_next_program_byte();
                let hi_byte = self.read_next_program_byte();
                let base_address = ((hi_byte as u16) << 8) | (lo_byte as u16);
                let lo_byte_2 = self.read_byte(base_address).expect("Invalid memory read");
                let hi_index = ((base_address+1) & 0xFF) | (base_address & 0xFF00); // page wrapping
                let hi_byte_2 = self.read_byte(hi_index).expect("Invalid memory read");
                return ((hi_byte_2 as u16) << 8) | (lo_byte_2 as u16);
            },

            AddressMode::REL => {
                let offset = self.read_next_program_byte() as i8;
                let new_pc = (self.program_counter as i32) + (offset as i32);
                return new_pc as u16;
            },

            AddressMode::ZPG => {
                self.cycle_count += 1;
                return self.read_next_program_byte() as u16;
            },

            AddressMode::ZPX | AddressMode::ZPY => {
                self.cycle_count += 2;
                let base_address = self.read_next_program_byte();
                let offset = match address_mode {
                    AddressMode::ZPX => self.reg_x,
                    AddressMode::ZPY => self.reg_y,
                    _ => unreachable!()
                };
                return base_address.wrapping_add(offset) as u16;
            },
        }
    }

    pub fn get_flags_as_u8(&self) -> u8 {
        let mut result: u8 = 0x20;
        if self.cpu_flags.carry             { result |= 0x01 }
        if self.cpu_flags.zero              { result |= 0x02 }
        if self.cpu_flags.interrupt_disable { result |= 0x04 }
        if self.cpu_flags.decimal           { result |= 0x08 }
        if self.cpu_flags.overflow          { result |= 0x40 }
        if self.cpu_flags.negative          { result |= 0x80 }
        return result;
    }

    fn set_flags_as_u8(&mut self, value: u8) {
        self.cpu_flags.carry             = value & 0x01 != 0;
        self.cpu_flags.zero              = value & 0x02 != 0;
        self.cpu_flags.interrupt_disable = value & 0x04 != 0;
        self.cpu_flags.decimal           = value & 0x08 != 0;
        self.cpu_flags.overflow          = value & 0x40 != 0;
        self.cpu_flags.negative          = value & 0x80 != 0;
    }

    fn jump_to_interrupt(&mut self, address: u16) {
        // Push return address to stack
        self.push_to_stack((self.program_counter >> 8) as u8);
        self.push_to_stack((self.program_counter & 0xFF) as u8);

        // Push processor flags to stack
        self.push_to_stack(self.get_flags_as_u8());

        // Jump
        self.program_counter = address;
    }

    pub fn run_to_next_nmi(&mut self) {
        if self.ppu_nmi_flag {
            self.ppu_nmi_flag = false;
            let interrupt_lo = self.read_byte(0xFFFA).expect("Failed to read NMI address");
            let interrupt_hi = self.read_byte(0xFFFB).expect("Failed to read NMI address");
            let interrupt_address = (interrupt_lo as u16) | ((interrupt_hi as u16) << 8);
            self.jump_to_interrupt(interrupt_address);
        }

        loop {
            self.run_one_instruction();
            if self.ppu_nmi_flag { return }
        }
    }

    pub fn run_one_instruction(&mut self) {
        let debug_print = false;

        let instruction = self.read_next_program_byte();
        let opcode = opcodes::decode(instruction);

        if debug_print {
            print!("{:04X}: {:02X} {:?}", self.program_counter - 1, instruction, opcode);
        }

        self.cycle_count += 2;

        let operand_address = self.get_operand_address(&opcode.address_mode);

        if debug_print {
            print!("  {:04X}", operand_address);
            match operand_address {
                0x0000 ..= 0x1FFF | 0x8000 ..= 0xFFFF => println!(" ({:02X})", self.read_byte(operand_address).expect("wtf")),
                _ => println!("")
            }
        }

        match opcode.mnemonic {
            Mnemonic::XXX => {} //panic!("Invalid opcode"),

            Mnemonic::ADC => {
                let other = self.read_byte(operand_address).expect("Bad memory access");
                let (result_1, overflow_1) = self.reg_a.overflowing_add(other);
                let (result, overflow) = result_1.overflowing_add(if self.cpu_flags.carry {1} else {0});
                self.set_zero_negative_flags(result);

                // http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html#:~:text=(M%5Eresult)%26(N%5Eresult)%260x80
                self.cpu_flags.overflow = (self.reg_a ^ result) & (other ^ result) & 0x80 != 0;
                self.cpu_flags.carry = overflow_1 || overflow;
                self.reg_a = result;
            }

            Mnemonic::SBC => {
                let other = self.read_byte(operand_address).expect("Bad memory access");
                let (result_1, overflow_1) = self.reg_a.overflowing_sub(other);
                let (result, overflow) = result_1.overflowing_sub(if self.cpu_flags.carry {0} else {1});
                self.set_zero_negative_flags(result);
                self.cpu_flags.overflow = (self.reg_a ^ result) & ((255 - other) ^ result) & 0x80 != 0;
                self.cpu_flags.carry = !(overflow_1 || overflow);
                self.reg_a = result;
            }
            
            Mnemonic::AND => {
                let other = self.read_byte(operand_address).expect("Invalid operand");
                self.reg_a &= other;
                self.set_zero_negative_flags(self.reg_a);
            }

            Mnemonic::BCC | Mnemonic::BCS | Mnemonic::BEQ | Mnemonic::BMI | Mnemonic::BNE | Mnemonic::BPL | Mnemonic::BVC | Mnemonic::BVS => {
                let condition = match opcode.mnemonic {
                    Mnemonic::BCC => !self.cpu_flags.carry,
                    Mnemonic::BCS => self.cpu_flags.carry,
                    Mnemonic::BEQ => self.cpu_flags.zero,
                    Mnemonic::BMI => self.cpu_flags.negative,
                    Mnemonic::BNE => !self.cpu_flags.zero,
                    Mnemonic::BPL => !self.cpu_flags.negative,
                    Mnemonic::BVC => !self.cpu_flags.overflow,
                    Mnemonic::BVS => self.cpu_flags.overflow,
                    _ => unreachable!()
                };

                if condition {
                    self.cycle_count += 1;
                    if (self.program_counter >> 8) != (operand_address >> 8) {
                        self.cycle_count += 1; // crossed page boundary
                    }

                    self.program_counter = operand_address;
                }
            }

            Mnemonic::BIT => {
                let other = self.read_byte(operand_address).expect("Invalid operand");
                self.cpu_flags.zero = self.reg_a & other == 0;
                self.cpu_flags.overflow = other & 0x40 != 0;
                self.cpu_flags.negative = other & 0x80 != 0;
            }

            Mnemonic::BRK => {
                let interrupt_lo = self.read_byte(0xFFFE).expect("Failed to read NMI address");
                let interrupt_hi = self.read_byte(0xFFFF).expect("Failed to read NMI address");
                let interrupt_address = (interrupt_lo as u16) | ((interrupt_hi as u16) << 8);
                self.jump_to_interrupt(interrupt_address);
            }

            Mnemonic::CLC => {
                self.cpu_flags.carry = false;
            }

            Mnemonic::CLD => {
                self.cpu_flags.decimal = false;
            }

            Mnemonic::CLI => {
                self.cpu_flags.interrupt_disable = false;
            }

            Mnemonic::CLV => {
                self.cpu_flags.overflow = false;
            }

            Mnemonic::CMP | Mnemonic::CPX | Mnemonic::CPY => {
                let reg = match opcode.mnemonic {
                    Mnemonic::CMP => self.reg_a,
                    Mnemonic::CPX => self.reg_x,
                    Mnemonic::CPY => self.reg_y,
                    _ => unreachable!()
                };

                let value = self.read_byte(operand_address).expect("Bad memory access");

                self.cpu_flags.carry = reg >= value;
                self.cpu_flags.zero = reg == value;
                self.cpu_flags.negative = reg.wrapping_sub(value) & 0x80 != 0;
            }

            Mnemonic::DEC => {
                let value = self.read_byte(operand_address).expect("Bad memory access").wrapping_sub(1);
                self.write_byte(operand_address, value).expect("Bad memory access");
                self.set_zero_negative_flags(value);
            }

            Mnemonic::DEX => {
                self.reg_x = self.reg_x.wrapping_sub(1);
                self.set_zero_negative_flags(self.reg_x);
            }

            Mnemonic::DEY => {
                self.reg_y = self.reg_y.wrapping_sub(1);
                self.set_zero_negative_flags(self.reg_y);
            }

            Mnemonic::EOR => {
                let other = self.read_byte(operand_address).expect("Invalid operand");
                self.reg_a ^= other;
                self.set_zero_negative_flags(self.reg_a);
            }

            Mnemonic::INC => {
                let value = self.read_byte(operand_address).expect("Bad memory access").wrapping_add(1);
                self.write_byte(operand_address, value).expect("Bad memory access");
                self.set_zero_negative_flags(value);
            }

            Mnemonic::INX => {
                self.reg_x = self.reg_x.wrapping_add(1);
                self.set_zero_negative_flags(self.reg_x);
            }

            Mnemonic::INY => {
                self.reg_y = self.reg_y.wrapping_add(1);
                self.set_zero_negative_flags(self.reg_y);
            }

            Mnemonic::JMP => {
                self.cycle_count += 1;
                self.program_counter = operand_address;
                self.update_ppu();
            }

            Mnemonic::JSR => {
                let return_address = self.program_counter - 1;

                // Push return address to stack
                self.push_to_stack((return_address >> 8) as u8);
                self.push_to_stack((return_address & 0xFF) as u8);

                self.program_counter = operand_address;
                self.cycle_count += 2; // already had +2 default and +2 for ABS address mode
            }

            Mnemonic::LDA => {
                self.reg_a = self.read_byte(operand_address).expect("Bad memory access");
                self.set_zero_negative_flags(self.reg_a);
            }

            Mnemonic::LDX => {
                self.reg_x = self.read_byte(operand_address).expect("Bad memory access");
                self.set_zero_negative_flags(self.reg_x);
            }

            Mnemonic::LDY => {
                self.reg_y = self.read_byte(operand_address).expect("Bad memory access");
                self.set_zero_negative_flags(self.reg_y);
            }

            Mnemonic::NOP => {
                // Do nothing
            }

            Mnemonic::ORA => {
                let other = self.read_byte(operand_address).expect("Invalid operand");
                self.reg_a |= other;
                self.set_zero_negative_flags(self.reg_a);
            }

            Mnemonic::PHA => {
                self.push_to_stack(self.reg_a);
                self.cycle_count += 1;
            }

            Mnemonic::PHP => {
                let flags = self.get_flags_as_u8() | 0x10;
                self.push_to_stack(flags);
            }

            Mnemonic::PLA => {
                self.reg_a = self.pull_from_stack();
                self.set_zero_negative_flags(self.reg_a);
                self.cycle_count += 2;
            }

            Mnemonic::PLP => {
                let flags = self.pull_from_stack();
                self.set_flags_as_u8(flags);
            }

            Mnemonic::ASL | Mnemonic::LSR | Mnemonic::ROL | Mnemonic::ROR => {
                let old_value = match opcode.address_mode {
                    AddressMode::IMP => self.reg_a,
                    _ => self.read_byte(operand_address).expect("Bad memory access")
                };
                let carry_bit: u8 = if self.cpu_flags.carry { 1 } else { 0 };
                let new_value = match opcode.mnemonic {
                    Mnemonic::ASL => old_value << 1,
                    Mnemonic::LSR => old_value >> 1,
                    Mnemonic::ROL => old_value << 1 | carry_bit,
                    Mnemonic::ROR => old_value >> 1 | carry_bit << 7,
                    _ => unreachable!()
                };
                
                self.set_zero_negative_flags(new_value);
                self.cpu_flags.carry = match opcode.mnemonic {
                    Mnemonic::ASL | Mnemonic::ROL => old_value & 0x80 != 0,
                    Mnemonic::LSR | Mnemonic::ROR => old_value & 0x01 != 0,
                    _ => unreachable!()
                };

                match opcode.address_mode {
                    AddressMode::IMP => self.reg_a = new_value,
                    _ => self.write_byte(operand_address, new_value).expect("Bad memory access")
                };
            }

            Mnemonic::RTI => {
                let flags = self.pull_from_stack();
                self.set_flags_as_u8(flags);

                // Pull return address from stack
                let return_lo = self.pull_from_stack();
                let return_hi = self.pull_from_stack();
                let return_address = (return_lo as u16) | ((return_hi as u16) << 8);

                self.program_counter = return_address;
                self.cycle_count += 4;
            }

            Mnemonic::RTS => {
                // Pull return address from stack
                let return_lo = self.pull_from_stack();
                let return_hi = self.pull_from_stack();
                let return_address = (return_lo as u16) | ((return_hi as u16) << 8);

                self.program_counter = return_address + 1;
                self.cycle_count += 4;
            }

            Mnemonic::SEC => {
                self.cpu_flags.carry = true;
            }

            Mnemonic::SED => {
                self.cpu_flags.decimal = true;
            }

            Mnemonic::SEI => {
                self.cpu_flags.interrupt_disable = true;
            }

            Mnemonic::STA => {
                self.write_byte(operand_address, self.reg_a).expect("Bad memory access");
            }

            Mnemonic::STX => {
                self.write_byte(operand_address, self.reg_x).expect("Bad memory access");
            }

            Mnemonic::STY => {
                self.write_byte(operand_address, self.reg_y).expect("Bad memory access");
            }

            Mnemonic::TAX => {
                self.reg_x = self.reg_a;
                self.set_zero_negative_flags(self.reg_x);
            }

            Mnemonic::TAY => {
                self.reg_y = self.reg_a;
                self.set_zero_negative_flags(self.reg_y);
            }

            Mnemonic::TSX => {
                self.reg_x = self.stack_pointer;
                self.set_zero_negative_flags(self.reg_x);
            }

            Mnemonic::TXA => {
                self.reg_a = self.reg_x;
                self.set_zero_negative_flags(self.reg_a);
            }

            Mnemonic::TXS => {
                self.stack_pointer = self.reg_x;
            }

            Mnemonic::TYA => {
                self.reg_a = self.reg_y;
                self.set_zero_negative_flags(self.reg_a);
            }
        }
    }

    // ----------------------------------------------------------------------------

    fn read_ppu_byte(&self, address : u16) -> u8 {
        match address {
            // Pattern tables
            0x0000 ..= 0x1FFF => self.rom_state.chr_rom[address as usize],

            // Nametables
            0x2000 ..= 0x2FFF => {
                let nametable_index = ((address & 0x0C00) >> 10) as usize;
                assert!(nametable_index < 4);
                let mirrored_index = match self.rom_state.nametable_mirror_mode {
                    NametableMirrorMode::Horizontal => nametable_index % 2,
                    NametableMirrorMode::Vertical => nametable_index / 2
                };
                self.ppu_nametable_ram[mirrored_index][(address & 0x03FF) as usize]
            }

            // Wrapping
            0x3000 ..= 0x3EFF => self.read_ppu_byte(address - 0x1000),

            // Palette
            0x3F00 ..= 0x3F1F => match address { 
                0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => self.ppu_palette_ram[(address - 0x3F10) as usize],
                _ => self.ppu_palette_ram[(address - 0x3F00) as usize]
            }

            // Wrapping
            0x3F20 ..= 0x3FFF => self.read_ppu_byte(address & 0x3F1F),

            // Default
            _ => panic!("Invalid address")
        }
    }

    fn write_ppu_byte(&mut self, address : u16, value : u8) {
        match address {
            // Nametables
            0x2000 ..= 0x2FFF => {
                let nametable_index = ((address & 0x0C00) >> 10) as usize;
                assert!(nametable_index < 4);
                let mirrored_index = match self.rom_state.nametable_mirror_mode {
                    NametableMirrorMode::Horizontal => nametable_index % 2,
                    NametableMirrorMode::Vertical => nametable_index / 2
                };
                self.ppu_nametable_ram[mirrored_index][(address & 0x03FF) as usize] = value;
            }

            // Wrapping
            0x3000 ..= 0x3EFF => self.write_ppu_byte(address - 0x1000, value),

            // Palette
            0x3F00 ..= 0x3F1F => match address { 
                    0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => self.ppu_palette_ram[(address - 0x3F10) as usize] = value,
                    _ => self.ppu_palette_ram[(address - 0x3F00) as usize] = value
                }

            // Wrapping
            0x3F20 ..= 0x3FFF => self.write_ppu_byte(address & 0x3F1F, value),

            // Default
            _ => panic!("Invalid address")
        }
    }

    fn get_pixel_from_pattern_table(&self, pt_base: u16, tile_index: u8, x: u8, y: u8) -> u8 {
        let plane_0_address = pt_base + ((tile_index as u16) << 4) + y as u16;
        let plane_0_row = self.read_ppu_byte(plane_0_address);
        let plane_1_row = self.read_ppu_byte(plane_0_address + 8);
        let bit_0 = (plane_0_row >> (7-x)) & 1;
        let bit_1 = (plane_1_row >> (7-x)) & 1;
        return bit_0 | (bit_1 << 1);
    }

    fn update_ppu(&mut self) {
        let n_cycles = (self.cycle_count - self.last_ppu_cycle) * 3;
        self.last_ppu_cycle = self.cycle_count;
    
        for _ in 0 .. n_cycles {

            // Cycle skipping on odd frames
            if self.ppu_y == -1 && self.ppu_x == 0 {
                if self.ppu_odd_frame && (self.ppu_mask & 0x18) != 0 {
                    self.ppu_x += 1;
                }
                self.ppu_odd_frame = !self.ppu_odd_frame;
            }

            match (self.ppu_x, self.ppu_y) {
                // Pre-render
                (1, -1) => {
                    // Clear sprite 0 hit
                    self.ppu_status &= !0x40;

                    // Reset nametable index
                    // This is probably not the right way to do things but 🤷‍♂️
                    self.ppu_ctrl &= !0x03;
                }

                // Sprite scanning
                (0, 0 ..= 239) => {
                    self.sprites_this_scanline.clear();

                    'check_sprites: for i in 0..64 {
                        let sprite_y = (self.ppu_oam_ram[i * 4] as i32) + 1;
                        if self.ppu_y >= sprite_y && self.ppu_y < sprite_y + 8 {
                            self.sprites_this_scanline.push(SpriteData {
                                index: i,
                                y: self.ppu_oam_ram[i * 4],
                                tile: self.ppu_oam_ram[i * 4 + 1],
                                attributes: self.ppu_oam_ram[i * 4 + 2],
                                x: self.ppu_oam_ram[i * 4 + 3],
                            });

                            if self.sprites_this_scanline.len() >= 8 {
                                break 'check_sprites;
                            }
                        }
                    }
                }

                // Drawing
                (1 ..= 256, 0 ..= 239) => {
                    let mut bg_pixel = 0u8;
                    let mut sprite_pixel = 0u8;
                    let mut sprite_in_front = true;

                    // Background drawing
                    if self.ppu_mask & 0x08 != 0 {
                        let mut nametable_base = (self.ppu_ctrl & 3) as u16 * 0x400 + 0x2000;
                        assert!(nametable_base == 0x2000 || nametable_base == 0x2400 || nametable_base == 0x2800 || nametable_base == 0x2C00);

                        // Position in nametable in pixels
                        let nt_x = (self.ppu_scroll_x as i32) + self.ppu_x - 1;
                        let nt_y = (self.ppu_scroll_y as i32) + self.ppu_y;

                        // Nametable tile coordinates
                        let mut tile_x = nt_x / 8;
                        if tile_x >= 32 {
                            tile_x -= 32;
                            nametable_base ^= 0x400;
                        }
                        let mut tile_y = nt_y / 8;
                        if tile_y >= 32 {
                            tile_y -= 32;
                            nametable_base ^= 0x800;
                        }

                        // Get nametable data
                        let nt_entry = self.read_ppu_byte(nametable_base + (tile_y * 32 + tile_x) as u16);
                        let attr_index = (tile_y / 4) * 8 + (tile_x / 4);
                        let attribute_byte = self.read_ppu_byte(nametable_base + 0x3C0 + attr_index as u16);
                        let attribute_idx = (tile_y % 4) / 2 * 2 + (tile_x % 4) / 2;
                        assert!(attribute_idx >= 0 && attribute_idx < 4);
                        let attribute = (attribute_byte >> (attribute_idx * 2)) & 3;

                        // Get pattern table data
                        let pattern_table_base = ((self.ppu_ctrl & 0x10) as u16) << 8;
                        assert!(pattern_table_base == 0x0000 || pattern_table_base == 0x1000);
                        let palette_index = self.get_pixel_from_pattern_table(pattern_table_base, nt_entry, (nt_x % 8) as u8, (nt_y % 8) as u8);

                        if palette_index == 0 {
                            bg_pixel = 0;
                        } else {
                            bg_pixel = palette_index + attribute * 4;
                        }

                        // Get palette colour
                        /*let colour_index = if palette_index > 0 {
                            self.read_ppu_byte(0x3F00 + (attribute as u16) * 4 + (palette_index as u16))
                        } else {
                            self.read_ppu_byte(0x3F00)
                        };
                        pixel = c_palette[colour_index as usize];*/
                    }

                    // Sprite drawing
                    if self.ppu_mask & 0x10 != 0 {
                        'sprite_loop: for sprite in &self.sprites_this_scanline {
                            let sprite_x = sprite.x as i32;
                            if self.ppu_x-1 >= sprite_x && self.ppu_x-1 < sprite_x + 8 {
                                let pattern_table_base = ((self.ppu_ctrl & 0x08) as u16) << 8;
                                assert!(pattern_table_base == 0x0000 || pattern_table_base == 0x1000);

                                let mut px = self.ppu_x - 1 - sprite_x;
                                if sprite.attributes & 0x40 != 0 {
                                    px = 7 - px;
                                }

                                let mut py = self.ppu_y - 1 - sprite.y as i32;
                                if sprite.attributes & 0x80 != 0 {
                                    py = 7 - py;
                                }

                                let palette_index = self.get_pixel_from_pattern_table(pattern_table_base, sprite.tile, px as u8, py as u8);

                                if palette_index == 0 {
                                    sprite_pixel = 0;
                                } else {
                                    let palette = 4 + (sprite.attributes & 3);
                                    sprite_pixel = palette_index + palette * 4;
                                }

                                if sprite_pixel != 0 {
                                    if sprite.index == 0 && bg_pixel != 0 {
                                        self.ppu_status |= 0x40; // zero hit
                                    }

                                    sprite_in_front = sprite.attributes & 0x20 == 0;

                                    break 'sprite_loop; // stop after the first non-transparent sprite is found
                                }
                            }
                        }
                    }

                    // Pixel priority
                    let palette_index = match (bg_pixel, sprite_pixel, sprite_in_front) {
                        (_, 0, true) => bg_pixel,
                        (_, _, true) => sprite_pixel,
                        (0, _, false) => sprite_pixel,
                        (_, _, false) => bg_pixel
                    };

                    // Get palette colour
                    let colour_index = self.ppu_palette_ram[palette_index as usize];
                    let pixel = PALETTE[colour_index as usize];

                    // Set the pixel in the frame buffer
                    let index = ((self.ppu_y * 256 + self.ppu_x - 1) * 4) as usize;
                    self.frame_buffer[index .. index+4].copy_from_slice(&pixel);
                }

                // VBlank flag
                (1, 241) => {
                    self.ppu_status |= 0x80;
                    if self.ppu_ctrl & 0x80 != 0 {
                        self.ppu_nmi_flag = true;
                    }
                }

                // All others
                _ => {}
            }

            // TODO more logic
            
            // Advance
            self.ppu_x += 1;
            if self.ppu_x == 341 {
                self.ppu_x = 0;
                self.ppu_y += 1;
    
                if self.ppu_y == 261 {
                    self.ppu_y = -1;
                }
            }
        }
    }
}
