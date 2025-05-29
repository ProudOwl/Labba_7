use std::io;

// Рекурсивная функция для построения кривой Коха
fn koch_curve(points: &mut Vec<(f64, f64)>, x1: f64, y1: f64, x2: f64, y2: f64, n: i32) {
    if n == 0 {
        points.push((x1, y1));
        return;
    }
    
    let dx = x2 - x1;
    let dy = y2 - y1;
    
    // Точки для разделения отрезка на 3 части
    let xa = x1 + dx / 3.0;
    let ya = y1 + dy / 3.0;
    let xb = x1 + 2.0 * dx / 3.0;
    let yb = y1 + 2.0 * dy / 3.0;
    
    // Точка вершины равностороннего треугольника (используем константу sqrt(3) напрямую)
    let sqrt_3 = 3.0_f64.sqrt();
    let xc = (x1 + x2) / 2.0 - (y2 - y1) * sqrt_3 / 6.0;
    let yc = (y1 + y2) / 2.0 + (x2 - x1) * sqrt_3 / 6.0;
    
    // Рекурсивно строим 4 отрезка
    koch_curve(points, x1, y1, xa, ya, n - 1);
    koch_curve(points, xa, ya, xc, yc, n - 1);
    koch_curve(points, xc, yc, xb, yb, n - 1);
    koch_curve(points, xb, yb, x2, y2, n - 1);
}

// Функция для рисования линии между двумя точками
fn draw_line(canvas: &mut Vec<Vec<char>>, x1: i32, y1: i32, x2: i32, y2: i32) {
    let mut x1 = x1;
    let mut y1 = y1;
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;
    
    loop {
        if x1 >= 0 && x1 < canvas[0].len() as i32 && y1 >= 0 && y1 < canvas.len() as i32 {
            canvas[y1 as usize][x1 as usize] = '*';
        }
        
        if x1 == x2 && y1 == y2 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x1 += sx;
        }
        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }
}

fn main() {
    println!("Введите глубину рекурсии для кривой Коха: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    let n: i32 = input.trim().parse().expect("Ожидалось целое число");
    
    let x1 = 0.0;
    let y1 = 0.0;
    let x2 = 1.0;
    let y2 = 0.0;
    
    let mut points = Vec::new();
    koch_curve(&mut points, x1, y1, x2, y2, n);
    points.push((x2, y2));
    
    let (min_x, max_x, min_y, max_y) = points.iter().fold(
        (points[0].0, points[0].0, points[0].1, points[0].1),
        |(min_x, max_x, min_y, max_y), &(x, y)| {
            (
                min_x.min(x),
                max_x.max(x),
                min_y.min(y),
                max_y.max(y),
            )
        },
    );
    
    // Вычисляем оптимальный размер холста с небольшими отступами
    let margin = 0.2;
    let width = max_x - min_x;
    let height = max_y - min_y;
    
    // Масштабируем координаты для вывода в консоль
    const CONSOLE_WIDTH: usize = 200; // Ширина консоли в символах
    const CONSOLE_HEIGHT: usize = 60; // Высота консоли в строках
    
    let scale_x = (CONSOLE_WIDTH as f64 - 1.0) / (width * (1.0 + 2.0 * margin));
    let scale_y = (CONSOLE_HEIGHT as f64 - 1.0) / (height * (1.0 + 2.0 * margin));
    let scale = scale_x.min(scale_y);
    
    // Создаем холст
    let mut canvas = vec![vec![' '; CONSOLE_WIDTH]; CONSOLE_HEIGHT];
    
    // Преобразуем все точки в экранные координаты
    let screen_points: Vec<(i32, i32)> = points
        .iter()
        .map(|&(x, y)| {
            let x = ((x - min_x + width * margin) * scale) as i32;
            let y = ((y - min_y + height * margin) * scale) as i32;
            let y = (CONSOLE_HEIGHT as i32 - 1) - y;
            (x, y)
        })
        .collect();
    
    // Рисуем линии между точками
    for i in 0..screen_points.len() - 1 {
        let (x1, y1) = screen_points[i];
        let (x2, y2) = screen_points[i + 1];
        draw_line(&mut canvas, x1, y1, x2, y2);
    }
    
    // Выводим холст
    for line in canvas {
        let s: String = line.into_iter().collect();
        println!("{}", s);
    }
}
