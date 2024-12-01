use std::path::PathBuf;

mod perceptron;
mod treinamento;

fn main() {
    let params_programa: Vec<_> = std::env::args().collect();
    let num_params = params_programa.len();
    let caminho_csv: PathBuf = [ "caminho", "para", "sua", "amostras.csv" ].iter().collect();
    let nome_programa = params_programa.first().unwrap();
    if num_params != 2 {
        println!("Uso: {} {}", nome_programa, caminho_csv.display());
        panic!("Número incorreto de argumentos");
    }
    let caminho_csv = params_programa.get(1).unwrap();
    treinamento::treinar(caminho_csv);
}
