// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021

use std::io;

fn lersalario()-> f64{
    let mut input =String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Not a number")
}
fn ler()-> i32{
    let mut input =String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Not a number")
}

fn main() {

    println!("digite o seu salario do mes");
    let mut salario =lersalario();
    println!("digite o numero de vendas");
    let vendas = ler();

    if vendas > 0 && vendas <= 500 {
      println!("esse é o seu salario bonificado {:.2} " , salario + salario * 0.05)
    }else if vendas >= 501 && vendas < 1000 {
        println!("esse é o seu salario bonificado {:.2} " ,salario + salario * 0.1)
    }else if vendas >= 1000{
        println!("esse é o seu salario bonificado {:.2} " ,salario + salario * 0.2)
    }

}
