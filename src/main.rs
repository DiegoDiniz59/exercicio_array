// Código do Montanha 

use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;

fn main() {
    let tamanho = 12;
    let mut soma = 0;
    let mut contador = 0;
    let mut o = String::new();
    let path = Path::new("./data/matriz.json");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let matriz: Vec<Vec<i32>> = serde_json::from_reader(reader).unwrap();

    println!("Matriz: {:?}", matriz);

    println!("Digite a operação desejada: ");
    
    print!("Soma S ou Média M => ");
    stdin().read_line(&mut o).unwrap();

    let o = o.trim();

    // Lê as colunas e linhas da área verde da matriz
    for i in 1..=10 {
        soma += matriz[i][11];
        contador += 1;
    }
    for i in 2..=9 {
        soma += matriz[i][10];
        contador += 1;
    }
    for i in 3..=8 {
        soma += matriz[i][9];
        contador += 1;
    }
    for i in 4..=7 {
        soma += matriz[i][8];
        contador += 1;
    }
    for i in 5..=6 {
        soma += matriz[i][7];
        contador += 1;
    }

        // Constrói a representação da matriz, enumera os pontos verdes com "X" e os demais com "."
        let mut representacao_matriz = Vec::new();
        for i in 0..tamanho {
            let mut linha = String::new();
            for j in 0..tamanho {
                if (j == 11 && i >= 1 && i <= 10) ||
                    (j == 10 && i >= 2 && i <= 9) ||
                    (j == 9 && i >= 3 && i <= 8) ||
                    (j == 8 && i >= 4 && i <= 7) ||
                    (j == 7 && (i == 5 || i == 6)) {
                    linha.push_str(" X ");
                } else {
                    linha.push_str(" . ");
                }
            }
            representacao_matriz.push(linha);
        }
    
        println!("Representação da matriz:");
        for linha in representacao_matriz {
            println!("{}", linha);
        }

    // Código do Montanha
    if o == "M" {
        println!("Média: {}", soma / contador);
    } else {
        println!("Soma: {}", soma);
    }
}
