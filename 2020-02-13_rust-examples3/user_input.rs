use std::io;

fn main() {

    let x = Some(Some(Some(Ok(5))));

    match x {
        Some(Some(Some(Ok(number)))) => dosomething(),
        _ => somesometingelse();
    }

    'outerloop: loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Couldn't read.");

        for character in user_input.chars() {
            match character {
                'q' => break 'outerloop,
                c @ '0'..='4' => println!("Digit from 0 to 4"),
                c @ call_your_func() => 
                _ => println!("Something else.")
            }
        }
    }
}
