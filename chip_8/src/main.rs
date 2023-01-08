mod chip8;

use chip8::Chip8;

fn main() {
    setup_graphic();
    setup_input();

    // Init and load game
    let chip8 = Chip8::new();
    chip8.load_game("pong".to_string());

    // Emulation loop
    loop {
        // emulate 1 cycle
        chip8.emulate_cycle();

        // draw flag set then draw
        if chip8.draw_flag {
            draw_graphic();
        }

        // store key press state Press/Release
        chip8.set_keys();
    }
}

fn setup_graphic() {}

fn setup_input() {}

fn draw_graphic() {}
