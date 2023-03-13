#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut stack = vec![];
    for i in inputs.iter() {
        match i {
            Value(i) => stack.push(*i),
            _ => {
                let (b, a) = (stack.pop()?, stack.pop()?);
                stack.push(match i {
                    Add => Some(a + b),
                    Subtract => Some(a - b),
                    Multiply => Some(a * b),
                    Divide if b != 0 => Some(a / b),
                    _ => None,
                }?)
            }
        }
    }
    match &stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}
