# Axum API

Technologies:

- Language: Rust
- Database: SQLite
- ORM: Diesel

### Start in dev mode with `cargo-watch`

First install the binary

```sh
cargo install cargo-watch
```

Start the project

```sh
cargo watch -q -c -w ./src -x run
```

If you have just installed

```sh
just run-dev-mode
```

## Interacting w/ API

GET All request

```sh
curl \
--header "Content-Type: application/json" \
--request GET \
http://localhost:5050/todos
```

POST One request

```sh
curl  \
--header "Content-Type: application/json" \
--request POST \
--data '{"title":"todo from curl","description":"curl POST request"}' \
http://localhost:5050/todos

```

## Benchmarks

Using the [rewrk](https://github.com/ChillFish8/rewrk) HTTP load benchmarker

```sh
rewrk -c 256 -d 60s -h http://localhost:5050/todos --pct
```

