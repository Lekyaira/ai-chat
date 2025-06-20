## Relevant Files

- `src/main.rs` - Application entry point and REPL startup.
- `src/repl.rs` - Handles multi-line user input and commands.
- `src/ollama_client.rs` - Manages communication with the Ollama model server.
- `src/config.rs` - Loads environment variables using `dotenvy`.
- `src/history.rs` - Stores in-memory chat history for the session.
- `src/error.rs` - Error types and utilities for printing to `stderr`.

### Notes

- Unit tests should typically be placed alongside the code files they are testing (e.g., `repl.rs` and `repl.test.rs` in the same directory).

## Tasks

- [ ] 1.0 Project Setup and Dependencies
- [ ] 2.0 Implement REPL with multi-line input
- [ ] 3.0 Connect to Ollama and stream responses
- [ ] 4.0 Load configuration from environment variables
- [ ] 5.0 Error handling and in-memory chat history
