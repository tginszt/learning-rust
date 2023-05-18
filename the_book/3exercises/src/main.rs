fn main() {
    println!("Here's fibonacci!");
    fibonacci(5);

    println!("Here's farenheit to celcius!");
    let celcius = farenheit_to_celcius(108f32);
    println!("{celcius}");
}

fn farenheit_to_celcius(degrees_fh: f32) -> f32 {
    ((degrees_fh - 32f32) * 5f32 / 9f32)
}

// Some people say that you can judge a programmer by his fibonacci, I'd disagree...
fn fibonacci(n: i32){
    let mut number1 = 1;
    let mut number2 = 1;
    let mut counter = 0;

    while counter < n {
        println!("{number1}");
        counter += 1;
        if counter == n {break}

        number1 += number2;

        println!("{number2}");
        counter += 1;

        number2 += number1;
    }
}