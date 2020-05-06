use std::io;

fn main() {
    let stdin = io::stdin();
    
    // ignore the fst line
    stdin
        .read_line(&mut " ".to_string())
        .expect("Failed to read line");
    
     let fst: Vec<i64> = read_line_vec_i64();
     let snd: Vec<i64> = read_line_vec_i64();
    
    let fst: (i64, i64) = gen_simple_fraction_top_down(&fst);
    let snd: (i64, i64) = gen_simple_fraction_top_down(&snd);
    println!("{:?}", fst);
    println!("{:?}", snd);
    
    let add = add_fractions(fst, snd);
    let sub = sub_fractions(fst, snd);
    let mult = mult_fractions(fst, snd);
    let div = div_fractions(fst, snd);
    
    println!("add {:?}", add);
    println!("sub {:?}", sub);
    println!("mult {:?}", mult);
    println!("div {:?}", div);
    
    for term in gen_continued_fraction(add) {
        print!(" {}", term);
    }
    println!("");
    for term in gen_continued_fraction(sub) {
        print!(" {}", term);
    }
    println!("");
    for term in gen_continued_fraction(mult) {
        print!(" {}", term);
    }
    println!("");
    for term in gen_continued_fraction(div) {
        print!(" {}", term);
    }
    println!("");
}

fn gen_simple_fraction_top_down(data: &[i64]) -> (i64, i64) {
    // For a given term a(k), we can compute a result p(k) / q(k)
    // We always start by defining p and q for k=0 and k=-1
    // p(-1) = 0, q(-1) = 1
    // p( 0) = 1, q( 0) = 0
    //
    // p(k) = a * p(-1) + p(-2)
    // q(k) = a * q(-1) + q(-2)
    
    let mut res = [(0, 1), (1, 0)];
    
    for term in data.iter() {
        // println!("{:#?}/{:#?}", res[1].0, res[1].1);
        let p = term * res[1].0 + res[0].0;
        let q = term * res[1].1 + res[0].1;
        res[0] = res[1];
        res[1] = (p, q);
    }
    // println!("{:#?}/{:#?}", res[1].0, res[1].1);
    res[1]
}

fn gen_continued_fraction((p, q): (i64, i64)) -> Vec<i64> {
    let negate = (p > 0 && q < 0) || (p < 0 && q > 0);
    let mut v: Vec<i64> = vec![];
    let mut p = p.abs();
    let mut q = q.abs();
    let mut temp = p % q;
    
    if temp == 0 {
        v.push(p / q);
    } else {
        while temp != 0 {
                v.push(p / q);
                temp = p % q;
                p = q;
                q = temp;
            }
            
        }
        if negate {
            // If we want to get a negated value from a positive continuedFraction, then we could either:
            // - Negate everything: negate all its terms /partial quotients
            //   or
            // - Apply "short negation": http://www.maths.surrey.ac.uk/hosted-sites/R.Knott/Fibonacci/cfINTRO.html#section13.1.2
            //   after appling "short negation", we could use 'collapse a zero' formula.

            if v.len() == 1 {
                v[0] *= -1;
            } else {
                v[0] = -v[0] - 1;
                
                if v[1] == 1 {
                    v.remove(1);
                    v[1] += 1;
                } else {
                    v.insert(1, 1);
                    v[2] -= 1;
                }
            }
        }
        // println!("{:#?}", v);
        v
    }
    
    fn add_fractions((p1, q1): (i64, i64), (p2, q2): (i64, i64)) -> (i64, i64) {
        let p1 = p1.checked_mul(q2).expect("overflow!");
        let p2 = p2.checked_mul(q1).expect("overflow!");
        let q = q1 .checked_mul(q2).expect("overflow!");
        (p1 + p2, q)
    }
    
    fn sub_fractions((p1, q1): (i64, i64), (p2, q2): (i64, i64)) -> (i64, i64) {
        let p1 = p1.checked_mul(q2).expect("overflow!");
        let p2 = p2.checked_mul(q1).expect("overflow!");
        let q = q1 .checked_mul(q2).expect("overflow!");
        (p1 - p2, q)
    }
    
    fn mult_fractions((p1, q1): (i64, i64), (p2, q2): (i64, i64)) -> (i64, i64) {
        (p1.checked_mul(p2).expect("overflow!") , q1.checked_mul(q2).expect("overflow!"))
    }
    
    fn div_fractions((p1, q1): (i64, i64), (p2, q2): (i64, i64)) -> (i64, i64) {
        (p1.checked_mul(q2).expect("overflow!") , q1.checked_mul(p2).expect("overflow!"))
    }
    
    fn read_line_vec_i64() -> Vec<i64> {
        let stdin = io::stdin();
        
        let mut val = String::new();
        stdin.read_line(&mut val).expect("Failed to read line");
        
        let line: Vec<i64> = val
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
        
        line
    }
    