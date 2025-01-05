// File contains functions which actually solve for the variable/perform computation

use std::f64::INFINITY;



// use postfix - infix algorithm to evaulate value substituted into function as a string
// f_proc = LHS - RHS for Newton Raphson
// 1 scan
pub fn evaluate(f_proc: &mut str, x: f64) -> f64 {
    use std::collections::HashMap;

    let mut operator_stack: Vec<char> = Vec::new(); // Stack for operators
    let mut operand_stack: Vec<f64> = Vec::new();   // Stack for operands

    // Define operator precedence
    let precedence: HashMap<char, i32> = HashMap::from([
        ('+', 1),
        ('-', 1),
        ('*', 2),
        ('/', 2),
        ('^', 3),
    ]);

    // Helper function to apply an operator to two operands
    fn apply_operator(op: char, b: f64, a: f64) -> f64 {
        match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            '^' => a.powf(b),
            _ => panic!("Invalid operator"),
        }
    }

    // Scan through the string
    let mut num_buffer = String::new(); // To handle multi-digit numbers

    for ch in f_proc.chars() {
        if ch.is_whitespace() {
            continue; // Ignore whitespace
        }

        if ch.is_digit(10) || ch == '.' {
            // Collect digits or decimal points into num_buffer
            num_buffer.push(ch);
        } else {
            // Push any number in num_buffer onto the operand stack
            if !num_buffer.is_empty() {
                operand_stack.push(num_buffer.parse().unwrap());
                num_buffer.clear();
            }

            if ch.is_alphabetic() {
                // Substitute the variable with its value
                operand_stack.push(x);
            } else if precedence.contains_key(&ch) {
                // Handle operators
                while !operator_stack.is_empty() && *operator_stack.last().unwrap() != '('
                    && precedence[&ch] <= precedence[operator_stack.last().unwrap()]
                {
                    let op = operator_stack.pop().unwrap();
                    let b = operand_stack.pop().unwrap();
                    let a = operand_stack.pop().unwrap();
                    operand_stack.push(apply_operator(op, b, a));
                }
                operator_stack.push(ch);
            } else if ch == '(' {
                operator_stack.push(ch);
            } else if ch == ')' {
                // Process until the matching '('
                while *operator_stack.last().unwrap() != '(' {
                    let op = operator_stack.pop().unwrap();
                    let b = operand_stack.pop().unwrap();
                    let a = operand_stack.pop().unwrap();
                    operand_stack.push(apply_operator(op, b, a));
                }
                operator_stack.pop(); // Remove '('
            }
        }
    }

    // Push the final number in num_buffer onto the operand stack
    if !num_buffer.is_empty() {
        operand_stack.push(num_buffer.parse().unwrap());
    }

    // Process remaining operators in the operator stack
    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        let b = operand_stack.pop().unwrap();
        let a = operand_stack.pop().unwrap();
        operand_stack.push(apply_operator(op, b, a));
    }

    // The final result is the last remaining value in the operand stack
    operand_stack.pop().unwrap()
}

// fprime_discrete(x0) = [f(x0+epsilon) - f(x0)] / f(epsilon)
pub fn finite_pos_difference(f_proc: &mut str, x0: f64, epsilon: f64) -> f64 {
    (evaluate(f_proc, x0+epsilon) - evaluate(f_proc, x0))/epsilon
}

// LHS - RHS = 0 using newton raphson algorithm
pub fn newton_raphson(f_proc: &mut str, mut guess: f64, tolerance: f64, epsilon: f64, iterations: u16) -> f64{
    
    let mut f_eval: f64 = 0.0;
    let mut f_prime_eval: f64 = 0.0;
    let mut x1: f64 = 0.0;

    for _ in 1..=iterations {
        f_eval = evaluate(f_proc, guess);
        f_prime_eval = finite_pos_difference(f_proc, guess, epsilon);
        if f_prime_eval.abs() < epsilon {
            break;
        }
        x1 = guess - f_eval / f_prime_eval;
        if (x1 - guess).abs() <= tolerance {
            return x1;
        }
        guess = x1;

    }

    // no convergence
    return INFINITY;
}
