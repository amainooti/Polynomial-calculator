# Goal
This is a command-line polynomial calculator. A user types in two polynomials and an operation, and the program outputs the result as a simplified polynomial. It handles addition, subtraction, multiplication, and division by a single term. It does not need to solve for x, find roots, graph anything, or handle symbolic variables other than x. Version one processes one operation at a time


## The project ladder:

**Rung 1**: A type that represents a single term. A type that represents a polynomial as a collection of terms. Can you model 3x² + 2x - 5 in your type system?</br>
**Rung 2**: A simplification function. Given a polynomial with possibly redundant like terms, produce a clean version. This is the function everything else depends on.</br>
**Rung 3**: Addition and subtraction. These are almost identical once you have simplification working.</br>
**Rung 4**: Multiplication. Slightly more involved — requires pairing every term with every other term.</br>
**Rung 5**: Display. A function that takes a polynomial and produces a clean human-readable string. 3x² + 2x - 5, not something ugly.</br>
**Rung 6**: A parser. Takes a string the user typed and produces a polynomial. This is harder than it sounds — do it last.</br>
**Rung 7**: Division by a monomial. Tackle this only after everything else is solid.</br>
**Rung 8**: A REPL-style loop so the user can keep entering expressions without restarting.


## Major Components:
Term — The base unit. Holds a coefficient (a floating point or integer number) and a degree (a non-negative integer). Two terms are "like" if their degrees match.</br>
Polynomial — An ordered collection of terms. Internally it can be messy — duplicate degrees, zero coefficients. The canonical form is always simplified and sorted by degree descending.</br>
Simplifier — Takes a polynomial in any state and returns it in canonical form. Combines like terms, removes terms with zero coefficients, sorts by degree descending. Every operation passes its result through this before returning.</br>
Arithmetic Operations — Four functions or implemented as operator overloads in Rust. Each takes two polynomials and returns a new polynomial. None of them mutate their inputs. </br>
Formatter/Display — Takes a polynomial in canonical form and produces a string. Handles edge cases: coefficient of 1 (x² not 1x²), degree of 0 (just the number), degree of 1 (x not x¹), negative terms (subtracts rather than adds a negative).</br>
Parser — Takes a string like "3x^2 + 2x - 5" and produces a Polynomial. Handles spaces, negative signs, missing coefficients, missing exponents.
Constraints:

Single variable only — x
Integer degrees only (no x^0.5)
Division scoped to monomial divisor in version one
No equation solving — input is always an expression, not an equation


```rs
if term.exp == 0 {
            print!("{}", term.coeff.abs());
        } else if term.exp == 1 {
            print!("{}x", term.coeff.abs());
        } else {
            print!("{}x^{}", term.coeff.abs(), term.exp);
        }

```