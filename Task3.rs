use std::io;

fn calculate(nums: &[i32], pos: usize, current: i32, target: i32) -> bool {
    if pos == nums.len() {
        return current == target;
    }
    
    // сложение
    if calculate(nums, pos + 1, current + nums[pos], target) {
        return true;
    }
    
    // умножение
    if calculate(nums, pos + 1, current * nums[pos], target) {
        return true;
    }
    
    false
}

fn main() {
    println!("Введите N: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    let n: usize = input.trim().parse().expect("Ожидалось целое число");
    
    if n == 0 {
        println!("Ошибка");
        return;
    }
    
    println!("Введите {} двузначных чисел(-ла): ", n);
    let mut numbers = Vec::with_capacity(n);
    for _ in 0..n {
        let mut num_input = String::new();
        io::stdin().read_line(&mut num_input).expect("Ошибка чтения ввода");
        let num: i32 = num_input.trim().parse().expect("Ожидалось двузначное число");
        numbers.push(num);
    }
    
    println!("Введите число S: ");
    let mut s_input = String::new();
    io::stdin().read_line(&mut s_input).expect("Ошибка чтения ввода");
    let s: i32 = s_input.trim().parse().expect("Ожидалось целое число");
    
    if calculate(&numbers, 1, numbers[0], s) {
        println!("Да, можно получить число {} из данных чисел", s);
    } else {
        println!("Нет, нельзя получить число {} из данных чисел", s);
    }
}
