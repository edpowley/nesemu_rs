use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image, DrawParam, FilterMode};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::timer;

use ggez::mint::Point2;

mod emulator;
mod opcodes;

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

fn run_test_rom() {
    let rom_path = Path::new("../nestest.nes");
    let mut emu_state = emulator::EmuState::new(rom_path);

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

        emu_state.run_one_instruction();
    }
}

fn main() {
    let scale_factor = 4;
    let window_mode = WindowMode {
        width: (256 * scale_factor) as f32,
        height: (240 * scale_factor) as f32,
        ..Default::default()
    };
    let window_setup = WindowSetup {
        title: "Emulator".to_owned(),
        ..Default::default()
    };

    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    emu_state: emulator::EmuState,
    frame_image: Image,
    frame_count: u64
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let rom_path = Path::new("C:\\Users\\edpow\\Dropbox\\ROMs\\NES\\S\\Super Mario Bros..nes");

        return MyGame {
            emu_state: emulator::EmuState::new(rom_path),
            frame_image: Image::solid(ctx, 256, Color::BLACK).expect("Failed to create image"),
            frame_count: 0
        };
    }

}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.emu_state.run_to_next_nmi();
        //println!("{} {}", self.emu_state.ppu_x, self.emu_state.ppu_y);

        self.frame_image = Image::from_rgba8(_ctx, 256, 240, &self.emu_state.frame_buffer)?;
        self.frame_image.set_filter(FilterMode::Nearest);

        self.frame_count += 1;
        if self.frame_count % 60 == 0 {
            println!("{} fps", timer::fps(_ctx));
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        
        graphics::draw(ctx, &self.frame_image, DrawParam::new().scale([4f32, 4f32]))?;
        
        graphics::present(ctx)
    }
}

/*
fn main() {
    run_game();
}
*/