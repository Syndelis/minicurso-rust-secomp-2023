fn cinco_ou_maior(x: i32) -> i32 {
    if x > 5 { x } else { 5 }
}

fn main() {
    for i in 0..10 {
        println!(
            "{}",
            cinco_ou_maior(i)
        );
    }
}
