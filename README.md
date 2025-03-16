# Axum test server
This is a simple test server for the Axum web framework. It  listens on port 3000 and responds with a simple message.

## Running the server with hot reload
Running the server with hot reload is simple but requires the `systemfd` and `cargo-watch` crates to be installed. To install these crates, run the following command:
```bash
cargo install cargo-watch systemfd
```
And to run the server with hot reload, run the following command:
```bash
systemfd --no-pid -s http::8080 -- cargo watch -x run
```
## Installing the openAPI generator & generating the code

Installing the openAPI generator with npm:

```bash
npm install @openapitools/openapi-generator-cli -g
```
Or with brew:
```bash
brew install openapi-generator
```

Generating the openAPI code can be done using this generator command
```bash
openapi-generator generate -i ./api/openapi.yaml -g rust-axum -o open_api -p generateAliasAsModel=true
```