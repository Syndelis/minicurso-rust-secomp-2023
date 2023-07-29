---
marp: true
paginate: true
class:
    - lead
    - invert
header: UFSJ | Secomp 2023
footer: A linguagem Rust e abstrações de alto nível
style: |
  .columns {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
  }

  .unequal-columns {
    display: grid;
    grid-template-columns: auto auto auto;
    gap: 1rem;
  }

  .column-23 {
    grid-column: 1 / 3;
  }

  .column-13 {
    grid-column: 3 / 3;
  }

  .column-34 {
    grid-column: 1 / 4;
  }

  .column-14 {
    grid-column: 4 / 4;
  }
---

<!-- _header: '' -->
<!-- _footer: '' -->
<!-- _paginate: false -->

# A linguagem Rust e abstrações de alto nível

## SECOMP 2023
### Brenno Lemos

- [![width:30px](./img/github-logo.png) Syndelis](https://github.com/Syndelis)
- [![width:30px](./img/mastodon-logo.svg) @brenno@fosstodon.org](https://fosstodon.org/@brenno)

![bg right vertical 40%](./img/ufsj.png)
![bg right vertical 50%](./img/secomp-2023.png)

---

# Antes de mais nada

## Instale Rust e participe do *live-coding*

```sh
$ curl https://sh.rustup.sh | sh
```

![bg left 50%](./img/rust-logo-white.png)

---

# Por que Rust?

- Padrão único de organização estrutural;
- Possui um gerenciador de pacotes oficial;
- Impossibilita* condições de corrida e vazamento de memória;
- É o inimigo № 1 do *Segmentation Fault*;
  - Segurança e confiabilidade 🤝


![bg right 50%](./img/does_not_compile.svg)

---

# Exemplo: Gerencimanto de Memória Automático

<div class="columns">
<div>

## C

```c
#include <stdlib.h>
int main() {
    // Alocamos o vetor
    int *vec = (int*) malloc(
        50 * sizeof(int)
    );

    // Usamos o vetor...
    usa_vetor(vec);

    // Liberamos a memória
    free(vec);
}
```

</div>

<div>

## Rust

```rs
fn main() {
    // Alocamos o vetor
    let vec: Vec<i32> = Vec::new();

    // Usamos o vetor...
    usa_vetor(&vec);

    // A memória é liberada
    // automaticamente
}
```

</div>

---

# Índice - O que vamos aprender

1. A Sintaxe de Rust;
    - Comparando com C e Python;
2. Sistema de posse e empréstimo
    (*ownership & borrowing system*);
3. Estruturas e traços
    (*structs & traits*);
4. Implementação "cobertor"
    (*blanket trait implementation*);

![bg right 70%](./img/happy-3d-ferris.png)

---

# 1. Um Resumo da Sintaxe

<div class="columns">
<div>

- Similar ao C;
- Parênteses são opcionais e desencorajados;
- `for` genérico ao invés de numérico;
- `return` opcional na maioria dos casos;
- Tipagem pós-fixada ao invés de prefixada;
- Macros explícitos com `!`;

</div>

<div style="font-size: 2em">

```rust
fn cinco_ou_maior(x: i32) -> i32 {
  if x > 5 { x } else { 5 }
}
```

```rust
fn main() {
  for i in 0..10 {
    println!(
      "Valor: {}",
      cinco_ou_maior(i)
    );
  }
}
```

</div>
</div>

---

<!-- _header: '' -->
<!-- _footer: '' -->

# 1.1. Declaração de variáveis

<div class="columns">
<div>

- Declaradas com `let`;
- Apesar do nome, não são sempre "variáveis";
  - Por padrão, são **imutáveis**;
- Opcionalmente **mutáveis** com `mut`;
- Podem ser "redefinidas", criando uma nova variável com o mesmo identificador;
  - Dizemos que a variável foi "sombreada" (*shadowed*);
- Tipos podem ser omitidos se *inferíveis*;

</div>

<div>

<div>

## Inválido —

</div>

<div style="font-size: 1.5em">

```rust
let x = 10;
x = 20; // Erro!
x += 1; // Erro!
```

</div>

## Válido —

<div class="columns" style="font-size: 2em">

```rust
let mut x = 10;
x = 20;
x += 1;
```

```rust
let x = 10;
let x = 20;
let x = x + 1;
```

</div>

</div>

</div>
</div>

---

# 2. Posse vs. Empréstimo

- Um dos aspectos mais complicados para iniciantes na linguagem;
- É a "magia" por trás da segurança de Rust;

<div style="font-size: 1.55em">

```rust
let x = vec![1, 2, 3]; // Dono do dado
let y = x; // Passagem de posse

let a = &x[0]; // Erro! `x` não é mais dona do dado!
```

```rust
let x = vec![1, 2, 3];
let y = &x; // Empréstimo

let a = &x[0]; // OK
```

</div>

---

<!-- _header: '' -->
<!-- _footer: '' -->

# 2.1. A Importância do Gerenciamento de Memória Automático

Você consegue dizer qual linha causará um erro?

<div class="unequal-columns">

<div class="column-34" style="font-size: 3rem">

```c
#include <stdio.h>
#include <stdlib.h>

int main() {
  int *a = (int*)malloc(sizeof(int) * 10);
  int *b = a;

  free(a);

  printf("%d\n", a[5]);
  printf("%d\n", b[9]);

  free(b);
}
```

</div>

<div class="column-14">

## Responda <br> aqui

![width:240px](./img/qr-question-c-segfault.svg)

</div>
</div>


---

# 3. Estruturas e Traços

- Estruturas nos permitem agrupar e armazenar dados de maneira arbitrária;

```rust
struct Cpf([u8; 11]);

struct Pessoa {
  nome: String,
  cpf: Cpf,
}
```