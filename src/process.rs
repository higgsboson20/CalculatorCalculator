
/*
VarPattern = alphabetic
*/

fn is_valid_eq(eq: &str) -> bool {
    let split_eq = eq.split("=");
    return split_eq.count() == 2;
}


pub fn get_vars(eq: &str, var: &str) -> Result<Vec<String>, String> {
    let mut vars = Vec::new();
    let mut check = false;

    if !is_valid_eq(eq) {
        return Err(format!("Equation '{}' is not a valid equation.", eq));
    }

    for chr in eq.chars() {
        if chr.is_alphabetic() {
            if chr.to_string() == var {
                check = true;
            }
            vars.push(chr.to_string());
        }
    }

    if check {
        Ok(vars)
    } else {
        Err(format!("Variable '{}' not found in the equation.", var))
    }

}


