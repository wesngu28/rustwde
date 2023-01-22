# Web Dev Evaluator Backend

The backend for the [web dev evaluator collab project monorepo](https://github.com/kyleung1/WebdevEvaluator).

This was my second time with Rust and the first time not in a crunched hackathon experience. I learned a lot more about error handling with Rust in this, as well as using the web framework [Axum](https://crates.io/crates/axum) and connection to MongoDB.

I switched between using MongoDB and SQLite multiple times during development, using the official MongoDB wrapper for Rust and Rusqlite for them respectively. I chose to use MongoDB at the end for production as the setup and process necessary to embed SQLite to the resulting binary and then fetch and fill the database seemed less desirable than just using a MongoDB instance on Atlas.

## Run locally

To run this locally, download Rust, then clone this repository and use

```rust
cargo run
```
