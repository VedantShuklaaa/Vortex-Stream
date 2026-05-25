# Vortex Stream

Realtime multi-exchange crypto market data SDK for Node.js and Rust.

Vortex Stream provides unified realtime market data streams across multiple crypto exchanges through a single API.

***

# Features

* Unified exchange abstraction
* Realtime trade streaming
* Multi-exchange support
* Native Rust performance
* Node.js bindings via napi-rs
* Async websocket infrastructure
* Dynamic subscriptions
* Lightweight API

***

# Installation

```bash
npm install vortex-stream-sdk
bun add vortex-stream-sdk
```

***

# Quick Start

```js
const { VortexStream } = require("vortex-stream-sdk");

const stream = new VortexStream();

stream.trades(
  "binance",
  "BTCUSDT",
  (trade) => {
    console.log(trade);
  }
);

setInterval(() => {}, 1000);
```

***

# Trade Structure

```ts
{
  exchange: string;
  symbol: string;
  event_type: string;
  event_time: string;
  trade_id: string;
  last_price: string;
  quantity: string;
  is_buyer_maker?: boolean;
  timestamp: number;
}
```

***

# Supported Exchanges

* Binance
* Coinbase
* Bitfinex
* Bitget
* Bitstamp
* Bybit
* Crypto.com
* HTX
* Kraken
* Okx

***

# Examples

Inside:

```txt
examples/
```

Available examples:

* single\_exchange.rs
* multi\_exchange.rs
* dynamic\_subscriptions.rs
* unsubscribe.rs

***

# Roadmap

* Orderbook streams
* Candlestick streams
* Funding rates
* Liquidation streams
* Exchange reconnect resilience
* More exchange integrations
* Electron support examples
* Full TypeScript SDK

***

# License

MIT
