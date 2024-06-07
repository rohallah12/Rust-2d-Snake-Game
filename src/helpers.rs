use crossterm::{
    cursor,
    style::{self, Stylize},
    ExecutableCommand,
};
use std::io::{self, Stdout};

pub fn create_frame(std: &mut Stdout, width: u16, height: u16) -> io::Result<()> {
    for y in 0..height {
        for x in 0..width {
            if (y == 0 || y == height - 1) || (x == 0 || x == width - 1) {
                std.execute(cursor::MoveTo(x, y))?
                    .execute(style::PrintStyledContent("█".white()))?;
            }
        }
    }
    print!("\n");
    Ok(())
}

//Create a block at position (x, y)
pub fn create_block(std: &mut Stdout, x: u16, y: u16, r: char) -> io::Result<()> {
    std.execute(cursor::MoveTo(x, y))?
        .execute(style::PrintStyledContent(r.magenta()))?;
    Ok(())
}

//Clear the terminal
pub fn clear() {
    std::process::Command::new("clear").status().unwrap();
}
