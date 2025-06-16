# 📦 MegaStore - Sistema de Recomendação com Grafos em Rust

Este projeto é um estudo de caso sobre a implementação de um sistema de busca e recomendação de produtos utilizando **Rust** e **estruturas de grafos**. Ele simula parte da estrutura de dados de uma loja online de grande porte, a **MegaStore**, com foco em **eficiência**, **escalabilidade** e **organização** do catálogo de produtos.

---

## 🧠 Objetivo

Desenvolver uma solução que permita:

- ✅ Indexar o catálogo de forma eficiente por **nome**, **marca** e **categoria**
- 🔍 Realizar **buscas rápidas e precisas**, mesmo com milhões de produtos
- 📈 Garantir **escalabilidade** para lidar com o crescimento do catálogo

---

## 🏗️ Arquitetura do Sistema

A estrutura é composta por:

- **`Produto`**: Nó do grafo que representa um item do catálogo
- **`GrafoProdutos`**: Estrutura principal que armazena os produtos, índices e conexões
- **Índices (`HashMap`)**: Permitem buscas rápidas por nome, marca e categoria

```text
┌──────────────────────────────┐
│        GrafoProdutos         │
├──────────────────────────────┤
│ produtos         HashMap<ID, Produto>         │
│ arestas          HashMap<ID, Vec<ID>>         │
│ indice_nome      HashMap<String, HashSet<ID>> │
│ indice_marca     HashMap<String, HashSet<ID>> │
│ indice_categoria HashMap<String, HashSet<ID>> │
└──────────────────────────────┘
````

---

## 🚀 Exemplo de Uso

```rust
let mut grafo = GrafoProdutos::new();

grafo.adicionar_produto(Produto {
    id: 1,
    nome: "Notebook X".to_string(),
    marca: "TechBrand".to_string(),
    categoria: "Eletrônicos".to_string(),
});

let resultados = grafo.buscar_por_categoria("Eletrônicos");

for produto in resultados {
    println!("Produto encontrado: {:?}", produto);
}
```

---

## 🧪 Testes

Para rodar os testes:

```bash
cargo test
```

> Os testes cobrem inserção e busca por nome e categoria, além da verificação de acessos seguros com `unwrap_or`.

---

## ⚙️ Algoritmos e Estruturas de Dados

* `HashMap`: Armazenamento rápido dos produtos e dos índices
* `HashSet`: Evita duplicatas nos índices
* Métodos utilizados: `insert`, `get`, `unwrap_or`, `clone`, `default`
* Complexidade de busca em índices: **O(1)**

---

## 📈 Considerações de Desempenho

* Busca por chave em `HashMap` é **constante (O(1))**, ideal para grandes volumes
* Evita iteração completa pelo catálogo
* Estrutura pensada para **expansão futura** com filtros compostos ou ranking de relevância

---
## ▶️ Como Rodar

```bash
git clone [https://github.com/seu-usuario/megastore.git](https://github.com/ichcamile/Mega_Store.git)
cd megastore
cargo run --bin main
```

---

## 🧾 Sobre

Projeto desenvolvido como parte do curso de Rust com foco em estruturas de dados e algoritmos, simulando um cenário corporativo realista.
