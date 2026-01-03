# FerrisDB

This is a project made for practicing and learning Rust language. It's a key-value redis like database.

## How to run

Execute this command to run the server:

```shell
cargo run --bin ferris-db
```

In another terminal execute this command for the CLI:

```shell
cargo run --bin ferris-cli
```

## How to use the CLI


| Command | Description | Example |
| ------- | ----------- | ------- |
| SET | Set a new record (TTL is optional) | `SET key value 100` |
| GET | Get value using the key | `GET key` |
| DELETE | Delete a record using the key | `DELETE key` |
| RENAME | Rename a key | `RENAME key new_key` |
| TTL | Get the remaining TTL in seconds | `TTL key` |
| EXPIRE | Update the TTL | `EXPIRE key 200` |
| KEYS | Get all keys | `KEYS` |
| COUNT | Count the records | `COUNT` |
| CLEAR | Remove all records | `CLEAR` |