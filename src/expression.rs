use thiserror::Error;


pub struct Expression {
    components: Vec<ExpressionComponent>
}

#[derive(Error, Debug)]
pub enum ExpressionCalculationError {
    #[error("Mismatched parenthesis in the expression.")]
    MissmatchedParenthesis,
    #[error("Calculation unsuccesfull.")]
    RPNCalculation
}

impl Expression {
    pub fn calculate(&self) -> Result<f64, ExpressionCalculationError> {
        let mut rpn_componenets = vec![];
        let mut operator_stack = Vec::<Operator>::new();
        for component in self.components.iter() {
            match component {
                ExpressionComponent::Number(_) => rpn_componenets.push(component.clone()),
                ExpressionComponent::Operator(operator) => {
                    match operator {
                        Operator::ArithmeticOperator(arithmetic_operator) => {
                            loop {
                                let Some(last_operator) = operator_stack.last() else {
                                    break;
                                };
                                
                                if *last_operator == Operator::LeftParenthesis {
                                    break;
                                }
                                
                                let Operator::ArithmeticOperator(last_arithmetic_operator) = last_operator else {
                                    return Err(ExpressionCalculationError::MissmatchedParenthesis);
                                };

                                if last_arithmetic_operator.precedence() < arithmetic_operator.precedence() &&
                                    (last_arithmetic_operator.precedence() != arithmetic_operator.precedence() || arithmetic_operator.associativity() != Associativity::Left) {
                                    break;
                                }

                                rpn_componenets.push(ExpressionComponent::Operator(operator_stack.pop().unwrap()));
                            }

                            operator_stack.push(operator.clone());
                        },
                        Operator::RightParenthesis => {
                            loop {
                                let Some(last_operator) = operator_stack.last() else {
                                    return Err(ExpressionCalculationError::MissmatchedParenthesis);
                                };

                                if *last_operator == Operator::LeftParenthesis {
                                    operator_stack.pop().unwrap();
                                    break;
                                }

                                rpn_componenets.push(ExpressionComponent::Operator(operator_stack.pop().unwrap()));
                            }
                        },
                        Operator::LeftParenthesis => operator_stack.push(operator.clone()),
                    }
                }
            }
        }

        while !operator_stack.is_empty() {
            rpn_componenets.push(ExpressionComponent::Operator(operator_stack.pop().unwrap()))
        }

        let mut rpn_stack = vec![];
        for component in rpn_componenets.iter() {
            match component {
                ExpressionComponent::Number(number) => {
                    rpn_stack.push(number.clone());
                },
                ExpressionComponent::Operator(Operator::ArithmeticOperator(operator)) => {
                    let second = rpn_stack.pop().ok_or(ExpressionCalculationError::RPNCalculation)?;
                    let first = rpn_stack.pop().ok_or(ExpressionCalculationError::RPNCalculation)?;
                    rpn_stack.push(operator.calculate(first, second));
                },
                _ => return Err(ExpressionCalculationError::RPNCalculation)
            }
        }

        if rpn_stack.len() > 1 {
            return Err(ExpressionCalculationError::RPNCalculation);
        }

        rpn_stack.pop().ok_or(ExpressionCalculationError::RPNCalculation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ExpressionComponent {
    Number(f64),
    Operator(Operator),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    ArithmeticOperator(ArithmeticOperator),
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Associativity {
    Left,
    Right
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ArithmeticOperator {
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Power,
}

impl ArithmeticOperator {
    fn calculate(&self, a: f64, b: f64) -> f64 {
        match self {
            ArithmeticOperator::Plus => a + b,
            ArithmeticOperator::Minus => a - b,
            ArithmeticOperator::Asterisk => a * b,
            ArithmeticOperator::ForwardSlash => a / b,
            ArithmeticOperator::Power => a.powf(b)
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            ArithmeticOperator::Plus => 2,
            ArithmeticOperator::Minus => 2,
            ArithmeticOperator::Asterisk => 3,
            ArithmeticOperator::ForwardSlash => 3,
            ArithmeticOperator::Power => 4
        }
    }

    fn associativity(&self) -> Associativity {
        match self {
            ArithmeticOperator::Plus => Associativity::Left,
            ArithmeticOperator::Minus => Associativity::Left,
            ArithmeticOperator::Asterisk => Associativity::Left,
            ArithmeticOperator::ForwardSlash => Associativity::Left,
            ArithmeticOperator::Power => Associativity::Right,
        }
    } 
}

#[derive(Error, Debug)]
pub enum ExpressionConversionError {
    #[error("Illegal arguments found in the expression.")]
    IllegalArguments
}

impl TryFrom<String> for Expression {
    type Error = ExpressionConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut components = vec![];
        for component in value.split(' ') {
            components.push(component.trim().try_into()?);
        }

        Ok(Expression { components })
    }
}

impl TryFrom<&str> for ExpressionComponent {
    type Error = ExpressionConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(ExpressionComponent::Operator(Operator::ArithmeticOperator(ArithmeticOperator::Plus))),
            "-" => Ok(ExpressionComponent::Operator(Operator::ArithmeticOperator(ArithmeticOperator::Minus))),
            "*" => Ok(ExpressionComponent::Operator(Operator::ArithmeticOperator(ArithmeticOperator::Asterisk))),
            "/" => Ok(ExpressionComponent::Operator(Operator::ArithmeticOperator(ArithmeticOperator::ForwardSlash))),
            "^" => Ok(ExpressionComponent::Operator(Operator::ArithmeticOperator(ArithmeticOperator::Power))),
            "(" => Ok(ExpressionComponent::Operator(Operator::LeftParenthesis)),
            ")" => Ok(ExpressionComponent::Operator(Operator::RightParenthesis)),
            _ => {
                if let Ok(number) = value.parse::<f64>() {
                    return Ok(ExpressionComponent::Number(number));
                }
                Err(ExpressionConversionError::IllegalArguments)
            }
        }
    }
}