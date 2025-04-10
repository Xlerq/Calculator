mod calculator;
mod errors;

use std::io;
use calculator::Token;
use calculator::build_tree;
use calculator::evaluate;


fn main()
{
    print!("Calculator\n----------\n'q' -> quit\n");


    loop
    {
        let mut key_input: String = String::new();
        io::stdin()
            .read_line(&mut key_input)
            .expect("Blad wczytywania");

            {
                let load: &str = key_input.trim();
                if load == "q" {break;}
            }

        let load: Vec<&str> = key_input.trim().split_whitespace().collect();

        let mut main_vec: Vec<Token> = Vec::new();

        for (i, token) in load.iter().enumerate()
        {
            if  i % 2 == 0
            {
                match token.parse::<f64>()
                {
                    Ok(num) => main_vec.push(Token::Number(num)),
                    Err(_) =>
                    {
                        println!("Zła liczba, kurwa: {}", token);
                        break;
                    }
                }
            }
            else
            {
                match token.chars().next()
                {
                    Some(ch) => main_vec.push(Token::Operator(ch)),
                    None =>
                    {
                        println!("Zły operator, kurwa: {}", token);
                        break;
                    }
                }
            }
        }

        match build_tree(main_vec)
        {
            Ok(tree) => match evaluate(tree)
            {
                Ok(result) => println!("{}", result),
                Err(e) => println!("Błąd, kurwa: {}", e),
            },
            Err(e) => println!("Błąd przy budowaniu drzewa, kurwa: {}", e),
        }
    }
}

