pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub fn get_terminal_size() -> Result<Size, std::io::Error> {
    let size = termion::terminal_size()?;

    Ok(Size {
        width: size.0,
        height: size.1,
    })
}
