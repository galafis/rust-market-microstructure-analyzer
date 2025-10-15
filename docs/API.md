# API Documentation

## Módulos

### 📖 OrderBook Module

Módulo para análise de order book (livro de ofertas).

#### Funções Principais

##### `calculate_spread`

Calcula o spread bid-ask.

```rust
pub fn calculate_spread(orderbook: &OrderBook) -> Option<(Decimal, Decimal)>
```

**Parâmetros:**
- `orderbook` - O order book para análise

**Retorna:**
- `Some((spread, spread_percentage))` - Tupla com spread absoluto e percentual
- `None` - Se o order book estiver vazio

**Exemplo:**
```rust
use market_microstructure_analyzer::orderbook;

if let Some((spread, spread_pct)) = orderbook::calculate_spread(&orderbook) {
    println!("Spread: ${} ({:.4}%)", spread, spread_pct);
}
```

---

##### `calculate_imbalance`

Calcula o desequilíbrio do order book.

```rust
pub fn calculate_imbalance(orderbook: &OrderBook, depth: Option<usize>) -> Decimal
```

**Parâmetros:**
- `orderbook` - O order book para análise
- `depth` - Número de níveis a considerar (None para todos)

**Retorna:**
- Valor entre -1.0 e 1.0
  - Positivo: mais pressão de compra
  - Negativo: mais pressão de venda
  - Zero: mercado equilibrado

**Exemplo:**
```rust
let imbalance = orderbook::calculate_imbalance(&orderbook, Some(5));

if imbalance > dec!(0.5) {
    println!("Forte pressão de compra!");
}
```

---

##### `best_bid` / `best_ask` / `mid_price`

Obtém preços do topo do livro.

```rust
pub fn best_bid(orderbook: &OrderBook) -> Option<Decimal>
pub fn best_ask(orderbook: &OrderBook) -> Option<Decimal>
pub fn mid_price(orderbook: &OrderBook) -> Option<Decimal>
```

**Exemplo:**
```rust
if let Some(mid) = orderbook::mid_price(&orderbook) {
    println!("Mid price: ${}", mid);
}
```

---

### 📊 Metrics Module

Módulo para cálculo de métricas avançadas.

#### Estruturas

##### `VolumeProfile`

```rust
pub struct VolumeProfile {
    pub levels: HashMap<Decimal, Decimal>,  // Níveis de preço e volumes
    pub poc: Option<Decimal>,                // Point of Control
    pub vah: Option<Decimal>,                // Value Area High
    pub val: Option<Decimal>,                // Value Area Low
}
```

#### Funções

##### `calculate_volume_profile`

Calcula o perfil de volume dos trades.

```rust
pub fn calculate_volume_profile(trades: &[Trade], tick_size: Decimal) -> VolumeProfile
```

**Parâmetros:**
- `trades` - Lista de trades executados
- `tick_size` - Tamanho do tick para agrupamento

**Exemplo:**
```rust
let profile = metrics::calculate_volume_profile(&trades, dec!(1.0));

if let Some(poc) = profile.poc {
    println!("POC (maior volume): ${}", poc);
}
```

---

##### `calculate_delta`

Calcula o delta de volume (compras - vendas).

```rust
pub fn calculate_delta(trades: &[Trade]) -> Decimal
```

**Retorna:**
- Positivo: mais volume de compra
- Negativo: mais volume de venda

**Exemplo:**
```rust
let delta = metrics::calculate_delta(&trades);

if delta > dec!(0) {
    println!("Pressão de compra dominante");
}
```

---

##### `calculate_cvd`

Calcula o Cumulative Volume Delta ao longo do tempo.

```rust
pub fn calculate_cvd(trades: &[Trade]) -> Vec<(i64, Decimal)>
```

**Retorna:**
- Vetor de (timestamp, cvd_acumulado)

**Exemplo:**
```rust
let cvd = metrics::calculate_cvd(&trades);

for (timestamp, value) in cvd.iter().rev().take(5) {
    println!("{}: {:.2}", timestamp, value);
}
```

---

##### `weighted_mid_price`

Calcula o mid price ponderado pelos volumes.

```rust
pub fn weighted_mid_price(orderbook: &OrderBook) -> Option<Decimal>
```

---

### 🔍 Patterns Module

Módulo para detecção de padrões.

#### Enums

##### `Pattern`

```rust
pub enum Pattern {
    IcebergOrder { price: Decimal, estimated_size: Decimal },
    Spoofing { price: Decimal, side: String },
    Support { price: Decimal, strength: Decimal },
    Resistance { price: Decimal, strength: Decimal },
    Absorption { price: Decimal, volume: Decimal },
}
```

#### Funções

##### `detect_iceberg_orders`

Detecta ordens iceberg (grandes ordens ocultas).

```rust
pub fn detect_iceberg_orders(
    trades: &[Trade],
    min_fills: usize,
    price_tolerance: Decimal,
) -> Vec<Pattern>
```

**Parâmetros:**
- `trades` - Trades recentes
- `min_fills` - Número mínimo de fills para considerar
- `price_tolerance` - Tolerância de preço para agrupar

**Exemplo:**
```rust
let icebergs = patterns::detect_iceberg_orders(&trades, 3, dec!(1.0));

for pattern in icebergs {
    if let Pattern::IcebergOrder { price, estimated_size } = pattern {
        println!("Iceberg em ${}: ~{} total", price, estimated_size);
    }
}
```

---

##### `detect_spoofing`

Detecta possível spoofing (ordens falsas).

```rust
pub fn detect_spoofing(orderbook: &OrderBook, threshold: Decimal) -> Vec<Pattern>
```

---

##### `detect_support_resistance`

Detecta níveis de suporte e resistência.

```rust
pub fn detect_support_resistance(orderbook: &OrderBook, threshold: Decimal) -> Vec<Pattern>
```

---

##### `detect_absorption`

Detecta absorção de liquidez.

```rust
pub fn detect_absorption(
    trades: &[Trade],
    volume_threshold: Decimal,
    price_range: Decimal,
) -> Vec<Pattern>
```

---

### 📈 Tape Module

Módulo para análise de tape reading.

#### Enums

##### `TradeType`

```rust
pub enum TradeType {
    Buy,
    Sell,
    Block { side: String },
}
```

#### Funções

##### `classify_trade`

Classifica um trade individual.

```rust
pub fn classify_trade(trade: &Trade, block_threshold: Decimal) -> TradeType
```

---

##### `calculate_trade_pressure`

Calcula pressão de compra vs venda.

```rust
pub fn calculate_trade_pressure(trades: &[Trade]) -> (Decimal, Decimal, Decimal)
```

**Retorna:**
- `(buy_volume, sell_volume, net_volume)`

**Exemplo:**
```rust
let (buy, sell, net) = tape::calculate_trade_pressure(&trades);

println!("Buy: {}, Sell: {}, Net: {}", buy, sell, net);
```

---

##### `identify_block_trades`

Identifica block trades (trades grandes).

```rust
pub fn identify_block_trades(trades: &[Trade], threshold: Decimal) -> Vec<Trade>
```

---

##### `calculate_aggression_ratio`

Calcula ratio de agressão dos compradores.

```rust
pub fn calculate_aggression_ratio(trades: &[Trade]) -> Decimal
```

**Retorna:**
- Valor entre 0.0 e 1.0
  - > 0.6: alta agressão de compra
  - < 0.4: alta agressão de venda

---

##### `detect_trade_clusters`

Detecta clusters de trades rápidos.

```rust
pub fn detect_trade_clusters(
    trades: &[Trade],
    time_window: i64,
    min_cluster_size: usize,
) -> Vec<usize>
```

---

##### `calculate_vwap`

Calcula Volume-Weighted Average Price.

```rust
pub fn calculate_vwap(trades: &[Trade]) -> Option<Decimal>
```

**Exemplo:**
```rust
if let Some(vwap) = tape::calculate_vwap(&trades) {
    println!("VWAP: ${:.2}", vwap);
}
```

---

### 🎨 Visualization Module

Módulo para visualização.

#### Funções

##### `print_orderbook`

Imprime order book formatado.

```rust
pub fn print_orderbook(orderbook: &OrderBook, levels: usize)
```

---

##### `print_trades`

Imprime tape de trades.

```rust
pub fn print_trades(trades: &[Trade], limit: usize)
```

---

##### `ascii_depth_chart`

Gera gráfico ASCII de profundidade.

```rust
pub fn ascii_depth_chart(orderbook: &OrderBook, height: usize) -> String
```

**Exemplo:**
```rust
let chart = visualization::ascii_depth_chart(&orderbook, 10);
println!("{}", chart);
```

---

## Tipos de Dados

### `OrderBook`

```rust
pub struct OrderBook {
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
    pub timestamp: i64,
}
```

### `Level`

```rust
pub struct Level {
    pub price: Decimal,
    pub quantity: Decimal,
}
```

### `Trade`

```rust
pub struct Trade {
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: String,  // "buy" ou "sell"
    pub timestamp: i64,
}
```

---

## Exemplos Completos

Veja o diretório [`examples/`](../examples/) para exemplos completos de uso.
