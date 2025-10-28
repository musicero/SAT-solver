#[derive(Debug, Clone)]
struct Literal {
    name: String,
    negated: bool,
}
type Clause = Vec<Literal>;
type CNF = Vec<Clause>;

use std::collections::HashMap;
type Assignment = HashMap<String, bool>;

// DPLL has 4 steps:
// 1. Unit propagation
// 2. Pure literal elimination
// 3. Branching
// 5. Termination

fn parse2(formula: &str) -> CNF {
    formula
        .replace(" ", "")
        .split("},")
        .map(|s| {
            s.replace("{", "")
                .replace("}", "")
                .split(",")
                .map(|mut exp| {
                    let mut negated = false;
                    if exp.contains('-') {
                        exp = exp.trim_matches('-');
                        negated = true;
                    }
                    Literal {
                        name: exp.to_string(),
                        negated,
                    }
                })
                .collect()
        })
        .collect()
}

fn main() {
    // let formula = parse2("{a,b},{b,a},{c,b}");
    // let formula = parse2("{a}, {b,c}");
    let formula = parse2("{a,b}");

    println!("{:?}", formula);

    // for clause in formula {
    //     for literal in clause {
    //         print!("{}", literal.name);
    //     }
    // }

    let mut assignment = HashMap::new();

    dpll(formula, &mut assignment);

    print!("{:?}", assignment)
}

fn dpll(cnf: CNF, assignment: &mut Assignment) -> Option<Assignment> {
    let cnf = unit_propagate(cnf, assignment)?;

    if cnf.is_empty() {
        return Some(assignment.clone()); // all clauses satisfied
    }

    if cnf.iter().any(|clause| clause.is_empty()) {
        return None; // conflict
    }

    // find a literal that is not yet assigned
    let literal = match pick_literal(&cnf, &assignment) {
        Some(lit) => lit,
        None => {
            if cnf.is_empty() {
                return Some(assignment.clone());
            } else {
                // unsatisfiable -> we have
                return None;
            }
        }
    };

    // branch
    for value in [true, false] {
        let mut assignment = assignment.clone();
        assignment.insert(literal.name.clone(), value);
        if let Some(result) = dpll(cnf.clone(), &mut assignment) {
            return Some(result);
        }
    }

    // neither branch was satisfiable
    None
}

fn pick_literal(cnf: &CNF, assignment: &Assignment) -> Option<Literal> {
    for clause in cnf {
        for literal in clause {
            if !assignment.contains_key(&literal.name) {
                return Some(literal.clone());
            }
        }
    }
    None
}

fn unit_propagate(mut cnf: CNF, assignment: &mut Assignment) -> Option<CNF> {
    loop {
        // pick clause
        let unit = cnf.iter().find_map(|clause| {
            // pattern match literal with slice representation of clause
            if let [literal] = &clause[..] {
                Some(literal)
            } else {
                None
            }
        });

        // shadow unwrap
        let unit = match unit {
            Some(lit) => lit,
            None => break, // no more unit clauses
        };

        // value we want to assign
        let value_to_assign = !unit.negated;
        if let Some(&existing) = assignment.get(&unit.name) {
            // if existing value and value_to_assign does not match, it is unsatisfiable
            if existing != value_to_assign {
                return None;
            }
        } else {
            assignment.insert(unit.name.clone(), value_to_assign);
        }

        cnf = simplify(cnf, assignment)?;
    }

    // in the end return the simplified formula
    Some(cnf)
}

fn simplify(mut cnf: CNF, assignment: &Assignment) -> Option<CNF> {
    // keep only the clauses that arent satisified
    cnf.retain(|clause| {
        !clause.iter().any(|literal| {
            match assignment.get(&literal.name) {
                Some(&value) => value != literal.negated, // literal evaluates to true?
                None => false,
            }
        })
    });

    // remove all literals assigned false
    for clause in cnf.iter_mut() {
        clause.retain(|literal| match assignment.get(&literal.name) {
            Some(&value) => value == literal.negated, // keep if literal is not false
            None => true,
        })
    }

    // check for empty clauses
    for clause in &cnf {
        if clause.is_empty() {
            return None;
        }
    }

    Some(cnf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_formula_1() {
        let cnf = parse2("{a}");
        let mut assignment = HashMap::new();
        let result = dpll(cnf, &mut assignment);
        assert!(result.is_some());
    }

    #[test]
    fn test_formula_2() {
        let cnf = parse2("{a},{b}");
        let mut assignment = HashMap::new();
        let result = dpll(cnf, &mut assignment);
        assert!(result.is_some());
    }

    #[test]
    fn test_formula_3() {
        let cnf = parse2("{a,b}");
        let mut assignment = HashMap::new();
        let result = dpll(cnf, &mut assignment);
        assert!(result.is_some());
    }

    #[test]
    fn test_formula_4() {
        let cnf = parse2("{-a,b}");
        let mut assignment = HashMap::new();
        let result = dpll(cnf, &mut assignment);
        assert!(result.is_some());
    }

    #[test]
    fn test_formula_5() {
        let cnf = parse2("{a},{-a}");
        let mut assignment = HashMap::new();
        let result = dpll(cnf, &mut assignment);
        assert!(result.is_none());
    }

    #[test]
    fn test_formula_6() {
        let cnf = parse2("{a,b},{-a,c},{-b,-c}");
        let mut assignment = HashMap::new();
        let result = dpll(cnf, &mut assignment);
        assert!(result.is_some());
    }

    #[test]
    fn test_formula_7() {
        let cnf = parse2("{a,b},{b,c},{-a,-b},{-c,d}");
        let mut assignment = HashMap::new();
        let result = dpll(cnf, &mut assignment);
        assert!(result.is_some());
    }

    #[test]
    fn test_formula_8() {
        let cnf = parse2("{a,b},{-a,b},{a,-b},{-a,-b}");
        let mut assignment = HashMap::new();
        let result = dpll(cnf, &mut assignment);
        assert!(result.is_none());
    }
}
