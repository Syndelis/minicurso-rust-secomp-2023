use std::fmt::{Display, Formatter, Result};

fn main() {
    let p = Pessoa {
        nome: "Brenno".into(),
        cpf: Cpf([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1]),
    };

    println!("{}: {}", p.cpf, p.nome);
}

struct Pessoa {
    nome: String,
    cpf: Cpf,
}

struct Cpf([u8; 11]);

impl Display for Cpf {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut first = true;

        for chunk in self.0.chunks(3) {
            if !first {
                if chunk.len() == 3 {
                    write!(f, ".")?;
                }
                else {
                    write!(f, "-")?;
                }
            }

            first = false;

            for part in chunk {
                write!(f, "{}", part)?;
            }
        }

        Ok(())
    }
}
