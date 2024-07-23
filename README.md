# JWT Authentication Service

This repository contains a Rust-based web service that utilizes JSON Web Tokens (JWT) for authentication. The service is built using the Axum framework for creating web applications with Rust. It demonstrates a simple login system where users can log in to receive a JWT token and then use this token to access protected routes.

## Features

- **User Authentication**: Implements a basic user authentication flow where users can log in using their credentials to receive a JWT token.
- **Protected Routes**: Includes examples of protected routes that require a valid JWT token to access.
- **JWT Token Management**: Utilizes the `jsonwebtoken` crate for encoding and decoding JWT tokens, ensuring secure token management.

## Dependencies

The project relies on several key Rust crates:

- `tokio`: An asynchronous runtime for Rust, used as the foundation for running the web server and handling asynchronous operations.
- `jsonwebtoken`: A crate for encoding and decoding JWT tokens, providing the core functionality for token management.
- `axum`: A web application framework built on top of Hyper, providing routing and request handling.
- `serde` and `serde_json`: Serialization and deserialization libraries for converting Rust structs to JSON and vice versa.
- `chrono`: A date and time library for Rust, used for handling time-based operations in JWT tokens.

## Project Structure

- `src/main.rs`: The entry point of the application, setting up the web server and routes.
- `src/controler.rs`: Contains the route handlers, including the login handler and a handler for retrieving protected information.
- `src/model.rs`: Defines the data models used in the application, including structures for login information and responses.

## Getting Started

To run the project, ensure you have Rust and Cargo installed. Then, navigate to the project directory and run:

```sh
cargo run
```

This will start the web server on `http://localhost:3000`. You can then use tools like `curl` or Postman or Hoppscotch to interact with the service.

### Routes

- `POST /login`: Logs in a user and returns a JWT token.
- `GET /get_info`: Returns protected information that requires a valid JWT token.
