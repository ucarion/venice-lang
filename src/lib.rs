#![feature(plugin)]
 #![plugin(peg_syntax_ext)]

mod ast;
mod interpreter;

peg_file! grammar("grammar.rustpeg");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::grammar;
        use interpreter::Interpreter;

        use std::fs::File;
        use std::io::Read;

        let mut f = File::open("example.ve").unwrap();
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();

        let ast = grammar::program(&input).unwrap();
        let interpreter = Interpreter::new(ast);
        interpreter.execute();

        panic!();
    }
}
