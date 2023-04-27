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
    - [ ] [PEP 301](https://peps.python.org/pep-0301)
    - [ ] [PEP 503](https://peps.python.org/pep-0503/)
    - [ ] [PEP 592](https://peps.python.org/pep-0592/)
    - [ ] [PEP 629](https://peps.python.org/pep-0629/)
    - [ ] [PEP 658](https://peps.python.org/pep-0658/)
- Server configuration:
    - [ ] `config.toml` 
    - [ ] CLI 
- [ ] Embed package readme to website
- [ ] Add Mirrors (and cache?) to others python indexes.

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
