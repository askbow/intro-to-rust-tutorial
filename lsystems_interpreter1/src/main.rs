use turtle::*;

const N: i32 = 3;

fn main() {
    let mut fred = Turtle::new();
    let mut pattern = String::from(s:"F+F+F+F");

    let mut rules = Vec::<(u8, String)>::new();

    let rewrite: String = "F-F-F".into();
    let rule:(u8, String) = (b'F', rewrite);
    rules.push(rule);

    for _ in 0..N {
        let mut next_pattern = String::new();
        'patterns: for p:u8 in pattern.bytes() {
            for (matcher, rewrite) in &rules {
                if (*matcher == p) {
                    next_pattern.push_str(rewrite);
                    continue 'patterns; // effectively a GOTO label
                }
            }

            next_pattern.push(p.into());
        }
        pattern = next_pattern;
    }

    for character in pattern.chars() {
        match character {
            'F' => fred.forward(50.0),
            '+' => fred.left(90.0),
            '-' => fred.right(90.0),
            _ => continue,
        }
    }
}