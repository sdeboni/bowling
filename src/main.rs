use bowling::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut game = BowlingGame::new();

    loop {
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim().is_empty() {
            break;
        }
        let pins = buffer.trim().parse::<u8>().unwrap();
        if let Err(err) = game.roll(pins) {
            println!("{err:?}");
        }
        buffer.clear();
    }
    Ok(())
}
