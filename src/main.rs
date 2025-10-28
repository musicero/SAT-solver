#[derive(Debug)]
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
    let formula = parse2("{a,b},{b,a},{c,b}");

    // for clause in formula {
    //     for literal in clause {
    //         print!("{}", literal.name);
    //     }
    // }
    print!("{:?}", formula);
}

fn unit_propagate(mut cnf: CNF, assignment: &mut Assignment) -> Option<CNF> {
    // find clauses with only one literal
    for clause in &cnf {
        // pattern match vecs with only one literal
        if let [literal] = &clause[..] {
            let value_for_satisfaction = !literal.negated; // this is the value we have to set to satisfy it

            // if literal is assigned from before
            if let Some(existing) = assignment.get(&literal.name) {
                if *existing != value_for_satisfaction {
                    return None; // unsatisfiable
                }
            } else {
                assignment.insert(literal.name.clone(), value_for_satisfaction); // if literal is negated, assign it false
            }
            break;
        }
    }
    // ---- Simplification ----
    cnf = simplify(cnf, assignment)?;

    Some(cnf)
}

fn simplify(mut cnf: CNF, assignment: &Assignment) -> Option<CNF> {
    // keep only the clauses that arent satisified
    cnf.retain(|clause| {
        clause
            .iter()
            .all(|literal| !assignment.contains_key(&literal.name))
    });

    // remove all literals assigned false
    cnf.iter_mut()
        .for_each(|clause| clause.retain(|literal| assignment.get(&literal.name) != Some(&false)));

    // check for empty clauses
    for clause in &cnf {
        if clause.is_empty() {
            return None;
        }
    }

    Some(cnf)
}

fn prove(cnf: &CNF, assignment: &mut Assignment) -> Option<Assignment> {
    // if the clause set is empty, we
    if cnf.is_empty() {
        return Some(assignment.clone());
    }

    // check for contradictions
    for clause in cnf {
        if clause.is_empty() {
            return None;
        }
    }

    Default::default()
}

/*
Result prove(Sequent s) {
    if (s is axiom) {
        return "unsatisfiable"
    else if (no more rule applications possible on s) {
        return literals in s as satisfying interpretation
    }
    else {
        pick a possible rule application
        List<Sequent> prems = premisses from that rule application
        for p in prems {
            answer = prove(s)
            if (answer is a satisfying interpretation I) {
                return I
            }
        }
        // the proofs for all premisses were closed, so...
        return "unsatisfiable";
    }
}
 */
