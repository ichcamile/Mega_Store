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
        grafo.adicionar_produto(Produto {
        id: 3,
        nome: "Smartphone Galaxy S21".to_string(),
        marca: "Samsung".to_string(),
        categoria: "Eletrônicos".to_string(),
    });

    grafo.adicionar_produto(Produto {
        id: 4,
        nome: "Notebook Ultra Slim".to_string(),
        marca: "Lenovo".to_string(),
        categoria: "Eletrônicos".to_string(),
    });

    grafo.adicionar_produto(Produto {
        id: 5,
        nome: "Camiseta Básica".to_string(),
        marca: "Hering".to_string(),
        categoria: "Vestuário".to_string(),
    });

    grafo.adicionar_produto(Produto {
        id: 6,
        nome: "Jaqueta Jeans".to_string(),
        marca: "Levi's".to_string(),
        categoria: "Vestuário".to_string(),
    });

    grafo.adicionar_produto(Produto {
        id: 7,
        nome: "Sofá Retrátil 3 Lugares".to_string(),
        marca: "Mobly".to_string(),
        categoria: "Decoração".to_string(),
    });

    grafo.adicionar_produto(Produto {
        id: 8,
        nome: "Abajur de Mesa".to_string(),
        marca: "Tok&Stok".to_string(),
        categoria: "Decoração".to_string(),
    });

    grafo.adicionar_produto(Produto {
        id: 9,
        nome: "Café em Grãos Especial".to_string(),
        marca: "Santa Mônica".to_string(),
        categoria: "Alimentos".to_string(),
    });

    grafo.adicionar_produto(Produto {
        id: 10,
        nome: "Chocolate Meio Amargo 70%".to_string(),
        marca: "Lindt".to_string(),
        categoria: "Alimentos".to_string(),
    });


    let resultados = grafo.buscar_por_categoria("Calçados");
    for produto in resultados {
        println!("{:?}", produto);
    }
}
