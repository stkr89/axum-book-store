# Axum Bookstore API

This repository contains the source code for a REST API built using the Axum framework in Rust. The API is part of a bookstore application and provides endpoints for managing users and books.

The application uses PostgreSQL for data persistence and includes a setup for environment variables using the `dotenv` crate. Logging is set up using the `log` and `env_logger` crates.

## Endpoints

The endpoints provided by this API are defined in the handlers and include:

- User registration
- User login
- Create a book
- List books

Please refer to the individual handler functions for the exact paths and HTTP methods for these endpoints.

## Running the Application

To run the application, use the command `cargo run` in the terminal. Make sure to set up the required environment variables in a `.env` file in the root of the project.

Please note that this is a POC for building a REST API in Rust using axum.git remote remove origin