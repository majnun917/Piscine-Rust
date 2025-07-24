use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";
    let mut trials = 0;

    loop {
        println!("{}", riddle);

        let mut by_user = String::new();
        io::stdin()
            .read_line(&mut by_user)
            .expect("Failed to read line");

        trials += 1;

        if by_user.trim() == correct_answer {
            println!("Number of trials: {}", trials);
            break;
        }
    }
}
