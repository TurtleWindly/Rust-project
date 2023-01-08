
pub struct Chip8 {
    pub opcode: i16,
    pub memory: [u8; 4096],
    // CPU register
    pub v: [u8; 16],
    // Index register I and program counter pc
    pub i: u16,
    pub pc: u16,
    // Screen size is 64 x 32. Total: 2048
    pub display: [u8; 64 * 32],
    // two timer registers that count at 60 Hz. When set above zero they will count down to zero
    pub delay_timer: u8,
    pub sound_timer: u8,
    // stack
    pub stack: [u8; 16],
    // stack pointer
    pub sp: u8,
    // Key
    pub keypad: [u8; 16],
    // Draw flag
    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { opcode: 0, memory: [0; 4096], v: [0; 16], i: 0, pc: 0, display: [0; 64 * 32], delay_timer: 0, sound_timer: 0, stack: [0; 16], sp: 0, keypad: [0; 16], draw_flag: false}
    }
    pub fn load_game(&self, name: String) {
        
    }
    pub fn emulate_cycle(&self) {
        
    }
    pub fn set_keys(&self) {
        
    }
}
