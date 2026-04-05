# Contributing to ApeClaw

Thanks for your interest in contributing to ApeClaw — the Sovereign AI Agent for Sri Lanka.

ApeClaw is a fork of [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw). Contributions that
strengthen the Sri Lankan ecosystem, localization, security, and sovereign deployment are especially welcome.

---

## Branching Model

- **`main`** is the default branch and single source of truth
- Create `feat/*` or `fix/*` branches from `main`
- Open PRs targeting `main`

---

## Areas We Welcome Contributions

| Area | Examples |
|---|---|
| **Lanka tools** | CBSL API integration, CSE data, government APIs, BoC rates |
| **Localization** | Sinhala / Tamil UI strings, `tool_descriptions/si.toml` |
| **Security** | Audit logging, sandboxing, compliance (ISO 27001, SOC 2) |
| **Sovereign deployment** | Proxmox templates, RPi images, offline install scripts |
| **Documentation** | Sinhala README, deployment guides, video tutorials |
| **Bug fixes** | Any upstream or ApeClaw-specific issues |

---

## Development Setup

```bash
# Prerequisites: Rust 1.87+
curl https://sh.rustup.rs -sSf | sh

# Clone and build
git clone https://github.com/thaaaru/apeclaw-labs.git
cd apeclaw-labs
cargo build

# Check
cargo check

# Run tests
cargo test --lib
```

---

## Code Standards

- Run `cargo clippy` before submitting — we enforce `#![warn(clippy::all, clippy::pedantic)]`
- Format with `cargo fmt`
- New Lanka-specific tools go in `src/skills/lanka/`
- All new modules must have at least one `#[test]`
- Sinhala/Tamil strings are native UTF-8 — no special handling needed

---

## Commit Style

```
feat(lanka): add CBSL LKR exchange rate fetcher
fix(audit): handle missing logs/ directory on first run
docs: add Sinhala tool descriptions
```

---

## Contact

- **Issues:** https://github.com/thaaaru/apeclaw-labs/issues
- **Security vulnerabilities:** See [SECURITY.md](./SECURITY.md)
- **Maintainer:** Tharaka Kaushalya Mahabage

---

## Attribution

ApeClaw is built on [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) by ZeroClaw Labs,
licensed MIT OR Apache-2.0. We are grateful for their foundational work.
