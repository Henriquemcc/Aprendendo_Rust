use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main()
{
    println!("Adivinha o número!");

    let numero_secreto=rand::thread_rng().gen_range(1, 101);

    loop
    {
        println!("Por favor, digite a sua suposição.");

        let mut suposicao=String::new();

        io::stdin().read_line(&mut suposicao).expect("Falha ao ler a linha");

        let suposicao: u32 = match suposicao.trim().parse()
        {
            Ok(num)=> num,
            Err(_)=> continue,
        };

        match suposicao.cmp(&numero_secreto)
        {
            Ordering::Less => println!("O número digitado é menor."),
            Ordering::Equal=>
            {
                println!("Você ganhou!");
                break;
            },
            Ordering::Greater=> println!("O número digitado é maior.")
        }
    }
}