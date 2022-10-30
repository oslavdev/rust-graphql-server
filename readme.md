# Rust Graphql server

A simple Graphql server built with Rust and Juniper.


### Tech

- Diesel
- Juniper gql

### Setup PostgreSQl

Add *DATABASE_URL* with your settings in .env

```sh
    diesel setup    
    diesel migration generate create_users
```

### Run

```sh
 cargo run
```

### Production Build

```sh
 cargo build
```

