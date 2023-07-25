---
marp: true
paginate: true
class:
    - lead
    - invert
header: UFSJ | Secomp 2023
footer: A linguagem Rust e abstrações de alto nível
---

<!-- _backgroundColor: #11111111 -->
<!-- _color: #DEDEDE -->
<!-- _header: '' -->
<!-- _footer: '' -->
<!-- _paginate: false -->

# A linguagem Rust e abstrações de alto nível

## SECOMP 2023
### Brenno Lemos

- [![width:30px](./img/github-logo.png) Syndelis](https://github.com/Syndelis)
- [![width:30px](./img/mastodon-logo.svg) @brenno](https://fosstodon.org/@brenno)

![bg right vertical 40%](./img/ufsj.png)
![bg right vertical 50%](./img/secomp-2023.png)

---

# Antes de mais nada

## Instale Rust e participe do *live-coding*

```sh
$ curl https://sh.rustup.sh | sh
```

---

# Índice - O que vamos aprender

1. A sintaxe de Rust;
    - Comparando com C e Python;
2. Sistema de posse e empréstimo
    (*ownership & borrowing system*);
3. Estruturas e traços
    (*structs & traits*);
4. Implementação "cobertor"
    (*blanket trait implementation*);

![bg right 50%](./img/does_not_compile.svg)