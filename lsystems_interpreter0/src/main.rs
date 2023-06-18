use turtle::*;

fn main() {
    let mut fred = Turtle::new();
    // double-quotes is a string:
    let pattern = "F+F+F+F";

    for character in pattern.chars() {
        match character {
            // single-quotes is chars:
            'F' => fred.forward(50.0),
            '+' => fred.left(90.0),
            _ => continue,
        }
    }
}