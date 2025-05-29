use std::io;

fn decode_string(s: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = s.chars().collect();
    
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            // Парсим число k
            let mut k = 0;
            while i < chars.len() && chars[i].is_ascii_digit() {
                k = k * 10 + chars[i].to_digit(10).unwrap() as usize;
                i += 1;
            }
            
            // Пропускаем '['
            i += 1;
            
            // Находим подстроку внутри скобок
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
            let end = i - 1; // позиция перед ']'
            
            // Декодируем подстроку внутри скобок
            let decoded_substr = decode_string(&chars[start..end].iter().collect::<String>());
            
            // Добавляем k раз декодированную подстроку к результату
            for _ in 0..k {
                result.push_str(&decoded_substr);
            }
        } else if chars[i] != ']' {
            // Простые символы
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
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    let input = input.trim(); // Удаляем символ новой строки
    
    let decoded = decode_string(input);
    println!("Декодированная строка: {}", decoded);
}
