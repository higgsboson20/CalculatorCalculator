
use tcrate::math;

use math::{evaluate, finite_pos_difference, newton_raphson};

#[test]
fn test_basic_operations() {
    let mut eq1 = String::from("3 + 3");
    let mut eq2 = String::from("10 - 2");
    let mut eq3 = String::from("4 * 5");
    let mut eq4 = String::from("8 / 2");

    assert_eq!(evaluate(&mut eq1, 0.0), 6.0);
    assert_eq!(evaluate(&mut eq2, 0.0), 8.0);
    assert_eq!(evaluate(&mut eq3, 0.0), 20.0);
    assert_eq!(evaluate(&mut eq4, 0.0), 4.0);
}

#[test]
fn test_parentheses() {
    let mut eq1 = String::from("(3 + 3) * 2");
    let mut eq2 = String::from("10 - (2 + 3)");
    let mut eq3 = String::from("4 * (5 + 2)");
    let mut eq4 = String::from("(8 / 2) * (3 + 1)");

    assert_eq!(evaluate(&mut eq1, 0.0), 12.0);
    assert_eq!(evaluate(&mut eq2, 0.0), 5.0);
    assert_eq!(evaluate(&mut eq3, 0.0), 28.0);
    assert_eq!(evaluate(&mut eq4, 0.0), 16.0);
}

#[test]
fn test_variables() {
    let mut eq1 = String::from("x + 3");
    let mut eq2 = String::from("2 * x");
    let mut eq3 = String::from("x ^ 2");
    let mut eq4 = String::from("(x + 2) * (x - 3)");

    assert_eq!(evaluate(&mut eq1, 4.0), 7.0);
    assert_eq!(evaluate(&mut eq2, 3.0), 6.0);
    assert_eq!(evaluate(&mut eq3, 5.0), 25.0);
    assert_eq!(evaluate(&mut eq4, 3.0), 0.0);
}

#[test]
fn test_complex_expressions() {
    let mut eq1 = String::from("(3 + 5) * (2 - 8) / 2");
    let mut eq2 = String::from("5 * 3 * 2 * 8 - x * (x + x) - 5 * (2 - 8 * (5 + 3))");
    let mut eq3 = String::from("((x + 2) * 5) - (3 * (x - 1)) + 4 / 2");

    assert_eq!(evaluate(&mut eq1, 0.0), -24.0);
    assert_eq!(evaluate(&mut eq2, 4.0), 518.0);
    assert_eq!(evaluate(&mut eq3, 3.0), 21.0);
}



#[test]
fn test_eval_print() {
    let mut eqA: String = String::from("(3+3)-x");
    let mut eqB: String = String::from("5*3*2*8 - e*(e+e) - 5*(2-8*(5+3))");

    println!("{}", evaluate(&mut eqA, 4.0));
    println!("{}", evaluate(&mut eqB, 4.0));

}

#[test]
fn test_finite_diff(){
    let mut eqA: String = String::from("x^x - 2");
    let epsilon: f64 = 1e-6;
    let x0: f64 = 2.0;

    println!("{}", finite_pos_difference(&mut eqA, x0, epsilon));
}

#[test]
fn test_newton_method(){
    let mut eqA: String = String::from("x^x - 2");
    let mut eqB: String = String::from("x^5+x^3-x");
    let epsilon: f64 = 1e-6;
    let guess: f64 = -0.5; // guess
    let tolerance: f64 = 10e-7;
    let iterations: u16 = 10;

    println!("{}", newton_raphson(&mut eqA, guess, tolerance, epsilon, iterations));
    println!("{}", newton_raphson(&mut eqB, guess, tolerance, epsilon, iterations));
}