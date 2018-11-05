struct Term {
    degree: i32,
    coefficient: i32
}

struct Polynomial {
    terms: Vec<Term>
}

fn evaluate_polynomial(p: Polynomial, x: i32) -> i32 {
    let mut sum = 0;
    for term in &p.terms {
        sum += x.pow(term.degree as u32) * term.coefficient;
    };
    return sum
}

fn main() {
    let a = Term { degree: 2, coefficient: 1 };
    let b = Term { degree: 1, coefficient: 2 };
    let c = Term { degree: 0, coefficient: 1 };
    
    let x = vec!{a,b,c};
    
    let my_poly = Polynomial { terms: x };

    let o = evaluate_polynomial(my_poly, 3);

    println!("{}", o);
}
