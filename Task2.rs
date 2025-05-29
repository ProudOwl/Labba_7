use std::io;

fn decode_string(s: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = s.chars().collect();
    
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let mut k = 0;
            while i < chars.len() && chars[i].is_ascii_digit() {
                k = k * 10 + chars[i].to_digit(10).unwrap() as usize;
                i += 1;
            }

            if k < 1 || k > 300 {
                eprintln!("Ошибка: число повторений должно быть от 1 до 300.");
                return String::new();
            }
            
            i += 1; // пропускаем '['
            
            let start = i;
            let mut bracket_count = 1;
            while i < chars.len() && bracket_count > 0 {
                if chars[i] == '[' {
                    bracket_count += 1;
                } else if chars[i] == ']' {
                    bracket_count -= 1;
                }
                i += 1;
            }
            let end = i - 1;
            
            let substr: String = chars[start..end].iter().collect();
            let decoded_substr = decode_string(&substr);
            
            for _ in 0..k {
                result.push_str(&decoded_substr);
            }
        } else if chars[i] != ']' {
            result.push(chars[i]);
            i += 1;
        } else {
            i += 1;
        }
    }
    result
}

fn main() {
    println!("Введите закодированную строку: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения строки");
    let input = input.trim();

    // проверка длины строки
    if input.len() < 1 || input.len() > 30 {
        eprintln!("Ошибка: длина строки должна быть от 1 до 30 символов.");
        std::process::exit(1);
    }

    // проверка символов
    for c in input.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '[' || c == ']') {
            eprintln!("Ошибка: строка содержит недопустимые символы.");
            std::process::exit(1);
        }
    }

    let decoded = decode_string(input);
    println!("Декодированная строка: {}", decoded);
}
