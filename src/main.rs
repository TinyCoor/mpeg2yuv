
use std::io;
use std::fs::File;
use std::io::Write;

fn write_ppm_image(sink: &mut impl Write,pixels:&[u32],stride:usize)
->io::Result<()>{
    let w = stride;
    let h = pixels.len() /stride;
    writeln!(sink,"P6\n{} {} 255\n",w,h);
    for pixel in pixels {
        let r = ((pixel >> 8 * 2) & 0xFF) as u8;
        let g = ((pixel >> 8 * 1) & 0xFF) as u8;
        let b = ((pixel >> 8 * 0) & 0xFF) as u8;
        sink.write(&[r,g,b])?;
    };
    Ok(())
}

fn main() {
    const WIDTH :usize = 800;
    const HEIGHT :usize = 600;

    let mut pixels : [u32;WIDTH*HEIGHT] = [0xFF;WIDTH*HEIGHT];
    let mut file = File::create("output.ppm").unwrap();
    write_ppm_image(&mut file,&pixels,WIDTH).unwrap();


}
