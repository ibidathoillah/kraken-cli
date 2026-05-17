# kraken-cli

Unofficial AI-native Rust CLI for Kraken Spot, Margin, Futures, and xStocks. Use it to inspect markets, manage account data, place orders, stream live WebSocket events, run paper trading, and expose the same command surface to AI agents through MCP.

[![Rust](https://img.shields.io/badge/Rust-2021-000000?logo=rust)](https://www.rust-lang.org/)
[![CLI](https://img.shields.io/badge/interface-terminal-2f855a)](#quick-start)
[![WebSocket](https://img.shields.io/badge/websocket-live-2563eb)](#websocket-streaming)
[![MCP](https://img.shields.io/badge/MCP-ready-7c3aed)](#mcp-server)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

## Highlights

- **Multi-Asset Support**: Covers Crypto Spot, Tokenized U.S. Stocks (xStocks), Forex, and Perpetual Futures.
- **AI-Native Design**: Built-in Model Context Protocol (MCP) server for seamless integration with Cursor, Claude, and Gemini.
- **Robust Market Data**: Instant access to tickers, order books, OHLC, and recent trades with flexible pair normalization.
- **Safe Testing**: Independent local paper trading engines for both Spot and Futures.
- **Automation Ready**: Structured JSON output with stable error envelopes for reliable scripting.
- **Advanced Features**: Staking (Earn), subaccount management, and high-performance WebSocket streaming.

## Installation

### Binary (One-liner)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/ibidathoillah/kraken-cli/releases/latest/download/kraken-cli-installer.sh | sh
```

### From npm

```bash
npm install -g @ibidathoillah/kraken-cli
```

### From Source (Cargo)

```bash
cargo install --git https://github.com/ibidathoillah/kraken-cli.git
```

## Quick Start

Market data does not require credentials:

```bash
kraken ticker btc/usd
kraken orderbook eth/usd --count 10
kraken ohlc sol/usd --interval 60
kraken -o json ticker btc/usd
```

Configure private API credentials:

```bash
kraken auth set --api-key YOUR_API_KEY --api-secret YOUR_API_SECRET
kraken auth test
kraken auth show
```

## Command Reference

### Market (Spot & xStocks)

```bash
kraken status
kraken ticker btc/usd
kraken orderbook eth/usd --count 20
kraken trades btc/usd --count 50
kraken ohlc sol/usd --interval 1440
kraken assets
kraken pairs
```

### Account & Trading

```bash
kraken balance
kraken open-orders
kraken order buy btc/usd 0.001 --type limit --price 50000
kraken order sell eth/usd 0.1 --type market
kraken order cancel <TXID>
kraken --yes order cancel-all
```

### Futures (USDS-M)

```bash
kraken futures instruments
kraken futures tickers
kraken futures ticker PI_XBTUSD
kraken futures orderbook PI_XBTUSD
kraken futures history PI_XBTUSD
kraken futures balance
kraken futures positions
kraken futures order buy PI_XBTUSD 1 --type market
```

### Paper Trading

Local risk-free simulation (no API keys required).

```bash
# Spot Paper
kraken paper init
kraken paper buy btc/usd 0.1
kraken paper status

# Futures Paper
kraken futures paper init
kraken futures paper buy PI_XBTUSD 5 --leverage 10
kraken futures paper positions
```

### WebSocket Streaming

```bash
kraken ws ticker btc/usd eth/usd
kraken ws trades sol/usd
kraken ws book btc/usd --depth 10
kraken ws ohlc eth/usd --interval 1
```

### MCP Server

Expose Kraken tools to AI agents.

```bash
kraken mcp -s market,trade,account,futures
```

Example configuration for Claude Desktop:

```json
{
  "mcpServers": {
    "kraken": {
      "command": "kraken",
      "args": ["mcp", "-s", "all"]
    }
  }
}
```

## Output Formats

Every command supports `--output table` (default) and `--output json` (NDJSON for streaming).

```bash
kraken balance -o json
```

## Security

- Credentials stored with `0600` permissions in `~/.config/kraken/config.toml`.
- Uses HMAC-SHA512 signing for all private API requests.
- Pass `--yes` to skip destructive operation prompts.

## Development

```bash
cargo build
cargo test
```

## Related Projects

If you use multiple exchanges, check out these related CLI tools built with the same architecture:

- [indodax-cli](https://github.com/ibidathoillah/indodax-cli) - CLI for Indodax
- [bittime-cli](https://github.com/ibidathoillah/bittime-cli) - CLI for Bittime
- [binance-cli](https://github.com/ibidathoillah/binance-cli) - CLI for Binance Spot
- [tokocrypto-cli](https://github.com/ibidathoillah/tokocrypto-cli) - CLI for Tokocrypto
- [kraken-cli](https://github.com/ibidathoillah/kraken-cli) - CLI for Kraken (Spot, Margin, Futures)

## License

MIT. See [LICENSE](LICENSE).

## Disclaimer

This project is unofficial and is not affiliated with or endorsed by Kraken. Cryptocurrency trading is risky; review commands carefully before using write-capable API keys.
