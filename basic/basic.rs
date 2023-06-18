fn main() {
    // immutable and unused
    //let a: i32 = 42;
    /*
warning: unused variable: `a`
 --> .\basic.rs:3:9
  |
3 |     let a: i32 = 42;
  |         ^ help: if this is intentional, prefix it with an underscore: `_a`
  |
  = note: `#[warn(unused_variables)]` on by default
    */
    let _a: i32 = 42;
    //a += 42;
    /*
    error[E0384]: cannot assign twice to immutable variable `a`
 --> .\basic.rs:4:5
  |
3 |     let a: i32 = 42;
  |         -
  |         |
  |         first assignment to `a`
  |         help: consider making this binding mutable: `mut a`
4 |     a += 42;
  |     ^^^^^^^ cannot assign twice to immutable variable

error: aborting due to previous error; 2 warnings emitted
    */


    //add numbers
    //1+1;
    /*
32 |     1+1;
   |     ^^^ the arithmetic operation produces a value
   |
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
32 |     let _ = 1+1;
   |     +++++++
    */
    let _ = 1+1;

    //40 + 2.0
    /*
error[E0277]: cannot add a float to an integer
  --> .\basic.rs:33:8
   |
33 |     40 + 2.0
   |        ^ no implementation for `{integer} + {float}`
   |
   = help: the trait `Add<{float}>` is not implemented for `{integer}`
    */

    // typing in-line and no mix between types
    //1_i32 + 1_i64;
    /*
    error[E0277]: cannot add `i64` to `i32`
  --> .\basic.rs:45:11
   |
45 |     1_i32 + 1_i64;
   |           ^ no implementation for `i32 + i64`
   |

    */

    // number methods
    let _ = (42.42_f32).floor();
    let _ = (4.2_f32).powi(2);
    let _ = (5.2_f64).powf(3.3);

    // everything emits a value, expression-based language
    let the_answer: i32 = if 1 == 2 {
        3
    } else {
        42
    };

    // match is like switch
    let _the_question = match the_answer {
        3 => "some other universe",
        42 => "how many roads should one walk?",
        _ => "some think it has already happened",
    };

}