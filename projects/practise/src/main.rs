// use std::io;

// fn sum_ab(a: u32, b: u32) -> u32 {
//     a + b
// }

// fn main() {
//     println!("Program to add two numbers");

//     loop {
//         let mut a = String::new();
//         let mut b = String::new();

//         println!("Enter first number");
//         io::stdin().read_line(&mut a).expect("Failed to read line");

//         println!("Enter second number");
//         io::stdin().read_line(&mut b).expect("Failed to read line");

//         let a: u32 = a.trim().parse().expect("Invalid number");
//         let b: u32 = b.trim().parse().expect("Invalid number");

//         let sum_a_b = sum_ab(a, b);

//         println!("The sum of two numbers is {}", sum_a_b);

//         if sum_a_b > 100000 {
//             println!("The sum is too big!");
//         } else {
//             println!("The sum is too small!");
//             break;
//         }
//     }
// }

// fn main() {
//     let mut count = 0;
//     loop {
//         count = count + 1;
//         println!("count {count}");
//         if count > 10 {
//             break;
//         }
//     }
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut count = 10;

//     while count > 0 {
//         println!("count {count}");
//         count = count - 1;
//     }
// }

// fn main() {
//     for c in (1..11).rev() {
//         println!("{c}");
//     }
// }

// convert temperature from Celsius to Fahrenheit and vice versa

// use std::io;

// fn convert_to_fahrenheit(dc_temperature: f32) -> f32 {
//     dc_temperature * (9.0 / 5.0) + 32.0
// }
// fn convert_to_degree_celsius(f_temperature: f32) -> f64 {}

// fn main() {
//     println!("Convert temperature to Fahrenheit!!");
//     let mut temperature = String::new();
//     println!("Enter temperature in Degree celsius: ");
//     io::stdin()
//         .read_line(&mut temperature)
//         .expect("Could not read the temperature");

//     let temperature: f32 = temperature
//         .trim()
//         .parse()
//         .expect("Incorrect temperature value inserted");

//     let f_temperature: f32 = convert_to_fahrenheit(temperature);

//     println!(
//         "The Fahrenheit temperature for {} is {}",
//         temperature, f_temperature
//     );
// }

use std::collections::HashMap;

fn fibonacci(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(result) = cache.get(&n) {
        return *result;
    } else {
        let result = if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            fibonacci(n - 1, cache) + fibonacci(n - 2, cache)
        };
        cache.insert(n, result);
        return result;
    }
}

fn main() {
    let n = 100;
    let mut cache = HashMap::new();
    let fib = fibonacci(n, &mut cache);
    println!("The {}th Fibonacci number is {}", n, fib);
}
