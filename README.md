## About
This is authentication service (api) , Design base on hexaglonal architecture ,
Base on Mobile number and OTP (One time password)

## Features

- Login user (Mobile number , OTP)
- Resend OTP
- Authorization to refresh access token 
- Verify access token

## Stacks

- [Actix] - The Web framework opensource create in Rust 
- [Mongodb] - The database of this service , it's NoSQL
- [Docker] - The container services
- [Redis] - Store logs in memories

## Installation

This service create by Rust , If not rust compiler . Then install compiler first.
[Rust](https://www.rust-lang.org/tools/install)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

And install Mongodb for database service [Mongodb](https://www.mongodb.com/docs/manual/installation/)

Install Redis for logger store [Redis](https://redis.io/docs/getting-started/installation/)

## Development

Open your projects in favorite terminal and go to directory

```sh
cd authentication_service
```

Download and build dependencies

```sh
cargo build
```

Start service

```sh
cargo run
```

Build for production

```sh
cargo build --release
```

## Testings

Run integrations testing

```sh
cargo test
```

## Docker setup and run


## Issue

When redis cannot add LPUSH , You can check status redis service and

```sh
config set stop-writes-on-bgsave-error no
```

## Author

@Pinyoo Thotaboot