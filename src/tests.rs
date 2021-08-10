use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

use crate::emulator;

#[derive(Debug)]
struct LogLine {
    pub prg_cnt: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub sp: u8,
    pub ppu_x: i32,
    pub ppu_y: i32,
    pub cycle: u64,
}

fn parse_nestest_log(path: &Path) -> Vec<LogLine> {
    let mut result = Vec::new();
    let file = File::open(path).expect("Failed to open file");
    let lines = io::BufReader::new(file).lines();

    for line_result in lines {
        let line = line_result.expect("Failed to read line");

        result.push(LogLine {
            prg_cnt:    u16::from_str_radix(&line[ 0.. 4],        16).expect("Failed to parse"),
            a:          u8 ::from_str_radix(&line[50..52],        16).expect("Failed to parse"),
            x:          u8 ::from_str_radix(&line[55..57],        16).expect("Failed to parse"),
            y:          u8 ::from_str_radix(&line[60..62],        16).expect("Failed to parse"),
            p:          u8 ::from_str_radix(&line[65..67],        16).expect("Failed to parse"),
            sp:         u8 ::from_str_radix(&line[71..73],        16).expect("Failed to parse"),
            ppu_x:      i32::from_str_radix(&line[78..81].trim(), 10).expect("Failed to parse"),
            ppu_y:      i32::from_str_radix(&line[82..85].trim(), 10).expect("Failed to parse"),
            cycle:      u64::from_str_radix(&line[90..  ],        10).expect("Failed to parse"),
        });
    }

    return result;
}

#[test]
fn run_test_rom() {    
    let rom_path = Path::new("../nestest.nes");
    let mut emu_state = emulator::EmuState::new(rom_path).unwrap();

    let log_path = Path::new("../nestest.log");
    let log = parse_nestest_log(log_path);

    emu_state.program_counter = 0xC000;
    let mut line_number: u32 = 0;

    for log_line in log {
        line_number += 1;
        println!("{}: {:?}", line_number, log_line);
        assert_eq!(emu_state.program_counter, log_line.prg_cnt, "Program Counter: actual {:04X}, expected {:04X}", emu_state.program_counter, log_line.prg_cnt);
        assert_eq!(emu_state.reg_a, log_line.a, "A");
        assert_eq!(emu_state.reg_x, log_line.x, "X");
        assert_eq!(emu_state.reg_y, log_line.y, "Y");
        assert_eq!(emu_state.get_flags_as_u8(), log_line.p, "Flags: actual {:08b}, expected {:08b}", emu_state.get_flags_as_u8(), log_line.p);
        assert_eq!(emu_state.stack_pointer, log_line.sp, "Stack");

        emu_state.run_one_instruction().unwrap();
    }
}

