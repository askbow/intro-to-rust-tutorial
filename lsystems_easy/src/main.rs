use turtle::*;

fn main() {
    // Fred is mutable. Mutable Ninja Turtle.
    let mut fred = Turtle::new(); // fred's a good turtle name, don't you think?

    fred.hide();
    fred.forward(50.0);
    fred.left(90.0);
    fred.forward(50.0);
    fred.left(90.0);
    fred.forward(50.0);
    fred.left(90.0);
    fred.forward(50.0);
    fred.left(90.0);
}
