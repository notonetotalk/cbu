use rand::Rng;
use std::io;
use std::io::Write;

// keeps track of number of tries
struct Tries {
    init: u8,
    left: u8,
}

impl Tries {
    fn init(x: u8) -> Tries {
        Tries { init: x, left: x }
    }
    // decrement number of tries left
    fn dec(&mut self) {
        self.left = self.left - 1u8;
    }
    // pass total tries used
    fn total(&self) -> &'static str {
        match self.init - self.left {
            5u8 => "five tries",
            4u8 => "four tries",
            3u8 => "three tries",
            2u8 => "two tries",
            _ => "ONE TRY",
        }
    }
    // out put tries left as a word to avoid confusion
    fn left_str(&self) -> &'static str {
        match self.left {
            4u8 => "four",
            3u8 => "three",
            _ => "two",
        }
    }
}

struct Code {
    value: [char; 3],
}

impl Code {
    // generates randomized Code struct
    fn gen_rng() -> Code {
        let mut rng_letters: [char; 3] = ['A', 'A', 'A'];
        let mut rng = rand::thread_rng();
        for i in 0..3 {
            match rng.gen_range(0..3) {
                0 => (),
                1 => rng_letters[i] = 'B',
                _ => rng_letters[i] = 'C',
            }
        }
        Code { value: rng_letters }
    }
    // generates Code from input
    fn gen_from_input(s: &str) -> Code {
        let mut input_arr: [char; 3] = ['A', 'A', 'A'];
        for i in 0..3 {
            match s.chars().nth(i).unwrap() {
                'A' | 'a' => (),
                'B' | 'b' => input_arr[i] = 'B',
                _ => input_arr[i] = 'C',
            }
        }
        Code { value: input_arr }
    }
    // compares self to another Code
    // returns number of letters shared in the same indexes
    fn cmp(&self, abc: &Code) -> u8 {
        let mut m = 0u8;
        for i in 0..3 {
            if &abc.value[i] == &self.value[i] {
                m += 1u8;
            }
        }
        m
    }
}

fn verify_input(input: &str) -> [bool; 2] {
    match input {
        "quit" | "exit" | "q" => {
            return [true, false]; // valid input, exit
        }
        _ => {
            if input.chars().count() == 3 {
                for i in 0..3 {
                    match input.chars().nth(i).unwrap() {
                        'A' | 'B' | 'C' | 'a' | 'b' | 'c' => (),
                        _ => return [false, true], // invalid input
                    }
                }
                return [true, true]; // valid input
            } else {
                return [false, true]; // invalid input
            }
        }
    }
}

fn rng_death(deaths: [&'static str; 10]) -> &'static str {
    let i = rand::thread_rng().gen_range(0..10);
    deaths[i]
}

fn main() {
    const DEATH_LIST: [&'static str; 10] = [
        "Killed by the Paper Cut Serial Killer.",
        "Died while tring to traverse the code maze.",
        "The floor gave way to a lava pit.",
        "Killed while trying to escape the testing facility.",
        "The monster in the maze found you.",
        "You killed yourself after succumbing to pointlessness.",
        "Decapitated by a slow-moving truck.",
        "Killed by the Guessing Allotment Police.",
        "A laser-shooting robot arrived to kill you.",
        "You killed yourself after losing hope of ever solving this puzzle.",
    ];

    let mut play_again: bool = true;

    while play_again {
        let secret_code = Code::gen_rng();
        let mut tries = Tries::init(5u8);

        println!("\n    Try to find the secret code.");
        println!("    The code is a series of 3 letters.");
        println!("    They are each either \"A\", \"B\", or \"C\".");
        print!("    Input your first guess: ");
        io::stdout().flush().unwrap();

        play_again = loop {
            // take user input
            let mut input_string = String::new();
            match io::stdin().read_line(&mut input_string) {
                Ok(_) => (),
                Err(_) => {
                    println!("\n    Error when processing your input.");
                    print!("    Please try again: ");
                    io::stdout().flush().unwrap();
                    continue;
                }
            };

            let input_string: &str = input_string.trim();

            let [is_input_valid, keep_playing] = verify_input(input_string);

            if is_input_valid == false {
                println! {"\n    That is not a valid input."};
                print! {"    Please try again: "};
                io::stdout().flush().unwrap();
                continue;
            } else if keep_playing == false {
                break false;
            }

            // convert input into a Code struct
            let input_code = Code::gen_from_input(input_string);

            // save number of matching letters from guess into a variable
            let compared: u8 = secret_code.cmp(&input_code);

            tries.dec();

            if compared == 3u8 {
                println!(
                    "\n    \"{}{}{}\" is the correct secret code.",
                    secret_code.value[0], secret_code.value[1], secret_code.value[2]
                );
                println!("    Note: Sucessful attempt after {}.", tries.total());
                println!("    Thank you for your cooperation. Goodbye.\n");
                break true;
            } else if compared == 1u8 {
                println!(
                    "\n    1 character in \"{}{}{}\" is correct.",
                    input_code.value[0], input_code.value[1], input_code.value[2]
                );
            } else {
                println!(
                    "\n    {} of the characters in \"{}{}{}\" are correct.",
                    compared, input_code.value[0], input_code.value[1], input_code.value[2]
                );
            }

            // checks how many tries left or if user has lost
            if tries.left == 0u8 {
                println!("    You've run out of guesses alloted.");
                println!(
                    "    The secret code was \"{}{}{}\".",
                    secret_code.value[0], secret_code.value[1], secret_code.value[2]
                );
                println!("\n    You are now dead.");
                println!("    {}", rng_death(DEATH_LIST));
                println!("    Thank you for your cooperation. Goodbye.\n");
                break true;
            } else if tries.left == 1u8 {
                println!("    You have one guess remaining.");
            } else {
                println!("    You have {} guesses remaining.", tries.left_str());
            }

            print!("    Input your next guess: ");
            io::stdout().flush().unwrap();
        }; // end of loop

        if play_again == true {
            print!("      Play again? (y/n): ");
            io::stdout().flush().unwrap();

            let mut again_string = String::new();
            match io::stdin().read_line(&mut again_string) {
                Ok(_) => {
                    if again_string.trim() != "y" {
                        play_again = false;
                    }
                }
                Err(_) => play_again = false,
            };
        }
    } // end of while
    println!("");
}
