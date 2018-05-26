use std::io::{
    BufRead,
    BufReader,
    stdin,
    stdout,
    Write
};

pub fn read<In>(input: &mut In) -> String
    where In : BufRead
{
    let mut output = String::new();
    input.read_line(&mut output).expect("bye!");
    output
}

pub fn eval(s: String) -> String {
    s.trim().to_string()
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
        assert_repl("foo", "foo\n");
        assert_repl("   42  ", "42\n");
        //assert_repl("(  foo )", "(foo)\n") 
    }
}
