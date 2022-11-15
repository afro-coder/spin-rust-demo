## WASM demo in Rust 

This project uses Fermyon's [spin](https://github.com/fermyon/spin)

Steps to get started
- Install the Spin binary(Instructions in the Link above), [cargo and rustup](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Install dependencies
```bash
apt-get install -y curl git libssl-dev pkg-config
```
- Then add wasm as a target
```bash
rustup target add wasm32-wasi
```
- Run `spin build`
- Run `spin up --follow-all` this will show all stdout and stderr

*Note*: There is a Dockerfile but it's currently not optimized and creates a 5GB docker image.

The `spin.toml` is where the Routing is decided for the URL's

Spin has a nice example list

- https://www.fermyon.com/blog/introducing-spin
- https://www.fermyon.com/blog/spin-rest-apis
- https://www.fermyon.com/blog/spin-webhooks
