# Kiro Provider

Kiro (AWS CodeWhisperer) OAuth Provider Plugin for ProxyCast.

## Features

- OAuth authentication with AWS CodeWhisperer
- Support for Claude models via Kiro
- Token auto-refresh
- Risk control and fingerprint isolation
- Health check and credential management

## Installation

### Via ProxyCast Plugin Center

1. Open ProxyCast
2. Go to "Provider Pool" -> "OAuth Plugins"
3. Find "Kiro Provider" in recommended plugins
4. Click "Install"

### Manual Installation

Download the latest release from [GitHub Releases](https://github.com/aiclientproxy/kiro-provider/releases).

## Supported Platforms

- macOS (Apple Silicon): `kiro-provider-aarch64-apple-darwin`
- macOS (Intel): `kiro-provider-x86_64-apple-darwin`
- Windows (64-bit): `kiro-provider-x86_64-pc-windows-msvc.exe`
- Linux (x64): `kiro-provider-x86_64-unknown-linux-gnu`
- Linux (ARM64): `kiro-provider-aarch64-unknown-linux-gnu`

## CLI Usage

```bash
# Translate a request
kiro-provider-cli translate --input request.json --output response.json

# Refresh token
kiro-provider-cli refresh --credential-id <id>

# Health check
kiro-provider-cli health --credential-id <id>
```

## Configuration

See `plugin/config.json` for configuration options:

- `risk_control`: Risk control settings
- `token_refresh`: Token refresh settings
- `health_check`: Health check settings

## Development

### Prerequisites

- Rust 1.70+
- Cargo

### Build

```bash
cd src-tauri
cargo build --release --bin kiro-provider-cli
```

## License

MIT
