mod calculator;
mod errors;

use std::io;

fn main()
{
    println!("Calculator\n----------\n'q' -> quit\n");
    let mut key_input: String = String::new();


    loop
    {
        io::stdin()
            .read_line(&mut key_input)
            .expect("Blad wczytywania");

            {
                let load: &str = key_input.trim();
                if load == "q" {break;}
            }

        let load: Vec<&str> = key_input.trim().split_whitespace().collect();

        let mut digits: Vec<f64> = Vec::new();
        let mut chars: Vec<char> = Vec::new();

        for (i, token) in load.iter().enumerate()
        {
            if  i % 2 == 0
            {
                match token.parse::<f64>()
                {
                    Ok(num) => digits.push(num),
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
                    Some(ch) => chars.push(ch),
                    None =>
                    {
                        println!("Zły operator, kurwa: {}", token);
                        break;
                    }
                }
            }
        }
        println!("{:?}",digits);
        println!("{:?}",chars);
    }
}

