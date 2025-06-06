use minifb::{Key, Window, WindowOptions};

fn koch_curve(points: &mut Vec<(f64, f64)>, x1: f64, y1: f64, x2: f64, y2: f64, n: i32) {
    if n == 0 {
        points.push((x1, y1));
        return;
    }
    
    let dx = x2 - x1;
    let dy = y2 - y1;
    
    let xa = x1 + dx / 3.;
    let ya = y1 + dy / 3.;
    let xb = x1 + 2. * dx / 3.;
    let yb = y1 + 2. * dy / 3.;
    
    let sqrt_3 = 3.0_f64.sqrt();
    let xc = (x1 + x2) / 2. + dy * sqrt_3 / 6.;
    let yc = (y1 + y2) / 2. - dx * sqrt_3 / 6.;
    
    koch_curve(points, x1, y1, xa, ya, n - 1);
    koch_curve(points, xa, ya, xc, yc, n - 1);
    koch_curve(points, xc, yc, xb, yb, n - 1);
    koch_curve(points, xb, yb, x2, y2, n - 1);
}

fn main() {
    println!("Введите глубину рекурсии n: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Ошибка ввода");
    let n: i32 = input.trim().parse().expect("Ожидается целое число");
    
    let width = 1200;
    let height = 800;
    
    let x1 = 200.;
    let y1 = 600.;
    let x2 = 1000.;
    let y2 = 600.;
    
    let mut points = Vec::new();
    koch_curve(&mut points, x1, y1, x2, y2, n);
    points.push((x2, y2));
    
    let mut window = Window::new(
        "Кривая Коха",
        width,
        height,
        WindowOptions::default(),
    ).expect("Не удалось создать окно");
    
    let mut buffer: Vec<u32> = vec![0; width * height];
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Очистка экрана
        buffer.fill(0);
        
        // Рисование кривой
        for i in 0..points.len() - 1 {
            let (x1, y1) = points[i];
            let (x2, y2) = points[i + 1];
            draw_line(&mut buffer, width, x1 as i32, y1 as i32, x2 as i32, y2 as i32);
        }
        
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn draw_line(buffer: &mut [u32], width: usize, x1: i32, y1: i32, x2: i32, y2: i32) {
    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    let (mut x, mut y) = (x1, y1);
    
    loop {
        if x >= 0 && x < width as i32 && y >= 0 && y < width as i32 {
            buffer[(y as usize * width + x as usize) as usize] = 0x00FF00; // Зеленый цвет
        }
        
        if x == x2 && y == y2 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
