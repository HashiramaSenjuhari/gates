# Gates

**Gates** is a lightweight and efficient web framework in Rust designed for handling HTTP requests with built-in gatekeeping middleware.

## Features

- Supports **GET, POST, PUT, DELETE, SSE** endpoints
- **Gatekeeping middleware** for request handling
- **Response handling** with status codes, messages, and redirections

## Installation

Add `gates` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
gates = "0.1.0-beta"
```

## Usage

### Defining Routes

```rust
// POST request handler
#[gates(post = "/billionaire", gatekeeper = billionairehari)]
fn basicb(b: &GatesRequest) -> GateKeeperResponse {
    println!("{}", b.body);
    GateResponse::response(GatesResponse::new().status(200).message(""))
}
```

### Middleware Example

Middleware functions can intercept requests before they reach handlers.

```rust
fn billionairehari(billionaire: &GatesRequest) -> GateKeeperResponse {
    let path = billionaire.path;
    GateKeeper::next()
    GateKeeper::redirect(307, "/billionaires")
    GateKeeper::response(
        GatesResponse::new()
            .content_type("")
            .status(404)
            .message("billionairegreatharigreat"),
    )
}
```

### Supported Endpoints

```rust
// GET
#[gates(get = "/billionaire", gatekeeper = billionairehari)]
fn basicg(b: &GatesRequest) -> GateKeeperResponse {
    println!("{}", b.body);
    GateResponse::response(GatesResponse::new().status(200).message(""))
}

// DELETE
#[gates(delete = "/billionaire", gatekeeper = billionairehari)]
fn basicd(b: &GatesRequest) -> GateKeeperResponse {
    println!("{}", b.body);
    GateResponse::response(GatesResponse::new().status(200).message(""))
}

// PUT
#[gates(put = "/billionaire", gatekeeper = billionairehari)]
fn basicp(b: &GatesRequest) -> GateKeeperResponse {
    println!("{}", b.body);
    GateResponse::response(GatesResponse::new().status(200).message(""))
}
```

### SSE (Server-Sent Events)

```rust
#[gates(sse = "/billionaire", gatekeeper = billionairehari)]
fn basics(b: &GatesRequest) -> GateKeeperResponse {
    println!("{}", b.body);
    GateResponse::response(GatesResponse::new().status(200).message(""))
}
```

## Running the Server

Compile and run the project with:

```sh
cargo run
```

## License

This project is licensed under the MIT License.
