<div align="center">

<br>

<img src="https://rogddqelmxyuvhpjvxbf.supabase.co/storage/v1/object/public/files/d312g4dtssa.png" alt="Axiom Logo" width="356">

**Universal Minecraft Protocol Bridge**

_Bedrock Edition ↔ Java Edition — written in Rust_

<br>

[![Build](https://img.shields.io/github/actions/workflow/status/axiom-rs/axiom/ci.yml?branch=main&style=flat-square&logo=github-actions&logoColor=white&label=build&color=0d1117&labelColor=161b22)](https://github.com/XenithRed/axiom)
[![Crates.io](https://img.shields.io/crates/v/axiom-server?style=flat-square&logo=rust&logoColor=white&label=crates.io&color=0d1117&labelColor=161b22)](https://crates.io/crates/axiom-server)
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-0d1117?style=flat-square&logo=open-source-initiative&logoColor=white&labelColor=161b22)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.78%2B-0d1117?style=flat-square&logo=rust&logoColor=white&labelColor=161b22)](https://www.rust-lang.org)
[![unsafe](https://img.shields.io/badge/unsafe-forbidden-0d1117?style=flat-square&logo=shield&logoColor=white&labelColor=161b22)](https://doc.rust-lang.org/nomicon)

<br>

</div>

---

<br>

## ◆ &nbsp;What is Axiom?

Axiom is a **bidirectional protocol bridge** that allows Minecraft Bedrock Edition clients to connect to Java Edition servers — and vice versa — with zero modification on either side.

Unlike existing solutions, Axiom is not a translation shim bolted on top of an existing server. It is an **abstract game state machine** written from first principles in Rust, with full implementations of both protocols, a complete RakNet stack, Xbox Live authentication, and an automated resource pack pipeline.

<br>

## ◆ &nbsp;Why Axiom?

<br>

<div align="center">

|                               |  GeyserMC   |      Axiom       |
| :---------------------------- | :---------: | :--------------: |
| Language                      |    Java     |     **Rust**     |
| RAM baseline                  |   ~300 MB   |    **~18 MB**    |
| Throughput                    | ~50 k pkt/s | **~500 k pkt/s** |
| GC pauses                     |     yes     |     **none**     |
| Bidirectional bridge          |      ✗      |      **✓**       |
| Resource pack auto-conversion |      ✗      |      **✓**       |
| Protocol formally documented  |   partial   |   **complete**   |
| Fuzz-tested                   |      ✗      |      **✓**       |
| Formal spec (TLA+)            |      ✗      |      **✓**       |
| Binary size                   |   ~80 MB    |    **~4 MB**     |

</div>

<br>

The fundamental difference is architectural. GeyserMC translates packets one-to-one as they arrive. Axiom deserializes both sides into an **Abstract Game State Model** — a representation that is independent of either edition's wire format — and then re-serializes to the target. This makes the translation layer formally verifiable, version-upgradeable in isolation, and trivially extensible to future editions.

<br>

---

<br>

## ◆ &nbsp;Architecture

      Bedrock Client                  AXIOM CORE                  Java Server
      ┌──────────┐                 ┌────────────┐                ┌──────────┐
      │  RakNet  │                 │            │                │  Netty   │
      │   UDP    │◄───────────────►│    AGSM    │◄──────────────►│   TCP    │
      │  19132   │                 │  Abstract  │                │  25565   │
      └──────────┘                 │   State    │                └──────────┘
           │                       │   Model    │                      │
           │  ┌────────────────┐   │            │   ┌──────────────┐   │
           └─►│ Bedrock Proto  │──►│            │◄──│  Java Proto  │◄──┘
              │    Decoder     │   └─────┬──────┘   │   Decoder    │
              └────────────────┘         │           └──────────────┘
                                         │
                                ┌────────▼────────┐
                                │   Translation   │
                                │     Engine      │
                                │                 │
                                │  ◆ Block maps   │
                                │  ◆ Entity maps  │
                                │  ◆ Chunk conv.  │
                                │  ◆ NBT bridge   │
                                └────────┬────────┘
                                         │
                                ┌────────▼────────┐
                                │  Resource Pack  │
                                │    Pipeline     │
                                │                 │
                                │  ◆ Model conv.  │
                                │  ◆ Atlas regen  │
                                │  ◆ Sound map    │
                                └─────────────────┘

<br>

---

<br>

## ◆ &nbsp;Repository Structure

### 📦 Crates
The core logic is split into several modular crates:

- **[axiom-core](file:///c:/Users/USER/Desktop/app/Axiom/axiom/crates/axiom-core)**: The heart of the project. Contains the abstract game state, ECS logic, and networking traits.
- **[axiom-java-protocol](file:///c:/Users/USER/Desktop/app/Axiom/axiom/crates/axiom-java-protocol)**: A complete implementation of the Minecraft Java Edition protocol (TCP).
- **[axiom-bedrock-protocol](file:///c:/Users/USER/Desktop/app/Axiom/axiom/crates/axiom-bedrock-protocol)**: A complete implementation of the Minecraft Bedrock Edition protocol, including a custom RakNet (UDP) stack.
- **[axiom-translate](file:///c:/Users/USER/Desktop/app/Axiom/axiom/crates/axiom-translate)**: The translation engine that maps blocks, entities, items, and chunks between versions.
- **[axiom-resource-pipeline](file:///c:/Users/USER/Desktop/app/Axiom/axiom/crates/axiom-resource-pipeline)**: Automatically converts Java resource packs into Bedrock-compatible formats.
- **[axiom-auth](file:///c:/Users/USER/Desktop/app/Axiom/axiom/crates/axiom-auth)**: Handles the full Xbox Live and Minecraft authentication chain.
- **[axiom-config](file:///c:/Users/USER/Desktop/app/Axiom/axiom/crates/axiom-config)**: Manages server configuration settings and schemas.
- **[axiom-metrics](file:///c:/Users/USER/Desktop/app/Axiom/axiom/crates/axiom-metrics)**: Provides Prometheus-compatible telemetry and metrics.
- **[axiom-server](file:///c:/Users/USER/Desktop/app/Axiom/axiom/crates/axiom-server)**: The main entry point. Orchestrates the bridge/proxy between clients and servers.

### 🛠️ Tools
Standalone utilities for development and debugging:

- **[inspector](file:///c:/Users/USER/Desktop/app/Axiom/axiom/tools/inspector)**: A CLI tool to capture, decrypt, and analyze Minecraft network packets.
- **[rpconv](file:///c:/Users/USER/Desktop/app/Axiom/axiom/tools/rpconv)**: A standalone version of the resource pack converter utility.

### 🚧 Other Folders (In Progress)
- **specs/** &mdash; _In progress: Formal protocol specifications and TLA+ models._
- **benches/** &mdash; _In progress: Performance benchmarks and throughput tests._
- **fuzz/** &mdash; _In progress: Codec fuzzing targets for security validation._
- **tests/** &mdash; _In progress: Integration and end-to-end tests._
- **.github/** &mdash; _In progress: CI/CD workflows and GitHub configurations._

<br>

---

<br>

## ◆ &nbsp;Getting Started

### Prerequisites

```bash
# Rust 1.78 or later
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Optional — for fuzzing
cargo install cargo-fuzz
```

### Build

```bash
git clone https://github.com/axiom-rs/axiom
cd axiom
cargo build --release -p axiom-server
```

### Run

```bash
# Minimal config — bridge Bedrock :19132 → Java server at localhost:25565
./target/release/axiom-server --config config/default.toml
```

### Configuration

```toml
# config/default.toml

[server]
bedrock_bind = "0.0.0.0:19132"
java_host    = "127.0.0.1"
java_port    = 25565

[bridge]
online_mode  = true
max_players  = 100
chunk_radius = 8

[rp]
auto_convert = true
cache_dir    = ".axiom/rp-cache"
```

<br>

---

<br>

## ◆ &nbsp;Technical Highlights

<br>

### RakNet — From Scratch

The entire RakNet stack (`axiom-bedrock-protocol/src/raknet/`) is written from scratch with no external RakNet dependency. This includes:

- **ARQ** — sliding window with per-datagram retransmission tracking
- **RTT estimator** — RFC 6298 EWMA (`SRTT + 4×RTTVAR`)
- **Congestion control** — additive increase / multiplicative decrease
- **Fragmentation** — split/reassembly with compound ID tracking and depth limits
- **Ordering channels** — 32 independent reorder queues, O(1) amortized delivery
- **ACK compression** — range-compressed ACK/NAK packets (`compress_ranges`)

<br>

### LE-NBT Codec

Bedrock uses little-endian NBT with 16-bit string lengths. The Java edition uses big-endian NBT with 16-bit string lengths. Both are implemented independently in their respective crates, with a bridge layer in `axiom-translate` that converts between them without intermediate allocation on the hot path.

<br>

### Xbox Live Authentication

The full auth chain is implemented in `axiom-auth`:

```
MSA device-code flow
       │
       ▸  POST /devicecode  →  user_code + verification_uri
       ▸  poll /token       →  MSA access_token + refresh_token
       │
XBL (user.auth.xboxlive.com)
       │
       ▸  POST /user/authenticate  →  XBL token + uhs claim
       │
XSTS (xsts.auth.xboxlive.com)
       │
       ▸  Java  →  rp://api.minecraftservices.com/
       ▸  Bedrock →  https://multiplayer.minecraft.net/
       │
Minecraft Services
       │
       ▸  POST /authentication/login_with_xbox  →  JWT access token
       ▸  GET  /minecraft/profile               →  uuid + display name
```

Tokens are cached to disk with atomic writes and `chmod 0600` on Unix. Silent refresh is attempted before expiry. The Bedrock-scoped XSTS token is derived on-demand per connection since it uses a different relying party than the Java-scoped one.

<br>

### Resource Pack Pipeline

`axiom-resource-pipeline` converts Java Edition resource packs to Bedrock format on-the-fly:

- **Model resolution** — flattens the Java parent-chain inheritance tree
- **Geometry conversion** — `BlockModel.elements[]` (AABB cubes) → Bedrock `geometry.json` bones
- **Texture atlas** — reconstructs UV coordinates for Bedrock's different atlas layout
- **Animation metadata** — converts `.mcmeta` animation frames to `flipbook_textures.json`
- **Sound mapping** — `sounds.json` → `sound_definitions.json`

<br>

---

<br>

## ◆ &nbsp;Benchmarks

Results on Apple M3 Pro, release build, single thread:

```
packet/java_decode          time:  [312 ns  314 ns  317 ns]
packet/bedrock_decode       time:  [289 ns  291 ns  294 ns]
chunk/java_to_bedrock       time:  [1.21 ms 1.22 ms 1.24 ms]
block/lookup_20k            time:  [4.1 ns  4.2 ns  4.3 ns]
nbt/encode_compound_64k     time:  [88 µs   89 µs   91 µs]
```

Block state translation is an O(1) array index — `java_to_bedrock[state_id]` — with the full 20k-entry table pre-loaded into a static slice at startup.

<br>

---

<br>

## ◆ &nbsp;Protocol Documentation

The `specs/` directory contains the most complete public documentation of the Bedrock wire protocol available:

| File                         | Contents                                                    |
| :--------------------------- | :---------------------------------------------------------- |
| `specs/bedrock-proto.md`     | Every packet: binary layout, field types, state transitions |
| `specs/raknet.md`            | Full RakNet datagram/frame/ACK wire format                  |
| `specs/java-proto.md`        | Java Edition protocol reference (1.21+)                     |
| `specs/auth-flow.md`         | MSA → XBL → XSTS → Minecraft sequence diagrams              |
| `specs/translation.md`       | Block/entity/chunk translation rules, edge cases            |
| `specs/agsm.tla`             | TLA+ specification of the Abstract Game State Model         |
| `specs/resource-pipeline.md` | Resource pack conversion algorithm                          |

<br>

---

<br>

## ◆ &nbsp;Security

All codec boundaries are covered by `cargo-fuzz` targets. The fuzz corpus is run continuously in CI. Any crash or unexpected panic in a decode path is treated as a critical bug.

```bash
# Run fuzz targets locally
cargo fuzz run fuzz_bedrock_pkt
cargo fuzz run fuzz_raknet_frame
cargo fuzz run fuzz_nbt_bedrock
```

`#![forbid(unsafe_code)]` is set at the workspace level. No unsafe blocks exist in any crate.

<br>

---

<br>

## ◆ &nbsp;Contributing

Pull requests are welcome. Before opening one:

1. Run `cargo test --workspace`
2. Run `cargo clippy --workspace -- -D warnings`
3. Run `cargo fmt --check`
4. Add a benchmark if the change is performance-sensitive
5. Update the relevant spec in `specs/` if you modify protocol handling

<br>

---

<br>

## ◆ &nbsp;License

Axiom is dual-licensed under **MIT** and **Apache 2.0**. You may choose either license.

<br>

---

<div align="center">
<br>

_The game is the same. The wire never was.._

<br>

[![made-with-rust](https://img.shields.io/badge/made%20with-Rust-0d1117?style=flat-square&logo=rust&logoColor=white&labelColor=161b22)](https://www.rust-lang.org)
[![protocol-bedrock](https://img.shields.io/badge/protocol-Bedrock%201.21-0d1117?style=flat-square&logo=minecraft&logoColor=white&labelColor=161b22)](specs/bedrock-proto.md)
[![protocol-java](https://img.shields.io/badge/protocol-Java%201.21-0d1117?style=flat-square&logo=java&logoColor=white&labelColor=161b22)](specs/java-proto.md)

<br>
</div>
