mod lexer;

use lexer::scanner::Scanner;

fn main() {
    let source = "
        contract Token {
            init {
                total_supply: unit;
            }

            func new (total_supply): Self {
                Token {
                    total_supply: total_supply;
                }
            }
        }
    ".to_string();

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
