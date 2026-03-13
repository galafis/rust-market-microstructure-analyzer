# 📈 Rust Market Microstructure Analyzer

> High-performance market microstructure analysis engine built in Rust. Analyzes order flow, bid-ask spreads, trade classification, and market impact with sub-microsecond latency.

[![Rust](https://img.shields.io/badge/Rust-1.75-DEA584.svg)](https://img.shields.io/badge/)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED.svg)](https://img.shields.io/badge/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED.svg?logo=docker)](Dockerfile)

[English](#english) | [Português](#português)

---

## English

### 🎯 Overview

**Rust Market Microstructure Analyzer** is a production-grade Rust application that showcases modern software engineering practices including clean architecture, comprehensive testing, containerized deployment, and CI/CD readiness.

The codebase comprises **1,859 lines** of source code organized across **11 modules**, following industry best practices for maintainability, scalability, and code quality.

### ✨ Key Features

- **📈 Strategy Engine**: Multiple trading strategy implementations with configurable parameters
- **🔄 Backtesting Framework**: Historical data simulation with realistic market conditions
- **📊 Performance Analytics**: Sharpe ratio, Sortino ratio, maximum drawdown, and more
- **⚡ Real-time Processing**: Low-latency data processing optimized for market speed
- **🐳 Containerized**: Docker support for consistent deployment
- **🏗️ Object-Oriented**: 4 core classes with clean architecture

### 🏗️ Architecture

```mermaid
graph TB
    subgraph Data["📊 Market Data"]
        A[Data Feed]
        B[Historical Data]
    end
    
    subgraph Engine["⚙️ Analysis Engine"]
        C[Signal Generation]
        D[Strategy Logic]
        E[Risk Assessment]
    end
    
    subgraph Output["📈 Output"]
        F[Performance Metrics]
        G[Trade Signals]
        H[Reports]
    end
    
    A --> C
    B --> C
    C --> D --> E
    E --> F
    D --> G
    E --> H
    
    style Data fill:#e1f5fe
    style Engine fill:#f3e5f5
    style Output fill:#e8f5e9
```

```mermaid
classDiagram
    class VolumeProfile
    class OrderBook
    class Level
    class Trade
```

### 🚀 Quick Start

#### Prerequisites

- Rust 1.75+ (via [rustup](https://rustup.rs/))
- Cargo (included with Rust)

#### Installation

```bash
# Clone the repository
git clone https://github.com/galafis/rust-market-microstructure-analyzer.git
cd rust-market-microstructure-analyzer

# Build in release mode
cargo build --release
```

#### Running

```bash
# Run the application
cargo run --release

# Or run the binary directly
./target/release/rust_market_microstructure_analyzer
```

### 📁 Project Structure

```
rust-market-microstructure-analyzer/
├── docs/          # Documentation
│   └── API.md
├── examples/
│   ├── orderbook_analysis.rs
│   ├── pattern_detection.rs
│   └── tape_reading.rs
├── src/          # Source code
│   ├── metrics/
│   │   └── mod.rs
│   ├── orderbook/
│   │   └── mod.rs
│   ├── patterns/
│   │   └── mod.rs
│   ├── tape/
│   │   └── mod.rs
│   ├── visualization/
│   │   └── mod.rs
│   ├── lib.rs
│   ├── main.rs
│   └── types.rs
├── tests/         # Test suite
│   └── test_main.rs
├── CONTRIBUTING.md
├── Cargo.toml
├── Dockerfile
├── LICENSE
└── README.md
```

### 🛠️ Tech Stack

| Technology | Description | Role |
|------------|-------------|------|
| **Rust** | Core Language | Primary |
| **Docker** | Containerization platform | Framework |

### 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### 👤 Author

**Gabriel Demetrios Lafis**
- GitHub: [@galafis](https://github.com/galafis)
- LinkedIn: [Gabriel Demetrios Lafis](https://linkedin.com/in/gabriel-demetrios-lafis)

---

## Português

### 🎯 Visão Geral

**Rust Market Microstructure Analyzer** é uma aplicação Rust de nível profissional que demonstra práticas modernas de engenharia de software, incluindo arquitetura limpa, testes abrangentes, implantação containerizada e prontidão para CI/CD.

A base de código compreende **1,859 linhas** de código-fonte organizadas em **11 módulos**, seguindo as melhores práticas do setor para manutenibilidade, escalabilidade e qualidade de código.

### ✨ Funcionalidades Principais

- **📈 Strategy Engine**: Multiple trading strategy implementations with configurable parameters
- **🔄 Backtesting Framework**: Historical data simulation with realistic market conditions
- **📊 Performance Analytics**: Sharpe ratio, Sortino ratio, maximum drawdown, and more
- **⚡ Real-time Processing**: Low-latency data processing optimized for market speed
- **🐳 Containerized**: Docker support for consistent deployment
- **🏗️ Object-Oriented**: 4 core classes with clean architecture

### 🏗️ Arquitetura

```mermaid
graph TB
    subgraph Data["📊 Market Data"]
        A[Data Feed]
        B[Historical Data]
    end
    
    subgraph Engine["⚙️ Analysis Engine"]
        C[Signal Generation]
        D[Strategy Logic]
        E[Risk Assessment]
    end
    
    subgraph Output["📈 Output"]
        F[Performance Metrics]
        G[Trade Signals]
        H[Reports]
    end
    
    A --> C
    B --> C
    C --> D --> E
    E --> F
    D --> G
    E --> H
    
    style Data fill:#e1f5fe
    style Engine fill:#f3e5f5
    style Output fill:#e8f5e9
```

### 🚀 Início Rápido

#### Prerequisites

- Rust 1.75+ (via [rustup](https://rustup.rs/))
- Cargo (included with Rust)

#### Installation

```bash
# Clone the repository
git clone https://github.com/galafis/rust-market-microstructure-analyzer.git
cd rust-market-microstructure-analyzer

# Build in release mode
cargo build --release
```

#### Running

```bash
# Run the application
cargo run --release

# Or run the binary directly
./target/release/rust_market_microstructure_analyzer
```

### 📁 Estrutura do Projeto

```
rust-market-microstructure-analyzer/
├── docs/          # Documentation
│   └── API.md
├── examples/
│   ├── orderbook_analysis.rs
│   ├── pattern_detection.rs
│   └── tape_reading.rs
├── src/          # Source code
│   ├── metrics/
│   │   └── mod.rs
│   ├── orderbook/
│   │   └── mod.rs
│   ├── patterns/
│   │   └── mod.rs
│   ├── tape/
│   │   └── mod.rs
│   ├── visualization/
│   │   └── mod.rs
│   ├── lib.rs
│   ├── main.rs
│   └── types.rs
├── tests/         # Test suite
│   └── test_main.rs
├── CONTRIBUTING.md
├── Cargo.toml
├── Dockerfile
├── LICENSE
└── README.md
```

### 🛠️ Stack Tecnológica

| Tecnologia | Descrição | Papel |
|------------|-----------|-------|
| **Rust** | Core Language | Primary |
| **Docker** | Containerization platform | Framework |

### 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para enviar um Pull Request.

### 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

### 👤 Autor

**Gabriel Demetrios Lafis**
- GitHub: [@galafis](https://github.com/galafis)
- LinkedIn: [Gabriel Demetrios Lafis](https://linkedin.com/in/gabriel-demetrios-lafis)
