#[allow(unused)]


pub mod calc {
    use std::{io::{self, Read}, sync::Arc};
    
    
    enum Op {
        Adic,
        Sub,
        Mult,
        Div,
        Error
    }

    pub fn menu() {
        println!("Qual a operação desejada?");
        let mut op = input();

        let op_conv = conversor(op);
    }

    fn input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input error");
        input
    }

    fn conversor(mut text: String) -> i32 {
        io::stdin().read_line(&mut text).expect("converter error");
        let num_format: i32 = text.trim().parse().unwrap();
        num_format
    }

    fn op_selec(op: i32) -> Op{
        match op {
            1 => Op::Adic,
            2 => Op::Sub,
            3 => Op::Mult,
            4 => Op::Div,
            _ => Op::Error
        }
    }

    fn oppp(o: Op) {
        match o {
            Op::Adic => adic(),
            Op::Sub => sub(),
            Op::Mult => mult(),
            Op::Div => div(),
            Op::Error => error()
        }
    }

    fn recebe_dois_numeros() {

    }

    fn adic() {
    }
    fn sub() {
        
    }
    fn mult() {
        
    }
    fn div() {

    }
    fn error() {
        println!("Como caralhos rolou um erro???")
    }
}
