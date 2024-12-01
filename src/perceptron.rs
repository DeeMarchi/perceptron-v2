pub struct Perceptron {
    entradas: Vec<f64>,
    esperado: i32,
}

// const TAXA_APRENDIZAGEM: f64 = 0.1;
// const BIAS: f64 = -37.0;
// const TAXA_APRENDIZAGEM: f64 = 0.032152;
const TAXA_APRENDIZAGEM: f64 = 0.020010;
const BIAS: f64 = -37.0;

impl Perceptron {
    pub fn new(entradas: Vec<f64>, esperado: i32) -> Self {
        Self { entradas, esperado }
    }

    pub fn len(&self) -> usize {
        self.entradas.len()
    }

    fn step(&self, net: f64) -> f64 {
        if net >= 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    pub fn calcular_deltas(&self, pesos: &[f64]) -> Vec<f64> {
        let saida = self.calcular_ativacao(pesos);
        self.entradas
            .iter()
            .map(|x| {
                TAXA_APRENDIZAGEM * (self.esperado as f64 - saida) * x
            })
            .collect()
    }

    pub fn calcular_saida(&self, pesos: &[f64]) -> f64 {
        self.entradas
            .iter()
            .zip(pesos)
            .fold(0.0, |soma, (a, b)| soma + a * b) + BIAS
    }

    pub fn calcular_ativacao(&self, pesos: &[f64]) -> f64 {
        let net = self.calcular_saida(pesos);
        self.step(net)
    }

    pub fn entradas_to_string(&self) -> String {
        self.entradas
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn get_esperado(&self) -> i32 {
        self.esperado
    }

    pub fn calcular_erro(&self, pesos: &[f64]) -> f64 {
        self.esperado as f64 - self.calcular_ativacao(pesos)
    }

    pub fn exportar_linha_csv(&self, pesos: &[f64]) -> String {
        let texto_deltas = self
            .calcular_deltas(pesos)
            .iter()
            .map(|&delta| format!("{:.2}", delta))
            .collect::<Vec<String>>()
            .join(", ");
        let texto_entradas = self.entradas_to_string();
        let saida = self.calcular_saida(pesos);
        let ativacao = self.calcular_ativacao(pesos);
        let esperado = self.get_esperado();
        let erro = self.calcular_erro(pesos);
        format!("{}, {:.4}, {}, {}, {:.2}, {}", texto_entradas, saida, ativacao, esperado, erro, texto_deltas)
    }
}
