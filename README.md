# rust_pokemon_cli

Simple Rust CLI to get Pokemon data from https://pokeapi.co/

## Supported API

### Find pokemon by name

```
$ cargo run findByName bulbasaur
```

## Debugging

Add `RUST_LOG=info` env variables to turn on debugging. Example:

```
$ RUST_LOG=info cargo run findByName bulbasaur
```
