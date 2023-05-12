# Nest 🪺

Python Package Index in `rust` 🦀

## Getting Started

To run the server locally, first run `surrealdb` instance with `docker-compose up -d`.

Then you can initialize the database with `python init.py`

Then run the server with `cargo run`.

> You can test the server with the python's test module in the folder `my-module`.

## Roadmap

### Core features 
- Simple Index Interface
    - [ ] [PEP 503 - Simple Repository API](https://peps.python.org/pep-0503/)
    - [ ] [PEP 592 - Adding “Yank” Support to the Simple API](https://peps.python.org/pep-0592/)
    - [x] [PEP 629 - Versioning PyPI’s Simple API](https://peps.python.org/pep-0629/)
    - [ ] [PEP 643 – Metadata for Package Source Distributions](https://peps.python.org/pep-0643/)
    - [ ] [PEP 658 - PEP 658 – Serve Distribution Metadata in the Simple Repository API](https://peps.python.org/pep-0658/)
    - [ ] [PEP 691 -  JSON-based Simple API for Python Package Indexes](https://peps.python.org/pep-0691/)
- Server configuration:
    - [x] `config.toml`  
    - [ ] CLI (Migrations, Start server, ...)
- [ ] Embed package readme to website
- [ ] Add Mirrors (and cache?) to others python indexes.
- [ ] Search package

### Additionals features
- [ ] Packages statistics
- [ ] Enable multi project (multi workspaces / indexes)

### Some ideas
- Serving front-end with `Leptos`. (More Rust)
- Use `surreal` graph relation instead of record links when it makes sense.
- Strong documentation for deployement with local file storage and how to scale with distributed object storage.

## Useful links
- https://surrealdb.com/docs/
- https://docs.rs/axum/latest/axum/
