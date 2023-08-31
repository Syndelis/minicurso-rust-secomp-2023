fn main() {
    let cedulas = vec![200, 100, 50, 20, 10, 5, 2];

    let mut total = String::new();
    std::io::stdin().read_line(&mut total).unwrap();

    let mut total: i32 = total.trim().parse().unwrap();

    for cedula in cedulas {
        let qtd_cedula = total / cedula;

        if qtd_cedula == 0 { continue; }
    
        println!("{qtd_cedula} x R$ {cedula}");

        total -= qtd_cedula * cedula;

        if total <= 0 { break; }
    }
}
