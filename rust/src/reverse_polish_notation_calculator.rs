pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn add(a: i32, b: i32) -> i32 {
    b + a
}
fn subtract(a: i32, b: i32) -> i32 {
    b - a
}
fn multiply(a: i32, b: i32) -> i32 {
    b * a
}
fn divide(a: i32, b: i32) -> i32 {
    b / a
}

fn evaluate_two_last_items(stack: &mut Vec<i32>, calc: fn(i32, i32) -> i32) -> Option<()> {
    let a = stack.pop();
    let b = stack.pop();

    match (a, b) {
        (Some(a), Some(b)) => Some(stack.push(calc(a, b))),
        _ => None,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add => evaluate_two_last_items(&mut stack, add)?,
            CalculatorInput::Subtract => evaluate_two_last_items(&mut stack, subtract)?,
            CalculatorInput::Multiply => evaluate_two_last_items(&mut stack, multiply)?,
            CalculatorInput::Divide => evaluate_two_last_items(&mut stack, divide)?,
            CalculatorInput::Value(num) => stack.push(*num),
        }
    }

    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}
