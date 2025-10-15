# Contribuindo para Market Microstructure Analyzer

Obrigado por considerar contribuir com este projeto! 🎉

## 🚀 Como Contribuir

### 1. Fork e Clone

```bash
# Fork o repositório no GitHub, depois clone:
git clone https://github.com/SEU_USUARIO/rust-market-microstructure-analyzer.git
cd rust-market-microstructure-analyzer
```

### 2. Configure o Ambiente

```bash
# Instale Rust (se ainda não tiver)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Compile o projeto
cargo build

# Execute os testes
cargo test
```

### 3. Crie uma Branch

```bash
git checkout -b feature/minha-feature
# ou
git checkout -b fix/meu-bugfix
```

### 4. Faça suas Mudanças

- Escreva código limpo e bem documentado
- Adicione testes para novas funcionalidades
- Mantenha os testes existentes funcionando
- Siga o estilo de código Rust

### 5. Teste suas Mudanças

```bash
# Execute os testes
cargo test

# Verifique formatação
cargo fmt --check

# Execute o linter
cargo clippy -- -D warnings

# Execute os exemplos
cargo run --example orderbook_analysis
```

### 6. Commit e Push

```bash
git add .
git commit -m "feat: adiciona nova funcionalidade X"
git push origin feature/minha-feature
```

### 7. Abra um Pull Request

- Vá para o GitHub e abra um Pull Request
- Descreva suas mudanças claramente
- Referencie issues relacionadas

## 📝 Guia de Estilo

### Código

- Use `cargo fmt` para formatar o código
- Use `cargo clippy` para verificar warnings
- Escreva documentação com `///` para funções públicas
- Use nomes descritivos para variáveis e funções

### Commits

Siga o padrão [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` Nova funcionalidade
- `fix:` Correção de bug
- `docs:` Mudanças na documentação
- `test:` Adiciona ou corrige testes
- `refactor:` Refatoração de código
- `perf:` Melhorias de performance
- `chore:` Tarefas de manutenção

Exemplos:
```
feat: adiciona detecção de padrão head and shoulders
fix: corrige cálculo de imbalance quando orderbook está vazio
docs: atualiza README com exemplos de uso
test: adiciona testes para módulo de patterns
```

## 🧪 Testes

### Escrevendo Testes

Cada módulo deve ter testes unitários:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_minha_funcao() {
        // Arrange
        let input = /* ... */;
        
        // Act
        let result = minha_funcao(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

### Executando Testes

```bash
# Todos os testes
cargo test

# Testes de um módulo específico
cargo test orderbook::tests

# Com saída detalhada
cargo test -- --nocapture

# Em modo release
cargo test --release
```

## 📚 Documentação

### Comentários de Documentação

Use `///` para documentar funções públicas:

```rust
/// Calcula o spread bid-ask
///
/// # Arguments
/// * `orderbook` - O order book para análise
///
/// # Returns
/// Uma tupla contendo (spread, spread_percentual)
///
/// # Examples
/// ```
/// let (spread, pct) = calculate_spread(&orderbook);
/// println!("Spread: ${} ({}%)", spread, pct);
/// ```
pub fn calculate_spread(orderbook: &OrderBook) -> (Decimal, Decimal) {
    // ...
}
```

## 🐛 Reportando Bugs

Ao reportar um bug, inclua:

1. **Descrição clara** do problema
2. **Passos para reproduzir**
3. **Comportamento esperado**
4. **Comportamento atual**
5. **Ambiente** (SO, versão do Rust, etc.)
6. **Código de exemplo** (se possível)

## 💡 Sugerindo Features

Ao sugerir uma feature:

1. **Explique o caso de uso**
2. **Descreva a solução proposta**
3. **Liste alternativas consideradas**
4. **Mencione possíveis impactos**

## 📋 Checklist do Pull Request

Antes de enviar um PR, verifique:

- [ ] O código compila sem erros (`cargo build`)
- [ ] Todos os testes passam (`cargo test`)
- [ ] Código está formatado (`cargo fmt`)
- [ ] Sem warnings do clippy (`cargo clippy`)
- [ ] Testes adicionados para novas funcionalidades
- [ ] Documentação atualizada
- [ ] Commit messages seguem o padrão
- [ ] PR tem descrição clara

## 🎯 Áreas para Contribuição

Procurando onde contribuir? Veja estas áreas:

### Funcionalidades Prioritárias
- [ ] Implementar WebSocket feed em tempo real
- [ ] Adicionar suporte a múltiplos exchanges
- [ ] Criar dashboard web interativo
- [ ] Implementar backtesting

### Melhorias
- [ ] Otimizações de performance
- [ ] Mais testes e cobertura
- [ ] Documentação adicional
- [ ] Exemplos de uso

### Bugs Conhecidos
Veja as [issues](https://github.com/galafis/rust-market-microstructure-analyzer/issues) marcadas como `bug`.

## 📞 Contato

Dúvidas? Entre em contato:
- Abra uma [issue](https://github.com/galafis/rust-market-microstructure-analyzer/issues)
- LinkedIn: [gabriel-demetrius](https://www.linkedin.com/in/gabriel-demetrius/)

## 📜 Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob a mesma licença MIT do projeto.

---

**Obrigado por contribuir! 🙏**
