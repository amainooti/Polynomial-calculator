use std::collections::HashMap;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;

#[derive(Clone)]
struct Term {
    deg: u32,
    coeff: i64,
}

struct Poly {
    terms: Vec<Term>,
}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        if self.terms.is_empty() {
                write!(f, "0")?;
            }

        for term in &self.terms {
            let abs_coeff = term.coeff.abs();

            // ── Sign ─────────────────────────────
            if !first {
                if term.coeff >= 0 {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            } else if term.coeff < 0 {
                // first term and negative
                write!(f, "-")?;
            }

            // ── Coefficient ──────────────────────
            if abs_coeff != 1 || term.deg == 0 {
                write!(f, "{}", abs_coeff)?;
            }

            // ── Variable ─────────────────────────
            if term.deg == 1 {
                write!(f, "x")?;
            } else if term.deg > 1 {
                write!(f, "x^{}", term.deg)?;
            }


            first = false;
        }

        Ok(())
    }
}

impl Add for &Poly {
    // Function: implements addition of two polynomials. 
    // that returns a new Poly that is the sum of the two polynomials.

    type Output = Poly;
    
    fn add(self, other: Self) -> Poly {
        // Combine terms from both polynomials
        let mut combined_terms: Vec<Term> = self.terms.iter().cloned().collect(); // this is a vector of terms from the first polynomial

        combined_terms.extend(other.terms.iter().cloned()); // this adds the terms from the second polynomial to the combined_terms vector
        // Create a new polynomial with the combined terms
        let combined_poly = Poly { terms: combined_terms };
        // Simplify the combined polynomial to combine like terms
        simplify(&combined_poly)
        
    }
}


impl Sub for &Poly {
    type Output = Poly;

    fn sub(self, other: Self) -> Poly {
        // Negate the second polynomial's coefficients and add
        let negated_other_terms: Vec<Term> = other.terms.iter().map(|t| Term { deg: t.deg, coeff: -t.coeff }).collect();
        let negated_other_poly = Poly { terms: negated_other_terms };
        self + &negated_other_poly
    }
}

// TODO: create a simplify function that takes a Poly and returns a new 
// Poly in canonical form. Canonical means: 
// like terms are combined, terms with a zero coefficient are removed, 
// and terms are sorted by degree descending.


fn simplify(poly: &Poly) -> Poly{
    // create a hashmap to store the degree and its corresponding coefficient
    let mut map: HashMap<u32, i64> = HashMap::new();


    // ── Phase 1: Aggregate coefficients by degree ───────────────
    for term in &poly.terms {
        *map.entry(term.deg).or_insert(0) += term.coeff;
    }

    // ── Phase 2: Remove zero coefficients + rebuild terms ───────
    let mut terms: Vec<Term> = map
        .into_iter()
        .filter(|(_, coeff)| *coeff != 0)
        .map(|(deg, coeff)| Term { deg, coeff })
        .collect();

    // ── Phase 3: Sort terms by degree descending ───────────────
    terms.sort_by(|a, b| b.deg.cmp(&a.deg));

    Poly { terms }

}

fn main() {
    let poly = Poly {
        terms: vec![
            Term { deg: 2, coeff: 3 },
            Term { deg: 1, coeff: 2 },
            Term { deg: 2, coeff: 1 },
            Term { deg: 0, coeff: -5 },
            Term { deg: 1, coeff: 4 },
        ],
    };

    let simplified = simplify(&poly); 
    // Sum is the result of adding the simplified polynomial to itself.
    let sum = &simplified + &simplified; 
    let sub = &simplified - &simplified; // this should yield the zero polynomial
    println!("Original: {}", poly);
    println!("Simplified: {}", simplified);
    println!("Sum: {}", sum);
    println!("Sub: {}", sub);
}