use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use super::perceptron::Perceptron;

struct Treinamento {
    amostras: Vec<Perceptron>  
}

fn mostrar_pesos(pesos: &Vec<f64>, i: usize) {
    println!("Passo {i}");
    println!("w1, w2, w3, w4, w5, w6, w7, w8");
    let texto_pesos = pesos
        .iter()
        .map(|peso| format!("{:.4}", peso))
        .collect::<Vec<String>>()
        .join(", ");
    println!("{}\n", texto_pesos);
}

impl Treinamento {
    pub fn new(amostras: Vec<Perceptron>) -> Self {
        Self { amostras }
    }

    pub fn calcular_novos_pesos(&self, pesos: &[f64]) -> Vec<f64> {
        let num_amostras = self.amostras.len() as f64;
        let num_pesos = self.amostras
            .get(0)
            .expect("O número mínimo de amostras é pelo menos um")
            .len();
        let mut novos_pesos = vec![0.0; num_pesos ];
        for amostra in &self.amostras {
            let deltas = amostra.calcular_deltas(pesos);
            for (i, delta) in deltas.iter().enumerate() {
                novos_pesos[i] += delta;
            }
        }
        novos_pesos
            .iter_mut()
            .enumerate()
            .for_each(|(i, novo_peso)| {
                *novo_peso = pesos[i] + (*novo_peso / num_amostras)
            });
        novos_pesos
    }
}

fn criar_amostras_csv(caminho_csv: &str) -> io::Result<Vec<Perceptron>> {
    let caminho = Path::new(caminho_csv);
    let arquivo = File::open(caminho)?;
    let leitor = io::BufReader::new(arquivo);

    let mut amostras = Vec::new();

    for linha in leitor.lines() {
        let linha = linha?;
        let partes: Vec<&str> = linha
            .split(|caractere| caractere == ',' || caractere == ';')
            .collect();

        // pular linhas inválidas
        if partes.len() < 9 {
            continue;
        }

        let entradas: Vec<f64> = partes[0..8]
            .iter()
            .map(|&x| x.trim().parse().unwrap_or(0.0))
            .collect();
        let esperado: i32 = partes[8].trim().parse().unwrap_or(-1);
        amostras.push(Perceptron::new(entradas, esperado));
    }
    Ok(amostras)
}

const NUM_DIGITOS: usize = 8;
const MAX_PASSOS: usize = 5000;

pub fn treinar(caminho_csv: &str) {
    let mut pesos = vec![1.0; NUM_DIGITOS ];
    let amostras = criar_amostras_csv(caminho_csv).expect("Erro ao ler o arquivo");
    let treinamento = Treinamento::new(amostras);
    for decada in 0 .. MAX_PASSOS {
        let mostrar_saida = decada == (MAX_PASSOS - 1);
        if mostrar_saida {
            println!("Total de amostras {}", treinamento.amostras.len());
            mostrar_pesos(&pesos, decada);
            println!("x1, x2, x3, x4, x5, x6, x7, x8, saída (H), f1, o1, erro, dx1, dx2, dx3, dx4, dx5, dx6, dx7, dx8");
        }
        treinamento.amostras
            .iter()
            .for_each(|amostra| {
                if mostrar_saida {
                    println!("{}", amostra.exportar_linha_csv(&pesos));
                }
            });
        if mostrar_saida {
            let erro_global = treinamento.amostras
                .iter()
                .map(|amostra| amostra.calcular_erro(&pesos))
                .fold(0.0, |soma, erro| soma + (erro * erro));
            println!("Erro global: {:.2}", erro_global);
        }
        pesos = treinamento.calcular_novos_pesos(&pesos);
    }
}
