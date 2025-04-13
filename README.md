# ChronicleDB ⏰🕰️

**ChronicleDB** is an open-source database library written in [Rust](https://www.rust-lang.org/), drawing inspiration from [Datomic](https://www.datomic.com/). It aims to support all Datomic features (e.g., immutability, time-travel queries, Datalog) with a PostgreSQL backend. This is a **research project** exploring database design and is **not ready for production use**. 🕒

## Features (Planned) 📅
- Full Datomic feature parity, including immutable data and temporal queries.
- PostgreSQL backend for persistent storage.
- Rust-powered for performance and memory safety.

## Getting Started ⏲️

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable or nightly, check project requirements).
- [Nix](https://nixos.org/download.html) for dependency management via `flake.nix`.
- PostgreSQL (required for backend, once implemented).

### Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/qleguennec/chronicledb.git
   cd chronicledb
   ```

   
2. Run tests to verify the library:
  ```bash
   cargo test
  ```

### Project status 
ChronicleDB is under active development as a research project. It’s experimental, with incomplete features and potential breaking changes.

### Contributing
Not open to contributions.

### Acknowledgments 
Inspired by Datomic’s innovative approach to databases.

