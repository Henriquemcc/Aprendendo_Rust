use rand::Rng;
mod myio;

///Esta funcao eh a funcao principal, que eh a primeira funcao chamada quando o programa inicia.
fn main()
{
    //Exibindo mensagem de bem vindo
    println!("Bem vindo ao jogo da adivinhação");

    //Obtendo a dificuldade
    let dificuldade = obter_dificuldade();

    //Configurando a quantidade total de tentativas de acordo com a dificuldade escolhida
    let quantidade_total_de_tentativas = obter_quantidade_de_tentativas(dificuldade);

    //Gerando um numero secreto
    let numero_secreto:u8 = rand::thread_rng().gen_range(1, 101);

    //Declarando pontos
    let mut pontos: i16 = 1000;

    //Rodando a rodada
    for rodada in 1..quantidade_total_de_tentativas
    {
        //Imprimindo o numero da tentativa
        println!("Tentativa {} de {}", rodada, quantidade_total_de_tentativas);

        //Obtendo o chute do usuario
        let chute = obter_chute();

        //Verificando se o chute eh igual ao numero secreto
        if chute==numero_secreto
        {
            println!("Você acertou e fez {} pontos", pontos);
            break;
        }
        else
        {
            //Verificando se o chute eh maior que o numero secreto
            if chute > numero_secreto
            {
                println!("O seu chute foi maior que o número secreto");
            }

            //Verificando se o chute eh menor que o numero secreto
            else if chute < numero_secreto
            {
                println!("O seu chute foi menor que o numero secreto");
            }

            //Calculando a quantidade de pontos que foram perdidos
            let pontos_perdidos = (i16::from(chute)-i16::from(numero_secreto)).abs();
            pontos -= pontos_perdidos;
        }

    }

    //Imprimindo o numero secreto e a pontuacao
    println!("O número secreto era {}. Você fez {} pontos", numero_secreto, pontos);


}

///Esta funcao serve para obter do usuario a dificuldade que ele deseja jogar.
///Retorno: u8: Numero inteiro com o nivel de dificuldade.
fn obter_dificuldade() -> u8
{
    //Obtendo a dificuldade
    let mut dificuldade: u8;
    loop
    {
        //Exibindo as dificuldades
        println!("Selecione a dificuldade:");
        println!("(1) Fácil");
        println!("(2) Médio");
        println!("(3) Difícil");

        //Obtendo a dificuldade
        dificuldade= myio::read_u8();

        if dificuldade < 1 || dificuldade > 3
        {
            eprintln!("Valor inválido");
        }
        else
        {
            break;
        }
    }
    return dificuldade;
}

///Esta funcao serve para obter a quantidade total de tentativas com base na dificuldade do jogo.
///Parametro: dificuldade: Numero inteiro de 1 a 3 indicando a dificuldade do jogo (1-> facil, 2-> medio, 3->dificil).
///Retorno: u8: Numero inteiro indicando a quantidade total de tentativas que o usuario tem direito.
fn obter_quantidade_de_tentativas(dificuldade: u8) -> u8
{
    let mut quantidade_total_de_tentativas: u8=0;

    if dificuldade==1
    {
        quantidade_total_de_tentativas=20;
    }
    else if dificuldade == 2
    {
        quantidade_total_de_tentativas=10;
    }
    else if dificuldade== 3
    {
        quantidade_total_de_tentativas=5;
    }

    return quantidade_total_de_tentativas;
}

///Esta funcao serve para obter o chute do usuario.
///Retorno: u8: Numero inteiro do chute do usuario.
fn obter_chute() -> u8
{
    let mut chute:u8;
    loop
    {
        println!("Digite um numero entre 1 e 100");
        chute = myio::read_u8();

        //Verificando se o chute eh invalido
        if chute <1 || chute >100
        {
            //Imprimindo mensagem de erro
            eprintln!("O chute deve ser entre 1 e 100");
        }
        else
        {
            //Saindo do loop
            break;
        }
    }

    return chute;
}