// processes the equation for solving

use std::collections::HashMap;


fn is_valid_eq(eq: &str) -> bool {
    let split_eq = eq.split("=");
    return split_eq.count() == 2;
}

pub fn get_vars(eq: &str) -> Result<HashMap<String, String>, ()> {
    let mut vars = HashMap::new();

    if !is_valid_eq(eq) {
        return Err(())
    }

    let n = eq.len();
    for (i, chr) in eq.chars().enumerate() {
        if chr.is_alphabetic() {
            
            // check for sin, cos, tan, cot, sec, csc, log
            if i + 3 < n {
                if eq[i..i+3].eq("sin") {
                    
                }
            } 
            
            // check for ln
            else if i + 2 < n {

            }

            if !vars.contains_key(&chr.to_string()) {
                vars.insert(chr.to_string(), String::new());
            }
        }
    }

    Ok(vars)

}

// sub variables into it 
// substitute implicit multiplications
pub fn substitute(eq: &mut str, vars: HashMap<String, String>) -> String {
    let mut processed_eq = String::new();
    let mut prev_char = ' '; // To track the previous character for implicit multiplication logic

    for ch in eq.chars() {
        let key = &ch.to_string();

        // Check if this character is a variable that exists in the map
        if vars.contains_key(key) {
            if let Some(value) = vars.get(key) {
                if !value.is_empty() {
                    // Add implicit multiplication if needed
                    if prev_char.is_numeric() || prev_char == ')' || prev_char.is_alphabetic() {
                        processed_eq.push('*');
                    }
                    processed_eq.push_str(value);
                }
            }
        } else {
            // Handle implicit multiplication for numbers, variables, or parentheses
            if (prev_char.is_numeric() || prev_char == ')' || prev_char.is_alphabetic()) && (ch == '(' || ch.is_alphabetic()) {
                processed_eq.push('*');
            }
            processed_eq.push(ch);
        }

        // Update the previous character
        prev_char = ch;
    }

    processed_eq
}


pub fn subtract(eq: &mut str) -> String {
    // Split the equation into LHS and RHS
    let mut split_eq = eq.split("=");
    let mut lhs_eq = split_eq.next().unwrap().trim().to_string(); // LHS of the equation
    let rhs_eq = split_eq.next().unwrap().trim(); // RHS of the equation

    // Vector to hold terms from the RHS
    let mut subtractants: Vec<String> = Vec::new();

    // Split the RHS into terms (e.g., by spaces or operators)
    // Assuming terms are separated by "+" or "-" operators
    let mut term = String::new();
    let mut lpar = 0;
    let mut rpar = 0;
    for ch in rhs_eq.chars() {
        if (ch == '+' || ch == '-') && lpar == rpar {
            lpar = 0;
            rpar = 0;
            if !term.is_empty() {
                subtractants.push(term.trim().to_string());
                term.clear();
            }
            //term.push(ch); // Include the operator in the next term
        } 
        else if ch == '(' {
            lpar += 1;
            term.push(ch);
        } 
        else if ch == ')' {
            rpar += 1;
            term.push(ch);
        }
        else {
            term.push(ch);
        }
    }
    if !term.is_empty() {
        subtractants.push(term.trim().to_string()); // Add the last term
    }

    // Move each term to the LHS with subtraction
    for sub in subtractants {
        lhs_eq.push_str(" - ");
        lhs_eq.push_str(&sub);
    }

    // Return the modified LHS equation
    lhs_eq
}


pub fn process(eq: &mut str, vars: HashMap<String, String>) -> String {
    let mut sub_eq = substitute(eq, vars);
    subtract(&mut sub_eq)
}