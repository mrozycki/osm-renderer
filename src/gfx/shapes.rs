use super::canvas::{Canvas, Color};

#[allow(unused)]
pub fn rect(canvas: &mut Canvas, x: usize, y: usize, w: usize, h: usize, color: Color) {
    for i in x..(x + w) {
        for j in y..(y + h) {
            canvas.get_mut(i, j).map(|p| *p = color);
        }
    }
}

#[allow(unused)]
pub fn line(canvas: &mut Canvas, mut start: (i32, i32), mut end: (i32, i32), color: Color) {
    let gradient = (end.1 - start.1) as f64 / (end.0 - start.0) as f64;
    println!("Start: {:?} End: {:?} Gradient: {}", start, end, gradient);
    if gradient.abs() <= 1.0 {
        if start.0 > end.0 {
            std::mem::swap(&mut start, &mut end);
        }

        let mut y = start.1;
        let mut error = 0.0;
        for x in start.0..=end.0 {
            canvas.get_mut(x as usize, y as usize).map(|p| *p = color);
            error = error + gradient;
            if error <= -1.0 {
                error = error + 1.0;
                y = y - 1;
            } else if error >= 1.0 {
                error = error - 1.0;
                y = y + 1;
            }
        }
    } else {
        if start.1 > end.1 {
            std::mem::swap(&mut start, &mut end);
        }

        let mut x = start.0;
        let mut error = 0.0;
        for y in start.1..=end.1 {
            canvas.get_mut(x as usize, y as usize).map(|p| *p = color);
            error = error + 1.0 / gradient;
            if error <= -1.0 {
                error = error + 1.0;
                x = x - 1;
            } else if error >= 1.0 {
                error = error - 1.0;
                x = x + 1;
            }
        }
    }
}
