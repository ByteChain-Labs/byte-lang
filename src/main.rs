mod lexer;

use lexer::scanner::Scanner;

fn main() {
    let source = "
        const x = 10;
        if (x > 5) {
            print(x);
        } else {
            print(1);
        }
    ".to_string();

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
