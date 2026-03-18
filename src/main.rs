use std::collections::HashMap;
use std::fmt;

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

    println!("Original: {}", poly);
    println!("Simplified: {}", simplified);
}