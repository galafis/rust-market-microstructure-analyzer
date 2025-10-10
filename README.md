# 📊 Market Microstructure Analyzer em Rust

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/github/license/galafis/rust-market-microstructure-analyzer?style=for-the-badge)

![Stars](https://img.shields.io/github/stars/galafis/rust-market-microstructure-analyzer?style=for-the-badge)

**Engine de análise de microestrutura de mercado para order flow e tape reading em tempo real**

[Documentação](https://github.com/galafis/rust-market-microstructure-analyzer/tree/main/docs) •
[Exemplos](https://github.com/galafis/rust-market-microstructure-analyzer/tree/main/examples) •
[Reportar Bug](https://github.com/galafis/rust-market-microstructure-analyzer/issues)

</div>

---

## 📋 Índice

- [Visão Geral](#-visão-geral)
- [Funcionalidades](#-funcionalidades)
- [Arquitetura](#-arquitetura)
- [Tecnologias](#-tecnologias)
- [Instalação](#-instalação)
- [Uso](#-uso)
- [Exemplos](#-exemplos)
- [Conceitos](#-conceitos)
- [Performance](#-performance)
- [Roadmap](#-roadmap)
- [Licença](#-licença)
- [Autor](#-autor)

---

## 🇧🇷 Visão Geral

O **Market Microstructure Analyzer** é uma engine de análise de microestrutura de mercado desenvolvida em Rust para processar e analisar dados de **order book (Level 2)** e **fluxo de ordens (tape reading)** em tempo real.

### O que é Microestrutura de Mercado?

Microestrutura de mercado é o estudo de como as ordens são executadas e como os preços são formados. Analisa:
- **Order Book:** Profundidade de mercado (bids e asks)
- **Tape Reading:** Fluxo de negócios executados
- **Order Flow:** Padrões de comportamento de grandes players

### Por que usar?

- ⚡ **Alta Performance** - Processamento em Rust para latência mínima
- 📊 **Análise Level 2** - Order book completo com múltiplos níveis
- 🎯 **Tape Reading** - Identificação de padrões em tempo real
- 🔍 **Detecção de Padrões** - Iceberg orders, spoofing, absorption
- 📈 **Métricas Avançadas** - Spread, imbalance, volume profile
- 🎨 **Visualizações** - Gráficos de order flow e heatmaps

---

## 🇺🇸 Overview (English)

The **Market Microstructure Analyzer** is a market microstructure analysis engine developed in Rust to process and analyze **order book (Level 2)** and **order flow (tape reading)** data in real-time.

### What is Market Microstructure?

Market microstructure is the study of how orders are executed and how prices are formed. It analyzes:
- **Order Book:** Market depth (bids and asks)
- **Tape Reading:** Flow of executed trades
- **Order Flow:** Behavioral patterns of large players

---

## ✨ Funcionalidades

### Core Features

- 📖 **Order Book Analysis**
  - Análise de profundidade de mercado (Level 2)
  - Cálculo de bid-ask spread
  - Order imbalance detection
  - Volume profile analysis

- 📊 **Tape Reading**
  - Análise de fluxo de negócios executados
  - Identificação de grandes ordens (block trades)
  - Detecção de absorção de liquidez
  - Time & Sales analysis

- 🔍 **Pattern Detection**
  - **Iceberg Orders:** Ordens grandes ocultas
  - **Spoofing:** Ordens falsas para manipulação
  - **Support/Resistance:** Níveis de suporte e resistência dinâmicos
  - **Absorption:** Absorção de liquidez por grandes players

- 📈 **Advanced Metrics**
  - Bid-Ask Spread (absoluto e percentual)
  - Order Book Imbalance Ratio
  - Volume Profile (POC, VAH, VAL)
  - Delta Volume (buying vs selling pressure)
  - Cumulative Volume Delta (CVD)

---

## 🏗️ Arquitetura

![Arquitetura do Market Microstructure Analyzer](docs/architecture.png)

O sistema é composto por 5 módulos principais:

1. **OrderBook Module** - Gerenciamento e análise de order book
2. **Tape Module** - Processamento de tape reading
3. **Metrics Module** - Cálculo de métricas avançadas
4. **Patterns Module** - Detecção de padrões
5. **Visualization Module** - Geração de gráficos e heatmaps

---

## 🛠️ Tecnologias

| Tecnologia | Versão | Uso |
|------------|--------|-----|
| **Rust** | 1.70+ | Linguagem principal |
| **Tokio** | 1.40 | Runtime assíncrono |
| **Rust Decimal** | 1.36 | Precisão financeira |
| **Plotters** | 0.3 | Visualizações |
| **Serde** | 1.0 | Serialização |
| **Anyhow** | 1.0 | Error handling |

---

## 📦 Instalação

### Pré-requisitos

- Rust 1.70 ou superior ([instalar](https://www.rust-lang.org/tools/install))
- Git

### Clonar e Compilar

```bash
# Clone o repositório
git clone https://github.com/galafis/rust-market-microstructure-analyzer.git
cd rust-market-microstructure-analyzer

# Compile em modo release
cargo build --release

# Execute os testes
cargo test
```

---

## 🚀 Uso

### Execução Básica

```bash
# Executar o binário principal
cargo run --release

# Executar exemplo específico
cargo run --release --example orderbook_analysis
```

### Exemplo de Código

```rust
use market_microstructure_analyzer::*;
use rust_decimal_macros::dec;

fn main() -> Result<()> {
    // Criar order book
    let orderbook = OrderBook {
        bids: vec![
            Level { price: dec!(50000.00), quantity: dec!(1.5) },
            Level { price: dec!(49999.50), quantity: dec!(2.3) },
        ],
        asks: vec![
            Level { price: dec!(50001.00), quantity: dec!(1.2) },
            Level { price: dec!(50001.50), quantity: dec!(1.8) },
        ],
        timestamp: 1696435200,
    };

    // Calcular spread
    let best_bid = &orderbook.bids[0].price;
    let best_ask = &orderbook.asks[0].price;
    let spread = best_ask - best_bid;
    
    println!("Spread: ${}", spread);
    
    // Calcular imbalance
    let total_bid_volume: Decimal = orderbook.bids.iter()
        .map(|l| l.quantity)
        .sum();
    let total_ask_volume: Decimal = orderbook.asks.iter()
        .map(|l| l.quantity)
        .sum();
    
    let imbalance = (total_bid_volume - total_ask_volume) 
        / (total_bid_volume + total_ask_volume);
    
    println!("Order Imbalance: {}", imbalance);
    
    Ok(())
}
```

---

## 📚 Exemplos

O diretório `examples/` contém exemplos práticos:

- [`orderbook_analysis.rs`](examples/orderbook_analysis.rs) - Análise completa de order book

Para executar um exemplo:

```bash
cargo run --release --example orderbook_analysis
```

**Saída esperada:**
```
=== Market Microstructure Analyzer - Order Book Analysis ===

📊 Order Book Analysis:

  Bids (Buy Orders):
    Level 1: 1.5 @ $50000.00
    Level 2: 2.3 @ $49999.50
    Level 3: 0.8 @ $49999.00

  Asks (Sell Orders):
    Level 1: 1.2 @ $50001.00
    Level 2: 1.8 @ $50001.50
    Level 3: 2.5 @ $50002.00

  📈 Metrics:
    Best Bid: $50000.00
    Best Ask: $50001.00
    Spread: $1.00 (0.002%)

=== Analysis Complete ===
```

---

## 📖 Conceitos

### Order Book (Livro de Ofertas)

O order book mostra todas as ordens de compra (bids) e venda (asks) em diferentes níveis de preço:

```
Asks (Vendas)
$50002.00 | ████████ 2.5 BTC
$50001.50 | █████ 1.8 BTC
$50001.00 | ████ 1.2 BTC
─────────────────────────
$50000.00 | █████ 1.5 BTC  <- Best Bid
$49999.50 | ████████ 2.3 BTC
$49999.00 | ███ 0.8 BTC
Bids (Compras)
```

### Tape Reading

Tape reading é a análise do fluxo de negócios executados:
- **Green Trades:** Compras (taker comprou)
- **Red Trades:** Vendas (taker vendeu)
- **Block Trades:** Negócios grandes (>10 BTC)

### Order Imbalance

Mede o desequilíbrio entre compradores e vendedores:
- **Imbalance > 0:** Mais compradores (bullish)
- **Imbalance < 0:** Mais vendedores (bearish)
- **Imbalance ≈ 0:** Mercado equilibrado

---

## ⚡ Performance

### Benchmarks

| Operação | Tempo Médio | Throughput |
|----------|-------------|------------|
| Order Book Update | ~100μs | 10,000 ops/s |
| Spread Calculation | ~10μs | 100,000 ops/s |
| Imbalance Calculation | ~50μs | 20,000 ops/s |
| Pattern Detection | ~200μs | 5,000 ops/s |

### Otimizações

- ✅ Estruturas de dados eficientes (BTreeMap para order book)
- ✅ Cálculos incrementais (não recalcula tudo)
- ✅ Zero-copy quando possível
- ✅ Compilação otimizada com LTO

---

## 🗺️ Roadmap

- [x] Análise básica de order book
- [x] Cálculo de spread e imbalance
- [x] Tape reading básico
- [ ] WebSocket feed em tempo real
- [ ] Machine Learning para detecção de padrões
- [ ] Dashboard web interativo
- [ ] Suporte a múltiplos exchanges
- [ ] Alertas em tempo real
- [ ] Backtesting de estratégias baseadas em microestrutura

---

## 📜 Licença

Este projeto está licenciado sob a Licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

---

## ✍️ Autor

**Gabriel Demetrios Lafis**

Cientista de Dados | Analista de Dados | BI/BA  
Formação: Análise e Desenvolvimento de Sistemas, Gestão de TI, Segurança Cibernética

- 🔗 LinkedIn: [gabriel-demetrius](https://www.linkedin.com/in/gabriel-demetrius/)
- 💻 GitHub: [@galafis](https://github.com/galafis)
- 📧 Email: [Contato via LinkedIn](https://www.linkedin.com/in/gabriel-demetrius/)

---

<div align="center">

**⭐ Se este projeto foi útil, considere dar uma estrela!**

Made with ❤️ and Rust 🦀

</div>
