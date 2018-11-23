#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

impl Operator{
    fn precedence_checker(&self) -> i32 {
        match *self{
            Operator::Add | Operator::Sub => 0,
            Operator::Mul | Operator::Div => 1,

        }
    }
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {

    let mut operator_stack: Vec<InfixToken> = Vec::new();// creation of a stack
    let mut output_stack: Vec<PostfixToken> = Vec::new();// creation of a stack
    let mut parentheis_stack: Vec<InfixToken> = Vec::new();
    let mut num_operator_tokens = 0;
    let ref first_token = tokens[0];

    let mut dummy_var = 0;
    let mut left_paren_count = 0;
    let mut right_paren_count = 0;
    for token in tokens{
        match *token{
            InfixToken::Operand(ref token) =>  dummy_var = dummy_var + 1,
            InfixToken::Operator(ref token) => dummy_var = dummy_var + 1,
            InfixToken::LeftParen => left_paren_count = left_paren_count + 1,
            InfixToken::RightParen => right_paren_count = right_paren_count + 1,

        }
        if right_paren_count > left_paren_count{
            return None;
        }
    }
    if right_paren_count != left_paren_count{
        return None;
    }

    match *first_token{
        InfixToken::Operand(first_token) => {},
        InfixToken::Operator(first_token)=> return None,
        InfixToken::LeftParen => {},
        InfixToken::RightParen => return None,

    }
    if !tokens.is_empty(){
        let ref last_token = tokens[tokens.len() - 1];
        match *last_token{ // Last token
            InfixToken::Operand(last_token) => {},
            InfixToken::Operator(last_token)=> return None,
            InfixToken::LeftParen => return None,
            InfixToken::RightParen => {},
        }
    }
    if tokens.len() >= 3{
        let mut i = 1;
        while i != tokens.len() - 2{
            let ref prev_token = tokens[i - 1];
            let ref next_token = tokens[i + 1];
            let ref cur_token = tokens[i];
            if tokens[i] == InfixToken::LeftParen{
                match *prev_token{ // Last token
                    InfixToken::Operand(prev_token) => return None,
                    InfixToken::Operator(prev_token)=> {},
                    InfixToken::LeftParen => {},
                    InfixToken::RightParen => return None,
                }
                match *next_token{ // Last token
                    InfixToken::Operand(next_token) => {},
                    InfixToken::Operator(next_token)=> return None,
                    InfixToken::LeftParen => {},
                    InfixToken::RightParen => return None,
                }
            }
            if tokens[i] == InfixToken::RightParen{
                match *prev_token{ // Last token
                    InfixToken::Operand(prev_token) => {},
                    InfixToken::Operator(prev_token)=> return None,
                    InfixToken::LeftParen => return None,
                    InfixToken::RightParen => {},
                }
                match *next_token{ // Last token
                    InfixToken::Operand(next_token) => return None,
                    InfixToken::Operator(next_token)=> {},
                    InfixToken::LeftParen => return None,
                    InfixToken::RightParen => {},
                }
            }
            match *cur_token{
                InfixToken::Operand(cur_token) => {
                    match *prev_token{ // Last token
                        InfixToken::Operand(prev_token) => return None,
                        InfixToken::Operator(prev_token)=> {},
                        InfixToken::LeftParen => {},
                        InfixToken::RightParen => return None,
                    }
                    match *next_token{ // Last token
                        InfixToken::Operand(next_token) => return None,
                        InfixToken::Operator(next_token)=> {},
                        InfixToken::LeftParen => return None,
                        InfixToken::RightParen => {},
                    }
                },
                InfixToken::Operator(cur_token)=> {},
                InfixToken::LeftParen => {},
                InfixToken::RightParen => {},

            }

            i = i + 1;
        }
    }

    for token in tokens{//loops through tokens

        match *token{
            InfixToken::Operand(ref token) => output_stack.push(PostfixToken::Operand(*token)),
            InfixToken::Operator(ref token) => {
                                                if operator_stack.len() == 0{
                                                    match *token {
                                                        Operator::Add => operator_stack.push(InfixToken::Operator(*token)),
                                                        Operator::Sub => operator_stack.push(InfixToken::Operator(*token)),
                                                        Operator::Mul => operator_stack.push(InfixToken::Operator(*token)),
                                                        Operator::Div => operator_stack.push(InfixToken::Operator(*token)),
                                                    }

                                                }

                                                else{

                                                    let tos = operator_stack.pop().unwrap();
                                                    if tos == InfixToken::LeftParen{
                                                        operator_stack.push(tos);
                                                        operator_stack.push(InfixToken::Operator(*token));
                                                    }
                                                    else{
                                                        let y;
                                                        // changes to operator type
                                                        match tos{
                                                            InfixToken::Operator(tos) => match tos{
                                                            Operator::Add =>y = tos,
                                                            Operator::Sub =>y = tos,
                                                            Operator::Mul =>y = tos,
                                                            Operator::Div =>y = tos,
                                                            },
                                                            //InfixToken::LeftParen => {},
                                                            _ => return None,
                                                        }
                                                        //keep track of tos
                                                        // y becomes tos in operator type
                                                        if y.precedence_checker() >= token.precedence_checker(){
                                                            num_operator_tokens = operator_stack.len();
                                                            output_stack.push(PostfixToken::Operator(y));

                                                            while num_operator_tokens != 0{//dumps all operators onto stack
                                                                let mut x;
                                                                let output_operator = operator_stack.pop().unwrap();
                                                                if output_operator == InfixToken::LeftParen{
                                                                    break;
                                                                }
                                                                match output_operator{
                                                                    InfixToken::Operator(output_operator) => match output_operator{
                                                                        Operator::Add =>x = output_operator,
                                                                        Operator::Sub =>x = output_operator,
                                                                        Operator::Mul =>x = output_operator,
                                                                        Operator::Div =>x = output_operator,

                                                                    },
                                                                    _ => return None,
                                                                }
                                                                output_stack.push(PostfixToken::Operator(x));
                                                            }
                                                            operator_stack.push(InfixToken::Operator(*token));
                                                        }

                                                        else{
                                                            operator_stack.push(tos);
                                                            operator_stack.push(InfixToken::Operator(*token));
                                                        }
                                                    }
                                                }

                                            },
            InfixToken::LeftParen => operator_stack.push(InfixToken::LeftParen),
            InfixToken::RightParen => {
                                        // in my operator stack i only have opearators and left parenthesis
                                        let mut k; // will become operator type of infix token.
                                        while !operator_stack.is_empty(){
                                            let popped_operator = operator_stack.pop().unwrap();

                                            if popped_operator == InfixToken::LeftParen{
                                                break;
                                            }
                                            else{
                                                match popped_operator{
                                                    InfixToken::Operator(popped_operator) => match popped_operator{
                                                                    Operator::Add =>k = popped_operator,
                                                                    Operator::Sub =>k = popped_operator,
                                                                    Operator::Mul =>k = popped_operator,
                                                                    Operator::Div =>k = popped_operator,
                                                                },
                                                                _ => return None,
                                                            }
                                                            output_stack.push(PostfixToken::Operator(k));
                                         }
                                        }


            },

        }
    }
    if operator_stack.len() != 0{
        while operator_stack.len() != 0 { // gets rid of all operators at the end and pushes to output_stack
            let popped_operator_2 = operator_stack.pop().unwrap();
            let mut g;
            match popped_operator_2{
                InfixToken::Operator(popped_operator_2) => match popped_operator_2{
                                Operator::Add =>g = popped_operator_2,
                                Operator::Sub =>g = popped_operator_2,
                                Operator::Mul =>g = popped_operator_2,
                                Operator::Div =>g = popped_operator_2,
                            },
                            _ => return None,
            }
        output_stack.push(PostfixToken::Operator(g));
        }
    }

    if output_stack.len() == 0{
        return None;
    }

    if output_stack.len() == 1{
        return Some(output_stack);
    }
    else{
        return Some(output_stack);
    }
    //pop the operator stack into the output vector HERE

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {

        let result = vec![PostfixToken::Operand(1),PostfixToken::Operand(2),PostfixToken::Operand(3),PostfixToken::Operator(Operator::Add),PostfixToken::Operand(4),PostfixToken::Operand(5),PostfixToken::Operator(Operator::Mul),PostfixToken::Operator(Operator::Mul),PostfixToken::Operator(Operator::Add)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::Operand(1),InfixToken::Operator(Operator::Add),InfixToken::LeftParen,InfixToken::LeftParen,InfixToken::Operand(2), InfixToken::Operator(Operator::Add),InfixToken::Operand(3),InfixToken::RightParen, InfixToken::Operator(Operator::Mul),InfixToken::LeftParen,InfixToken::Operand(4),InfixToken::Operator(Operator::Mul),InfixToken::Operand(5),InfixToken::RightParen,InfixToken::RightParen,InfixToken::RightParen]));
    }
    #[test]
    fn test_1(){
        let result = vec![PostfixToken::Operand(1),PostfixToken::Operand(2),PostfixToken::Operator(Operator::Add),PostfixToken::Operand(3),PostfixToken::Operator(Operator::Add),PostfixToken::Operand(4),PostfixToken::Operator(Operator::Add),PostfixToken::Operand(5),PostfixToken::Operator(Operator::Add),PostfixToken::Operand(6),PostfixToken::Operator(Operator::Add),PostfixToken::Operand(7),PostfixToken::Operator(Operator::Add)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::LeftParen,InfixToken::LeftParen,InfixToken::LeftParen,InfixToken::LeftParen,InfixToken::LeftParen,InfixToken::Operand(1),InfixToken::Operator(Operator::Add),InfixToken::Operand(2),InfixToken::RightParen,InfixToken::Operator(Operator::Add),InfixToken::Operand(3),InfixToken::RightParen,InfixToken::Operator(Operator::Add),InfixToken::Operand(4),InfixToken::RightParen,InfixToken::Operator(Operator::Add),InfixToken::Operand(5),InfixToken::RightParen,InfixToken::Operator(Operator::Add),InfixToken::Operand(6),InfixToken::RightParen,InfixToken::Operator(Operator::Add),InfixToken::Operand(7),InfixToken::RightParen]));
    }
    #[test]
    fn tests_2(){
        let result = vec![PostfixToken::Operand(2), PostfixToken::Operand(2), PostfixToken::Operator(Operator::Add),PostfixToken::Operand(2), PostfixToken::Operand(2), PostfixToken::Operator(Operator::Add),PostfixToken::Operator(Operator::Add)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::Operand(2), InfixToken::Operator(Operator::Add), InfixToken::Operand(2),InfixToken::RightParen, InfixToken::Operator(Operator::Add),InfixToken::LeftParen,InfixToken::Operand(2), InfixToken::Operator(Operator::Add), InfixToken::Operand(2),InfixToken::RightParen ]));
    }
    #[test]
    fn test_3() {

        let result = vec![PostfixToken::Operand(5),PostfixToken::Operand(1),PostfixToken::Operator(Operator::Add)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::Operand(5),InfixToken::RightParen,InfixToken::Operator(Operator::Add),InfixToken::LeftParen,InfixToken::Operand(1),InfixToken::RightParen]));
    }
    #[test]
    fn test_4() {

        let result = vec![PostfixToken::Operand(1),PostfixToken::Operand(2),PostfixToken::Operator(Operator::Mul),PostfixToken::Operand(3),PostfixToken::Operand(4),PostfixToken::Operator(Operator::Div),PostfixToken::Operator(Operator::Add)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::Operand(1),InfixToken::Operator(Operator::Mul),InfixToken::Operand(2),InfixToken::Operator(Operator::Add),InfixToken::LeftParen,InfixToken::Operand(3),InfixToken::Operator(Operator::Div),InfixToken::Operand(4),InfixToken::RightParen,InfixToken::RightParen]));
    }
    #[test]
    fn test_5() {

        let result = vec![PostfixToken::Operand(5)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::LeftParen,InfixToken::LeftParen,InfixToken::Operand(5),InfixToken::RightParen,InfixToken::RightParen,InfixToken::RightParen]));
    }
    #[test]
    fn test_6() {

        let result = vec![PostfixToken::Operand(2),PostfixToken::Operand(1),PostfixToken::Operand(3),PostfixToken::Operator(Operator::Add),PostfixToken::Operator(Operator::Div),PostfixToken::Operand(5),PostfixToken::Operand(4),PostfixToken::Operator(Operator::Mul),PostfixToken::Operand(7),PostfixToken::Operand(2),PostfixToken::Operator(Operator::Sub),PostfixToken::Operator(Operator::Mul),PostfixToken::Operator(Operator::Mul)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::Operand(2),InfixToken::Operator(Operator::Div),InfixToken::LeftParen,InfixToken::Operand(1),InfixToken::Operator(Operator::Add),InfixToken::Operand(3),InfixToken::RightParen,InfixToken::RightParen,InfixToken::Operator(Operator::Mul),InfixToken::LeftParen,InfixToken::Operand(5),InfixToken::Operator(Operator::Mul),InfixToken::Operand(4),InfixToken::Operator(Operator::Mul),InfixToken::LeftParen,InfixToken::Operand(7),InfixToken::Operator(Operator::Sub),InfixToken::Operand(2),InfixToken::RightParen,InfixToken::RightParen]));
    }
    #[test]
    fn test_7() {

        let result = vec![PostfixToken::Operand(5)];//vec![PostfixToken::Operand(1),PostfixToken::Operand(2),PostfixToken::Operand(3),PostfixToken::Operator(Operator::Add),PostfixToken::Operand(4),PostfixToken::Operand(5),PostfixToken::Operator(Operator::Mul),PostfixToken::Operator(Operator::Mul),PostfixToken::Operator(Operator::Add)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::Operand(5),InfixToken::RightParen]));
    }


}
