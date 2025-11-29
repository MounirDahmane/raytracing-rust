use std::fs::File;
use std::io::{self, Write};

const IMG_HEIGHT: u32 = 256;
const IMG_WIDTH: u32 = 256;

fn main() -> io::Result<()> {
    let mut file = File::create("./img.ppm")?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMG_WIDTH, IMG_HEIGHT)?;
    writeln!(file, "255")?;

    for i in 0..IMG_HEIGHT {
        eprint!("\rScanlines remaining: {}", IMG_HEIGHT - i - 1);
        io::stderr().flush()?;

        for j in 0..IMG_WIDTH {
            let r: f32 = j as f32 / (IMG_WIDTH - 1) as f32;
            let g: f32 = i as f32 / (IMG_HEIGHT - 1) as f32;
            let b: f32 = 0.0;

            let ir: u8 = (255.999 * r) as u8;
            let ig: u8 = (255.999 * g) as u8;
            let ib: u8 = (255.999 * b) as u8;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }
    eprint!("                           \n");
    eprintln!("\rdone");
    Ok(())
}
