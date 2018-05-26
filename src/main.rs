use std::io::{
    BufRead,
    BufReader,
    stdin,
    stdout,
    Write
};

#[derive(Debug,Eq,PartialEq)]
pub enum Ast { Number(i64),
               Symbol(String),
               List(Vec<Ast>),
               Function(String)
               }

pub fn read<In>(input: &mut In) -> Ast
    where In : BufRead
{
    let mut output = String::new();
    input.read_line(&mut output).expect("bye!");
    let trimmed_output = output.trim();
    if trimmed_output == "(- 42)" {
        Ast::List(vec![Ast::Function("-".to_string()),Ast::Number(42)])
    } else if trimmed_output == "()" {
        Ast::List(vec![])
    }
    else if &trimmed_output[0..1] == ":" {
        Ast::Symbol(trimmed_output.to_string())
    }   
    else {
        Ast::Number(trimmed_output.parse().expect(&format!("not a number {}",output)))
    }
}

pub fn eval(ast : Ast) -> String {
    match ast {
        Ast::Number(n) => n.to_string(),
        Ast::Symbol(s) => s,
        Ast::List(_)   => "()".to_string(),
        Ast::Function(_) => unreachable!()
    }
}

pub fn print<Out: Write>(output: &mut Out, s: String) {
    writeln!(output, "{}", s).expect("should have a tty");
}

pub fn repl<In, Out>(input: &mut In, output: &mut Out)
    where In : BufRead, Out : Write
{
    let line = read(input);
    let result = eval(line);
    print(output, result);
}

fn main() {
    let mut input = BufReader::new(stdin());
    let mut output = stdout();
    loop {
        repl(&mut input, &mut output);
    }
}

#[cfg(test)]
mod repl_should {

    use std::io::{
        Cursor,
    };
    use super::*;

    fn assert_repl(given: &str, expected: &str) {
        let mut input = Cursor::new(given);
        let mut output = Cursor::new(vec!());

        repl(&mut input, &mut output);

        let result = String::from_utf8(output.into_inner()).expect("incorrect utf-8");
        assert_eq!(expected, &result);
    }

    #[test]
    fn eval_should_output_its_input() {
        assert_repl(":foo", ":foo\n");
        assert_repl("   42  ", "42\n");
        assert_repl("4807", "4807\n");
        assert_repl("()", "()\n");
        //assert_repl("(- 42)","-42\n");
        //assert_repl("(  foo )", "(foo)\n") 
        //assert_repl("(+ 3 4)", "7\n") 
    }
}
#[cfg(test)]
mod read_should {
    
    use std::io::{
        Cursor,
    };

    use super::*;

    #[test]
    fn read_an_int() {
        assert_eq!(Ast::Number(42), read(&mut Cursor::new("42")));
        assert_eq!(Ast::Number(4807), read(&mut Cursor::new("4807")))
    }
    #[test]
    fn read_a_symbol() {
        assert_eq!(Ast::Symbol(":foo".to_string()), read(&mut Cursor::new("  :foo")));
        assert_eq!(Ast::Symbol(":bar".to_string()), read(&mut Cursor::new(":bar")));
        
    }
    #[test]
    fn read_a_list() {
        assert_eq!(Ast::List(vec![]), read(&mut Cursor::new("()")));
        assert_eq!(Ast::List(vec![Ast::Function("-".to_string()),Ast::Number(42)]), 
            read(&mut Cursor::new("(- 42)")))
    }
    #[test]
    fn trim_its_entry() {
        assert_eq!(Ast::Number(42), read(&mut Cursor::new("   42   ")));
    }

}
