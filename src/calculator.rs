use crate::errors::CalculatorError;

#[derive(Debug)]
pub enum Token
{
    Number(f64),
    Operator(char),
}

#[derive(Debug)]
pub enum Node {
    Number(f64),
    Operator(char, Box<Node>, Box<Node>),
}

pub fn build_tree(tokens: Vec<Token>) -> Result<Node, String> {
    let mut nodes: Vec<Node> = Vec::new(); // Stos dla węzłów (liczb i poddrzew)
    let mut operators: Vec<char> = Vec::new(); // Stos dla operatorów

    // Sprawdzamy, czy liczba tokenów jest poprawna (powinna być nieparzysta: liczba, operator, liczba, ...)
    if tokens.len() % 2 == 0 || tokens.is_empty() {
        return Err("Zły format wyrażenia, kurwa!".to_string());
    }

    // Przechodzimy po tokenach
    for (i, token) in tokens.into_iter().enumerate() {
        match token {
            Token::Number(num) => {
                // Liczby wrzucamy na stos węzłów
                nodes.push(Node::Number(num));
            }
            Token::Operator(op) => {
                // Sprawdzamy, czy operator jest poprawny
                if !"+-*/".contains(op) {
                    return Err(format!("Zły operator, kurwa: {}", op));
                }

                // Sprawdzamy priorytety operatorów
                while !operators.is_empty() {
                    let last_op = *operators.last().unwrap();
                    // Priorytety: * i / mają wyższy priorytet niż + i -
                    let last_priority = if last_op == '*' || last_op == '/' { 2 } else { 1 };
                    let current_priority = if op == '*' || op == '/' { 2 } else { 1 };

                    // Jeśli operator na stosie ma wyższy lub równy priorytet, budujemy poddrzewo
                    if last_priority >= current_priority {
                        let op = operators.pop().unwrap();
                        // Bierzemy dwie ostatnie liczby/poddrzewa
                        if nodes.len() < 2 {
                            return Err("Za mało liczb, kurwa!".to_string());
                        }
                        let right = nodes.pop().unwrap();
                        let left = nodes.pop().unwrap();
                        // Tworzymy nowy węzeł i wrzucamy na stos
                        nodes.push(Node::Operator(op, Box::new(left), Box::new(right)));
                    } else {
                        break;
                    }
                }
                // Wrzucamy nowy operator na stos
                operators.push(op);
            }
        }
    }

    // Po przejściu tokenów zdejmujemy resztę operatorów ze stosu
    while !operators.is_empty() {
        let op = operators.pop().unwrap();
        if nodes.len() < 2 {
            return Err("Za mało liczb, kurwa!".to_string());
        }
        let right = nodes.pop().unwrap();
        let left = nodes.pop().unwrap();
        nodes.push(Node::Operator(op, Box::new(left), Box::new(right)));
    }

    // Na koniec powinien zostać jeden węzeł - korzeń drzewa
    if nodes.len() != 1 {
        return Err("Coś się zjebało przy budowaniu drzewa, kurwa!".to_string());
    }

    Ok(nodes.pop().unwrap())
}

pub fn evaluate(node: Node) -> Result<f64, CalculatorError> {
    match node {
        Node::Number(val) => Ok(val),
        Node::Operator(op, left, right) => {
            let left_val = evaluate(*left)?;
            let right_val = evaluate(*right)?;

            match op {
                '+' => Ok(left_val + right_val),
                '-' => Ok(left_val - right_val),
                '*' => Ok(left_val * right_val),
                '/' => {
                    if right_val == 0.0 {
                        Err(CalculatorError::DivisionByZero)
                    } else {
                        Ok(left_val / right_val)
                    }
                }
                _ => unreachable!("Nieznany operator, kurwa!"),
            }
        }
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '+' | '-' | '*' | '/' => {
                // Jeśli mamy już coś w `current`, to jest to liczba
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                // Jeśli to minus i następny znak to cyfra, to jest to liczba ujemna
                if ch == '-' {
                    if let Some(next_ch) = chars.peek() {
                        if next_ch.is_digit(10) {
                            current.push(ch); // Dodajemy minus do liczby
                            continue;
                        }
                    }
                }
                // W innym przypadku to operator
                tokens.push(ch.to_string());
            }
            '0'..='9' | '.' => {
                // Znak liczby (cyfra lub kropka)
                current.push(ch);
            }
            _ => {
                // Ignorujemy inne znaki (np. spacje)
                continue;
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

pub fn calculate_simple(tokens: Vec<Token>) -> Result<f64, CalculatorError> {
    if tokens.len() < 3 || tokens.len() % 2 == 0 {
        return Err(CalculatorError::WrongFormat(
            "Zły format, kurwa: liczba operator liczba".to_string(),
        ));
    }

    let mut result = match tokens[0] {
        Token::Number(num) => num,
        _ => return Err(CalculatorError::WrongFormat("Pierwszy token musi być liczbą, kurwa!".to_string())),
    };

    for i in (1..tokens.len()).step_by(2) {
        let op = match tokens[i] {
            Token::Operator(ch) => ch,
            _ => return Err(CalculatorError::WrongFormat("Oczekiwano operatora, kurwa!".to_string())),
        };

        let next_num = match tokens[i + 1] {
            Token::Number(num) => num,
            _ => return Err(CalculatorError::WrongFormat("Oczekiwano liczby, kurwa!".to_string())),
        };

        result = match op {
            '+' => result + next_num,
            '-' => result - next_num,
            '*' => result * next_num,
            '/' => {
                if next_num == 0.0 {
                    return Err(CalculatorError::DivisionByZero);
                }
                result / next_num
            }
            _ => return Err(CalculatorError::WrongFormat(format!("Zły operator, kurwa: {}", op))),
        };
    }

    Ok(result)
}
