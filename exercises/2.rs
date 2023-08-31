fn main() {
    let cedulas = vec![200, 50, 20, 5, 2];

    imprime_cedulas_disponiveis(&cedulas);

    let mut total = String::new();
    std::io::stdin().read_line(&mut total).unwrap();

    let total: i32 = total.trim().parse().unwrap();

    let resto = realiza_saque(total, &cedulas);

    if resto > 0 {
        println!("Faltou R$ {resto}!");
    }

}

fn imprime_cedulas_disponiveis(cedulas: &[i32]) {
    println!("Cédulas disponíveis neste caixa:");
    for cedula in cedulas {
        print!("R$ {cedula} ")
    }

    println!();
}

fn realiza_saque(mut total: i32, cedulas: &[i32]) -> i32 {
    for cedula in cedulas {
        let qtd_cedula = total / cedula;

        if qtd_cedula == 0 { continue; }

        println!("{qtd_cedula} x R$ {cedula}");

        total -= qtd_cedula * cedula;

        if total <= 0 { break; }
    }

    total
}