pub enum Operator { // enum that differentiates different operations
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
}

pub enum Token { // enum that differentiates between an operator or operand
    Operator(Operator),
    Operand(isize),
}

pub fn eval(tokens: &[Token]) -> Option<isize> { // takes array of tokens and returns option

    let mut stack: Vec<isize> = Vec::new();// creation of a stack

    for token in tokens{//loops through tokens

        match *token{
            Token::Operand(ref token) => stack.push(*token),//if operand pushes in stack
            Token::Operator(ref token) => {//if operator
                    if stack.len() < 2{
                        return None;
                    }
                    else{
                        let x1 = stack.pop().unwrap();//pops one number
                        let x2 = stack.pop().unwrap();//pops the 2nd number
                        match *token {
                            Operator::Add => stack.push(x1 + x2),
                            Operator::Sub => stack.push(x2 - x1),
                            Operator::Mul => stack.push(x1 * x2),
                        }
                    }

            },
        }
        if stack.len() == 0{
            return None
        }
    }
    if stack.len() == 1{
        return Some(stack.pop().unwrap());// then returns last number on the stack.
    }
    else{
        return None;
    }
}
