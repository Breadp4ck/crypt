use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // Получаем имя файла из аргументов командной строки
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Использование: lab2 <имя_файла>");
        return;
    }
    let filename = &args[1];

    // Читаем содержимое файла
    let mut file = File::open(filename).expect("Не удалось открыть файл");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Не удалось прочитать файл");

    // Вычисляем частоты символов
    let frequencies = calculate_frequencies(&contents);

    // Вычисляем энтропию
    let entropy = calculate_entropy(&frequencies);

    // Выводим результаты
    println!("Частоты символов:");
    for (symbol, frequency) in &frequencies {
        println!("'{}': {}", symbol, frequency);
    }
    println!("Энтропия: {:.2}", entropy);
}

// Функция для вычисления частот символов в строке
fn calculate_frequencies(text: &str) -> HashMap<char, f64> {
    let mut frequencies = HashMap::new();
    let length = text.len() as f64;

    // Подсчитываем количество появлений каждого символа
    for symbol in text.chars() {
        *frequencies.entry(symbol).or_insert(0.0) += 1.0;
    }

    // Нормализуем частоты
    for frequency in frequencies.values_mut() {
        *frequency /= length;
    }

    frequencies
}

// Функция для вычисления энтропии по частотам символов
fn calculate_entropy(frequencies: &HashMap<char, f64>) -> f64 {
    let mut entropy = 0.0;

    // Вычисляем энтропию по формуле Шеннона
    for frequency in frequencies.values() {
        if *frequency > 0.0 {
            entropy -= frequency * frequency.log2();
        }
    }

    entropy
}
