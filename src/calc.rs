#[allow(unused)]

pub mod calc {
    use std::io::{self, Read};

    pub fn menu() {
        println!(
            "Qual a operação desejada?
        1 -> adição
        2 -> subtração
        3 -> multiplicação
        4 -> divisão"
        );
        let mut inp = input();

        let op_conv = conversor(inp);
        op_select(op_conv);
    }

    fn input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input error");
        input
    }

    fn conversor(mut text: String) -> i32 {
        let num_format: i32 = text.trim().parse().unwrap();
        num_format
    }

    fn op_select(op: i32) {
        match op {
            1 =>  adic(),
            2 => sub(),
            3 => mult(),
            4 => div(),
            4 => pot(),
            _ => error(),
        }
    }

    fn recebe_dois_numeros() -> (i32, i32) {
        println!("Digite o primeiro número");
        let num1 = conversor(input());
        println!("Digite o segundo número");
        let num2 = conversor(input());

        (num1, num2)
    }

    fn adic() {
        let (num1, num2) = recebe_dois_numeros();
        println!(
            "O resultado da adição entre {} e {} é: {}",
            num1,
            num2,
            num1 + num2
        )
    }
    fn sub() {
        let (num1, num2) = recebe_dois_numeros();
        println!(
            "O resultado da subtração entre {} e {} é: {}",
            num1,
            num2,
            num1 - num2
        )
    }
    fn mult() {
        let (num1, num2) = recebe_dois_numeros();
        println!(
            "O resultado da multiplicação entre {} e {} é: {}",
            num1,
            num2,
            num1 * num2
        )
    }
    fn div() {
        let (num1, num2) = recebe_dois_numeros();
        println!(
            "O resultado da divisão entre {} e {} é: {}",
            num1,
            num2,
            num1 / num2
        )
    }
    fn pot() {
        let (num1, num2) = recebe_dois_numeros();
        println!(
            "O resultado da divisão entre {} e {} é: {}",
            num1,
            num2,
            num1 * num2
        )
        //TODO: implementar potenciação...
    }
    fn error() {
        println!("Como caralhos rolou um erro???")
    }

    pub fn nova_op() {
        loop {
            println!(
                "Deseja realizar uma nova operação?
        1 -> Sim
        2 -> Não"
            );

            let inp = conversor(input());

            if inp == 1 {
                menu()
            } else {
                break;
            }
        }
    }
}
