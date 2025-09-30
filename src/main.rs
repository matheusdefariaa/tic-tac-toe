use std::io::{self, Read};

fn main() {
    print!("
       0       1       2
  0        |        |
     ----------------------
  1        |        |
     ----------------------
  2        |        |

    \n");

    tic_tac_toe_game();
}

fn tic_tac_toe_game() {
    let mut vetor_casas = vec![
        vec![" ", " ", " "],
        vec![" ", " ", " "],
        vec![" ", " ", " "],
    ];

    loop {

        println!("
            0       1       2
        0     {} |   {}   |  {}
            --------------------
        1     {} |   {}   |  {}
            --------------------
        2     {} |   {}   |  {}

            ", vetor_casas[0][0], vetor_casas[1][0], vetor_casas[2][0],
               vetor_casas[0][1], vetor_casas[1][1], vetor_casas[2][1],
               vetor_casas[0][2], vetor_casas[1][2], vetor_casas[2][2]);

    let mut line = String::new();
    let mut col = String::new();

    println!("Digite a linha: ");
    io::stdin()
    .read_line(&mut line)
    .unwrap();


    println!("Digite a coluna: ");
    io::stdin()
    .read_line(&mut col)
    .unwrap();

    let line = line.trim().parse::<usize>().unwrap();
    let col = col.trim().parse::<usize>().unwrap();

    vetor_casas[col][line] = "x";

    }
}