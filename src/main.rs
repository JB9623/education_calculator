struct Calculator {
    results: Vec<String>,
    current_input: String,
    total: i32,
    adds: bool,
}

impl Calculator {
    fn new() -> Self {
        Self {
            results: vec![],
            current_input: String::new(),
            total: 0,
            adds: true,
        }
    }

    fn clear(&mut self) {
        self.current_input.clear();
    }

    fn push_char(&mut self, character: char) {
        self.current_input.push(character);
    }
}

const OKAY_CHARACTERS: &str = "1234567890+- ";

fn math(input: &str) -> i32 {
    if !input
        .chars()
        .all(|character| OKAY_CHARACTERS.contains(character))
        || !input
            .chars()
            .take(2)
            .any(|character| character.is_numeric())
    {
        panic!("Please only input numbers, +-, or spaces");
    }

    let input = input
        .trim_end_matches(|x| "+- ".contains(x))
        .chars()
        .filter(|x| *x != ' ')
        .collect::<String>();
    let mut calculator = Calculator::new();

    for character in input.chars() {
        match character {
            '+' => {
                if !calculator.current_input.is_empty() {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear();
                }
            }
            '-' => {
                if calculator.current_input.contains('-') || calculator.current_input.is_empty() {
                    calculator.push_char(character);
                } else {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear();
                    calculator.push_char(character);
                }
            }
            number => {
                if calculator.current_input.contains('-') {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear();
                    calculator.push_char(number);
                } else {
                    calculator.push_char(number);
                }
            }
        }
    }
    calculator.results.push(calculator.current_input);

    for entry in calculator.results {
        if entry.contains('-') {
            if entry.chars().count() % 2 == 1 {
                calculator.adds = match calculator.adds {
                    true => false,
                    false => true,
                };
                continue;
            } else {
                continue;
            }
        }
        if calculator.adds {
            calculator.total += entry.parse::<i32>().unwrap();
        } else {
            calculator.total -= entry.parse::<i32>().unwrap();
            calculator.adds = true;
        }
    }
    calculator.total
}

fn main() {
    let val = math("1 + 1");
    println!("{val}");
}