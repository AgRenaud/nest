# Nest 🪺

Python Package Index in `rust` 🦀

## Getting Started

To run the server locally, first run `postgres` instance with `docker-compose up -d`.

Then you can initialize the database by running sqlx migrations.

```sh
sqlx database create -D postgres://nest-user:nest-secret@localhost:5432/postgres
sqlx migrate run -D postgres://nest-user:nest-secret@localhost:5432/postgres
```

Then run the server with `cargo run`.

You can install [`Bunyan`](https://crates.io/crates/bunyan) to get human readable logs  `cargo run | bunyan`

> You can test the server with the python's test module in the folder `my-module`.

## Roadmap

### Core features

> `twine upload`, `poetry publish` and other similar commands should work.
> There is no user management for the moment.
> Simple `pip install --extra-index-url ...` is working too.

- Simple Index Interface
  - [x] [PEP 503 - Simple Repository API](https://peps.python.org/pep-0503/)
  - ~~[ ] [PEP 592 - Adding “Yank” Support to the Simple API](https://peps.python.org/pep-0592/)~~
  - [x] [PEP 629 - Versioning PyPI’s Simple API](https://peps.python.org/pep-0629/)
  - [ ] [PEP 643 – Metadata for Package Source Distributions](https://peps.python.org/pep-0643/)
  - [ ] [PEP 658 - PEP 658 – Serve Distribution Metadata in the Simple Repository API](https://peps.python.org/pep-0658/)
  - [ ] [PEP 691 - JSON-based Simple API for Python Package Indexes](https://peps.python.org/pep-0691/)
- Server configuration:
  - [x] `config.toml`
  - ~~[ ] CLI (Migrations, Start server, ...)~~
- Manage users:
  - [ ] Admin page
  - [x] User basic auth
  - [x] User sign up
  - [ ] User Roles (Contributor & admin)
- [ ] Embed package readme to website
- [ ] Add Mirrors (and cache?) to others python indexes.
- [ ] Search package

> Strikethrough text won't be implemented for first release.

## Useful links

- <https://docs.rs/axum/latest/axum/>

