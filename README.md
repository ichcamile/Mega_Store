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
│ produtos         HashMap<ID, Produto>       │
│ arestas          HashMap<ID, Vec<ID>>       │
│ indice_nome      HashMap<String, HashSet<ID>> │
│ indice_marca     HashMap<String, HashSet<ID>> │
│ indice_categoria HashMap<String, HashSet<ID>> │
└──────────────────────────────┘
