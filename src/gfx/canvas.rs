use std::fs::File;

use png::Encoder;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::rgb(255, 255, 255); width * height],
        }
    }

    #[allow(unused)]
    pub fn get(&self, x: usize, y: usize) -> Option<&Color> {
        self.pixels.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        self.pixels.get_mut(y * self.width + x)
    }

    pub fn save_png(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let ref mut w = std::io::BufWriter::new(file);
        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);

        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data: Vec<_> = self
            .pixels
            .iter()
            .flat_map(|pixel| vec![pixel.r(), pixel.g(), pixel.b()])
            .collect();

        writer.write_image_data(data.as_slice())?;

        Ok(())
    }
}
