use crate::units::Unit;
use std::time::{Instant};
use decimal::d128;

#[derive(Clone, Debug)]
pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
  Modulo,
  Caret,
  LeftParen, // lexer only
  RightParen, // lexer only
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
  Percent,
  Factorial,
}

#[derive(Clone, Debug)]
pub enum TextOperator {
  To,
  Of,
}

#[derive(Clone, Debug)]
pub enum Constant {
  Pi,
  E,
}

#[derive(Clone, Debug)]
pub enum FunctionIdentifier {
  Sqrt,
  Cbrt,

  Log,
  Ln,
  Exp,
  
  Round,
  Ceil,
  Floor,
  Abs,

  Sin,
  Cos,
  Tan,
}

#[derive(Clone, Debug)]
pub enum LexerKeyword {
  Per,
  PercentChar,
  In,
  DoubleQuotes,
  Mercury,
  Hg,
  PoundForce,
  PoundWord,
  Force,
}

mod units;

#[derive(Clone, Debug)]
pub enum Token {
  Operator(Operator),
  UnaryOperator(UnaryOperator),
  Number(d128),
  FunctionIdentifier(FunctionIdentifier),
  Constant(Constant),
  Paren, // parser only
  Per, // lexer only
  LexerKeyword(LexerKeyword),
  TextOperator(TextOperator),
  Negative, // parser only
  Unit(units::Unit),
}

pub type TokenVector = Vec<Token>;

mod lexer;
mod parser;
mod evaluator;
mod lookup;

fn main() {
  use std::env;
  let args: Vec<String> = env::args().collect();
  if args.len() >= 2 {
    eval(&args[1], true, Unit::Celcius);
  } else {
    println!("No argument supplied");
  }
}

pub fn eval(input: &str, allow_trailing_operators: bool, default_degree: Unit) {

  let lex_start = Instant::now();

  match lexer::lex(input, allow_trailing_operators, default_degree) {
    Ok(tokens) => {
      let lex_time = Instant::now().duration_since(lex_start).as_nanos() as f32;

      let parse_start = Instant::now();
      match parser::parse(&tokens) {
        Ok(ast) => {
          let parse_time = Instant::now().duration_since(parse_start).as_nanos() as f32;

          let eval_start = Instant::now();
          match evaluator::evaluate(&ast) {
            Ok(answer) => {
              let eval_time = Instant::now().duration_since(eval_start).as_nanos() as f32;

              println!("Lexed TokenVector: {:?}", tokens);
              println!("Parsed AstNode: {:#?}", ast);
              
              println!("Evaluated value: {} {:?}", answer.value, answer.unit);
              
              println!("\u{23f1}  {:.3}ms lexing", lex_time/1000.0/1000.0);
              println!("\u{23f1}  {:.3}ms parsing", parse_time/1000.0/1000.0);
              println!("\u{23f1}  {:.3}ms evaluation", eval_time/1000.0/1000.0);
            },
            Err(e) => println!("Eval error: {}", e),
          }
          
        },
        Err(e) => println!("Parsing error: {}", e),
      }

    },
    Err(e) => println!("Lexing error: {}", e),
  }
  
}
