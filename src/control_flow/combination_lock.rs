use std::io::stdin;

pub fn combination_lock() {
    enum State {
        Locked,
        Failed,
        Unlocked,
    }

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry_code = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();

                println!("Please enter your code: ");
                // stdin().read_line(&mut input).expect("Failed to read line");
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        println!("You entered: {}", input.trim_end());
                        entry_code.push_str(&input);
                    }
                    Err(_) => continue,
                };

                if entry_code == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry_code) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED!");
                entry_code.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED!");
                return;
            }
        }
    }
}
