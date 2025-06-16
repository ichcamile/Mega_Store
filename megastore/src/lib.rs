use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Produto {
    pub id: usize,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
}

#[derive(Debug)]
pub struct GrafoProdutos {
    produtos: HashMap<usize, Produto>,
    arestas: HashMap<usize, HashSet<usize>>,
    indice_nome: HashMap<String, HashSet<usize>>,
    indice_marca: HashMap<String, HashSet<usize>>,
    indice_categoria: HashMap<String, HashSet<usize>>,
}

impl GrafoProdutos {
    pub fn new() -> Self {
        Self {
            produtos: HashMap::new(),
            arestas: HashMap::new(),
            indice_nome: HashMap::new(),
            indice_marca: HashMap::new(),
            indice_categoria: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        let id = produto.id;
        self.produtos.insert(id, produto.clone());

        self.indice_nome.entry(produto.nome.clone()).or_default().insert(id);
        self.indice_marca.entry(produto.marca.clone()).or_default().insert(id);
        self.indice_categoria.entry(produto.categoria.clone()).or_default().insert(id);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Vec<&Produto> {
        self.indice_nome
            .get(nome)
            .unwrap_or(&HashSet::new())
            .iter()
            .filter_map(|id| self.produtos.get(id))
            .collect()
    }

    pub fn buscar_por_categoria(&self, categoria: &str) -> Vec<&Produto> {
        self.indice_categoria
            .get(categoria)
            .unwrap_or(&HashSet::new())
            .iter()
            .filter_map(|id| self.produtos.get(id))
            .collect()
    }
}
