# vectordb

`vectordb` is a high-performance distributed database focused on vector searches using Approximate Nearest Neighbor Search (ANN) combined with support for filters, text search, and flexible document storage. Inspired by systems like Elasticsearch, but with a focus on high-dimensional vectors and scalability.

## Goals

- **Schema-free**: No need to define schemas in advance.
- **Approximate Nearest Neighbor (ANN) Search**: Supports HSNW (Hierarchical Navigable Small World) for vector searches.
- **Flexible Search**: Combines filters, text search, and vector search.
- **On-the-fly Operations**: Insert, remove, and update documents in real-time.
- **Bulk Operations**: Supports bulk operations for greater efficiency.
- **Scalability**: Can be deployed in configurations ranging from singletons to elastic clusters.

## Technologies

- **Language**: [Rust](https://www.rust-lang.org/)
- **Communication Protocol**: TCP/Sockets (with potential future support for UDP)
- **Serialization**: [MessagePack](https://msgpack.org/) (for low latency communication, faster parsing)
- **Vector Search Algorithm**: HSNW (Hierarchical Navigable Small World)

## Performance and Scalability Goals

- **Storage**: Support for billions of documents, totaling up to terabytes of data.
- **Latency**: Low latency for specific operations (insert, remove, update). Tolerates up to 30 seconds of latency for indexing.
- **Vectors**: Support for vectors as big as 4096 dimensions.

## Installation

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- Cargo - Rust’s package manager

### Cloning the repository

```bash
git clone https://github.com/jejung/vectordb.git
cd vectordb
```

### Building the project

```bash
cargo build
```

### Running locally (default setup)

```bash
cargo run
```

### Running tests

```bash
cargo test
```

### How to Contribute

- Fork the project
- Create your feature branch (git checkout -b feature/your-feature)
- Commit your changes (git commit -m 'Add new feature')
- Push to the branch (git push origin feature/your-feature)
- Open a Pull Request

### Roadmap

- Communication Setup: Basic TCP communication implementation.
- Document Insertion and Manipulation: Implement document insertion, updating, and removal.
- Vector Search: Implement HSNW and vector search capabilities.
- Scalability and Clustering: Add support for distributed clusters.

### License

This project is licensed under the [MIT License](https://opensource.org/license/mit).
