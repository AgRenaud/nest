# Nest 🪺

Python Package Index in `rust` 🦀

## Getting Started

To run the server locally, first run `postgres` instance with `docker-compose up db -d`.

Then you can initialize the database by running sqlx migrations.

```sh
sqlx database create -D postgres://nest-user:nest-secret@localhost:5432/nest
sqlx migrate run -D postgres://nest-user:nest-secret@localhost:5432/nest
```

Then run the server with `cargo run`.

You can install [`Bunyan`](https://crates.io/crates/bunyan) to get human readable logs  `cargo run | bunyan`

> You can test the server with the python's test module in the folder `my-module`.

To upload a package, you will first need an account. You can create one by filling the sign up form at `http://localhost:5037` (default value).

![](./docs/nest-screenshots.png)

Then you should be able to upload a package with the current command:

```sh
twine upload package.whl --repository-url http://127.0.0.1:5037/simple
```

and install the package with `pip`

```sh
pip install --index-url http://localhost:5037/simple package
```

## Roadmap

### Core features

- Simple Index Interface
  - [x] [PEP 503 - Simple Repository API](https://peps.python.org/pep-0503/)
  - [x] [PEP 629 - Versioning PyPI’s Simple API](https://peps.python.org/pep-0629/)
  - [ ] [PEP 643 – Metadata for Package Source Distributions](https://peps.python.org/pep-0643/)
  - [ ] [PEP 658 - PEP 658 – Serve Distribution Metadata in the Simple Repository API](https://peps.python.org/pep-0658/)
  - [ ] [PEP 691 - JSON-based Simple API for Python Package Indexes](https://peps.python.org/pep-0691/)
- Server configuration:
  - [x] `config.toml`
- Manage users:
  - [ ] Admin page
  - [x] User basic auth
  - [x] User sign up
  - [x] User login / User session
  - [ ] User Roles (Contributor & admin)
- [x] Embed package readme to website (`markdown` file only.)
- [ ] Add Mirrors (and cache?) to others python indexes.
- [x] Search package

### Ideas

- Rely on [uv](https://github.com/astral-sh/uv) for PEP implementations such as `project`, `metadata`, etc.

## Useful Resources

Here is a list of the resources I heavily relied on to develop this project.

- <https://docs.rs/axum/latest/axum/>
- <https://www.zero2prod.com/>
