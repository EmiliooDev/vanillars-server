# Simple Server Vanilla Rust

This rust program sets up a server that listens on port 8080 and handles incoming HTTP requests.

## Try it yourself

Run the next commands in your the terminal:

```bash
git clone https://github.com/ILoveThatLady/vanillars-server
cd vanillars-server/
```

## Set it up

Install the libraries.

Write in the terminal:

```bash
# path: /vanilars-server
cargo build
```

Run the server:

```bash
cargo run
```

## How to use it

Send some HTTP request (all are sample answers):

1. Open a new terminal.

2. Send a GET request to retrieve a list of all cats:

```bash
curl http://localhost:8080/cat
```

3. Send a POST request to create a new cat with the name "Whiskers":

```bash
curl -X POST -H "Content-Type: application/json" -d '{"name": "Whiskers"}' http://localhost:8080/cat
```

4. Send a GET request to retrieve a specific cat with ID 123:

```bash
curl http://localhost:8080/cat/123
```

5. Send a PUT request to update the name of a cat with ID 123 to "Fluffy":

```bash
curl -X PUT -H "Content-Type: application/json" -d '{"name": "Fluffy"}' http://localhost:8080/cat/123
```

6. Send a DELETE request to delete a cat with ID 123:

```bash
curl -X DELETE http://localhost:8080/cat/123
```

#### Content for educational purposes.
