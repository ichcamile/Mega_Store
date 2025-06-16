use megastore::{GrafoProdutos, Produto};

fn main() {
    let mut grafo = GrafoProdutos::new();

    grafo.adicionar_produto(Produto {
        id: 1,
        nome: "Tênis Air".to_string(),
        marca: "Nike".to_string(),
        categoria: "Calçados".to_string(),
    });

    grafo.adicionar_produto(Produto {
        id: 2,
        nome: "Tênis Cloud".to_string(),
        marca: "Adidas".to_string(),
        categoria: "Calçados".to_string(),
    });

    let resultados = grafo.buscar_por_categoria("Calçados");
    for produto in resultados {
        println!("{:?}", produto);
    }
}
