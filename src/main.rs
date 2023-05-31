mod expression;

fn main() {
    loop {
        let input = inquire::Text::new("Enter an expression:").prompt().unwrap();
        if input == "exit" {
            return;
        }

        match expression::Expression::try_from(input) {
            Ok(expression) => {
                match expression.calculate() {
                    Ok(result) => println!("Result: {}", result),
                    Err(err) => println!("{err}"),
                }
            },
            Err(err) => println!("{err}"),
        };

        println!();
    }
}