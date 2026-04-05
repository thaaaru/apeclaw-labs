<h1 align="center">ApeClaw — Our Claw</h1>

<p align="center">
  <strong>The Sovereign AI Agent for Sri Lanka</strong><br>
  ⚡️ Lightweight. Rust-native. Runs entirely on local hardware. Data stays on the island.
</p>

<p align="center">
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg" alt="License" /></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-edition%202024-orange?logo=rust" alt="Rust 2024" /></a>
  <img src="https://img.shields.io/badge/version-0.1.0--alpha-blue" alt="Version" />
  <img src="https://img.shields.io/badge/RAM-<5MB-brightgreen" alt="RAM" />
</p>

---

## What is ApeClaw?

ApeClaw is a fork of [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) specialized for the Sri Lankan
digital ecosystem. It is a fully local, agentic AI assistant written in 100% Rust — no Python, no Node, no
cloud dependency. Every byte of your data stays on your hardware.

**Mission:** *Our Claw — Lightweight Agentic AI for Sri Lanka.*

---

## Key Capabilities

| Feature | Detail |
|---|---|
| **Binary size** | ~5 MB stripped release binary |
| **RAM footprint** | < 5 MB at idle — runs on Raspberry Pi Zero 2W |
| **AI providers** | Anthropic, OpenAI, Ollama, Gemini, OpenRouter, GLM, ZAI |
| **Channels** | Telegram, Discord, Slack, WhatsApp, Email, Matrix, iMessage, IRC |
| **Lanka tools** | CBSL LKR exchange rates, CSE market data (Sinhala/Tamil Unicode) |
| **Audit logging** | Every tool call logged to `logs/apeclaw_audit.log` (ISO 27001 A.12.4) |
| **Hardware** | Raspberry Pi GPIO, STM32 Nucleo, ESP32, Arduino via serial |
| **Cron** | Built-in scheduler with IANA timezone support |
| **Memory** | SQLite-backed persistent memory — no cloud sync |

---

## Quickstart

```bash
# Install from source
cargo install --path .

# Initialize workspace
apeclaw onboard

# Start interactive agent session
apeclaw agent

# Fetch LKR rates (via agent prompt)
apeclaw agent -m "What are the current LKR exchange rates from CBSL?"

# Start the gateway (REST + WebSocket)
apeclaw gateway start
```

---

## Configuration

ApeClaw reads `~/.apeclaw/config.toml`. Key fields:

```toml
# LLM provider
default_provider = "anthropic"
default_model    = "claude-sonnet-4-6"

# Sri Lanka compliance audit log (ISO 27001 A.12.4)
# Appends every tool call and outbound request to logs/apeclaw_audit.log
sl_audit_enabled = true
```

---

## Sovereign Deployment

ApeClaw is designed to run entirely on hardware within Sri Lanka, keeping all
data, keys, and model inference under local control.

### Proxmox VM (recommended for enterprise)

```bash
# 1. Create a Debian 12 LXC container (512 MB RAM, 4 GB disk)
# 2. Install Rust
curl https://sh.rustup.rs -sSf | sh

# 3. Clone and build
git clone https://github.com/apeclaw-labs/apeclaw
cd apeclaw
cargo build --release

# 4. Install as a system service
sudo cp target/release/apeclaw /usr/local/bin/
apeclaw service install
apeclaw service start
```

### Raspberry Pi 4 / 5 (edge deployment)

```bash
# Cross-compile on your workstation
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# Copy binary to Pi (< 5 MB)
scp target/aarch64-unknown-linux-gnu/release/apeclaw pi@192.168.1.x:~/

# On the Pi
./apeclaw onboard
./apeclaw daemon   # runs gateway + agent + cron scheduler
```

### Raspberry Pi Zero 2W (ultra-lightweight edge)

The `release` profile is already optimized for size (`opt-level = "z"`, `lto = "fat"`, `strip = true`).
The resulting binary runs comfortably within the Zero 2W's 512 MB RAM.

```bash
# Target: armv7-unknown-linux-gnueabihf (Pi Zero 2W runs 32-bit Raspbian by default)
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target armv7-unknown-linux-gnueabihf
```

### Why sovereign deployment?

- **Data sovereignty**: LKR financial data, CBSL rates, CSE snapshots, and all agent memory
  remain on Sri Lankan hardware — not on foreign cloud servers.
- **Compliance**: `sl_audit_enabled = true` writes a local `logs/apeclaw_audit.log` with
  every tool invocation, satisfying ISO 27001 Annex A.12.4 and SOC 2 CC7.2 audit trail requirements.
- **Resilience**: No internet dependency for core agent functions. Local Ollama + ApeClaw
  continues operating during international link outages.
- **Cost**: A Raspberry Pi 4 costs ~$35. There are no cloud API fees for local inference.

---

## Lanka-Specific Tools

Located in `src/skills/lanka/`:

| Module | Purpose |
|---|---|
| `market.rs` | CBSL LKR exchange rates, CSE ASPI/S&P20 index snapshots |

All output supports **Sinhala (සිංහල)** and **Tamil (தமிழ்)** Unicode labels natively —
Rust's `str` type is UTF-8 from the ground up.

---

## Security & Audit

ApeClaw is built with a **security-first** architecture:

- **Local audit log**: `logs/apeclaw_audit.log` — append-only JSONL, one event per line.
- **No telemetry**: Zero outbound analytics or crash reporting.
- **Secret store**: AES-256-GCM (ChaCha20-Poly1305) encrypted local key store.
- **Autonomy controls**: Configurable tool allowlist, domain blocklist, approval gates.
- **Sandbox**: Optional Landlock (Linux) or Bubblewrap process isolation.

---

## License

Apache-2.0

---

## Acknowledgements

ApeClaw is built on the shoulders of [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw)
by the ZeroClaw Labs team. Respect to the original engineers.
