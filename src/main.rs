use std::path::Path;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image, DrawParam, FilterMode};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::input::keyboard;
use ggez::timer;

mod emulator;
mod opcodes;

#[cfg(test)]
mod tests;

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
            emu_state: emulator::EmuState::new(rom_path).unwrap(),
            frame_image: Image::solid(ctx, 256, Color::BLACK).expect("Failed to create image"),
            frame_count: 0
        };
    }

}

const KEY_MAP: [(KeyCode, usize); 8] = [
    (KeyCode::A, 0),
    (KeyCode::S, 1),
    (KeyCode::Back, 2),
    (KeyCode::Return, 3),
    (KeyCode::Up, 4),
    (KeyCode::Down, 5),
    (KeyCode::Left, 6),
    (KeyCode::Right, 7)
];

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for (keycode, index) in KEY_MAP {
            self.emu_state.joypad1.buttons[index] = keyboard::is_key_pressed(_ctx, keycode);
        }

        self.emu_state.run_to_next_nmi()?;

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
