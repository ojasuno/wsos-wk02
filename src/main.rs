use std::io;

fn main() {
    println!("Welcome to Ackee Blockchain - Week 02");
    println!("This is a simple calculator program");
    println!("Please make your choice...");
    println!("A : for Addition");
    println!("S : for Substraction");
    println!("M : for Multiplication");
    println!("D : for Division");
    println!("E : for Exit");

    // empty_string_zero
    let mut empty_string_zero = String::new();
    io::stdin()
        .read_line(&mut empty_string_zero)
        .expect("Failed to read line");
    let digit_zero:String = empty_string_zero
        .trim()
        .parse()
        .expect("Input not an integer");
    println!("empty_string_zero: {}", empty_string_zero);

    // empty_string_one
    let mut empty_string_one = String::new();
    io::stdin()
        .read_line(&mut empty_string_one)
        .expect("Failed to read line");
    let digit_one:f32 = empty_string_one
        .trim()
        .parse()
        .expect("Input not an integer");
    println!("empty_string_one: {}", empty_string_one);

    // empty_string_two
    let mut empty_string_two = String::new();
    io::stdin()
        .read_line(&mut empty_string_two)
        .expect("Failed to read line");
    let digit_two:f32 = empty_string_two
        .trim()
        .parse()
        .expect("Input not an integer");
    println!("empty_string_two: {}", empty_string_two);

    // println!("STILL THERE -- empty_string_zero: {}", empty_string_zero);

    // if empty_string_zero == "A" {
    if digit_zero == "A" {
        println!("You picked -- {}", digit_zero);
        let op_result_add = calc_add(digit_one, digit_two);
        println!("add: Result of this operation is: {}", op_result_add);    
    }
    else if digit_zero == "S" {
        println!("You picked -- {}", digit_zero);
        let op_result_substract = calc_substract(digit_one, digit_two);
        println!("substract: Result of this operation is: {}", op_result_substract);    
    }
    else if digit_zero == "M" {
        println!("You picked -- {}", digit_zero);
        let op_result_multiply = calc_multiply(digit_one, digit_two);
        println!("multiply: Result of this operation is: {}", op_result_multiply);    
    }
    else if digit_zero == "D" {
        println!("You picked -- {}", digit_zero);
        let op_result_divide = calc_divide(digit_one, digit_two);
        println!("divide: Result of this operation is: {}", op_result_divide);    
    }
    else if digit_zero == "E" {
        println!("You picked -- {}", digit_zero);
    }
    else {
        println!("You picked an unavailable choice");
    }

}

fn calc_add(digit_one: f32, digit_two: f32) -> f32 {
    let calc_result = digit_one + digit_two;
    return calc_result;
}

fn calc_substract(digit_one: f32, digit_two: f32) -> f32 {
    let calc_result = digit_one - digit_two;
    return calc_result;
}

fn calc_multiply(digit_one: f32, digit_two: f32) -> f32 {
    let calc_result = digit_one * digit_two;
    return calc_result;
}

fn calc_divide(digit_one: f32, digit_two: f32) -> f32 {
    let calc_result = digit_one / digit_two;
    return calc_result;
}