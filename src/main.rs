use std::{io, process::exit};
use rand::prelude::*;
fn main() {
    tic_tac_toe_game();
}

fn tic_tac_toe_game() {
    let mut vetor_casas: Vec<Vec<String>> = vec![
        vec![" ".into(), " ".into(), " ".into()],
        vec![" ".into(), " ".into(), " ".into()],
        vec![" ".into(), " ".into(), " ".into()],
    ];

    loop {

        println!("
            0       1       2
        0   {}   |   {}   |   {}
          ------------------------
        1   {}   |   {}   |   {}
          ------------------------
        2   {}   |   {}   |   {}

            ", vetor_casas[0][0], vetor_casas[1][0], vetor_casas[2][0],
               vetor_casas[0][1], vetor_casas[1][1], vetor_casas[2][1],
               vetor_casas[0][2], vetor_casas[1][2], vetor_casas[2][2]);

        let win_computador = winner(&vetor_casas, "o");

        if !win_computador {
                let player = player();
                vetor_casas[player.1][player.0] = "x".into();
        }

        else {
            println!("Computador venceu");
            exit(0);
        }

        let win_player = winner(&vetor_casas, "x");

        if !win_player {
            let computador = computer();
            vetor_casas[computador.1][computador.0] = "o".into();
        }

        else {
            println!("Player venceu");
            exit(0);
        }
    }
}


fn player() -> (usize, usize) {
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

    (line, col)
}

fn computer() -> (usize, usize) {
    let mut line: Vec<usize> = (0..=2).collect();
    let mut col: Vec<usize> = (0..=2).collect();

    let mut rng = rand::rng();

    line.shuffle(&mut rng);
    let line = line.choose(&mut rng);

    let mut rng = rand::rng();

    col.shuffle(&mut rng);
    let col = col.choose(&mut rng);

    (*line.unwrap(), *col.unwrap())
}

fn winner(v: &Vec<Vec<String>>, l: &str) -> bool {
    if v[0][0] == l && v[1][0] == l && v[2][0] == l {
        return true;
    }

    else if v[0][1] == l && v[1][1] == l && v[2][1] == l  {
            return true;
        }

    else if  v[0][2] == l && v[1][2] == l && v[2][2] == l {
            return true;
        }
    
    else if v[0][0] == l && v[0][1] == l && v[0][2] == l {
        return true;
    }

    else if v[1][0] == l && v[1][1] == l && v[1][2] == l {
        return true;
    }

    else if v[2][0] == l && v[2][1] == l && v[2][2] == l {
        return true;
    }

    else if v[0][0] == l && v[1][1] == l && v[2][2] == l  {
        return true;
    }

    else if v[0][2] == l && v[1][1] == l && v[2][0] == l  {
        return true;
    }

    else {
        return  false;
    }
}