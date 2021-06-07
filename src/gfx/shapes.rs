use super::canvas::{Canvas, Color};

pub fn rect(canvas: &mut Canvas, x: usize, y: usize, w: usize, h: usize, color: Color) {
    for i in x..(x + w) {
        for j in y..(y + h) {
            canvas.get_mut(i, j).map(|p| *p = color);
        }
    }
}
