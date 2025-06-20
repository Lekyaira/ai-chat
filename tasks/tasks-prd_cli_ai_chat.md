## Relevant Files

- `src/main.rs` - Entry point starting the REPL and orchestrating modules.
- `src/config.rs` - Loads environment variables via `dotenvy`.
- `src/ollama_client.rs` - Communicates with the Ollama server and streams responses.
- `src/repl.rs` - Implements the interactive REPL loop with history.
- `src/config_test.rs` - Unit tests for `config.rs`.
- `src/ollama_client_test.rs` - Unit tests for `ollama_client.rs`.
- `src/repl_test.rs` - Integration tests for the REPL behaviour.

### Notes

- Unit tests should typically be placed alongside the code files they are testing.

## Tasks

- [ ] 1.0 Set up project dependencies and configuration
  - [ ] 1.1 Verify Cargo dependencies for `dotenvy`, `ollama-rs`, `tokio`, `rustyline`, and `ansi_term`
  - [ ] 1.2 Add `.env.example` documenting `OLLAMA_HOST` and `OLLAMA_PORT`
- [ ] 2.0 Environment configuration module
  - [ ] 2.1 Create `Config` struct to load variables using `dotenvy`
  - [ ] 2.2 Print a helpful message if variables are missing
- [ ] 3.0 Ollama client implementation
  - [ ] 3.1 Write async function to send prompts and stream responses via `ollama-rs`
  - [ ] 3.2 Handle network errors and print to `stderr`
- [ ] 4.0 Interactive REPL
  - [ ] 4.1 Build multi-line input loop using `rustyline` or similar
  - [ ] 4.2 Store chat history in memory only
  - [ ] 4.3 Provide `:exit` and `:help` commands and initial instructions
- [ ] 5.0 Stream and display model output
  - [ ] 5.1 Render tokens as they arrive
  - [ ] 5.2 Colorize error messages using `ansi_term`
- [ ] 6.0 Testing suite
  - [ ] 6.1 Unit tests for `Config` behaviour
  - [ ] 6.2 Unit tests for the Ollama client streaming logic
  - [ ] 6.3 Integration test covering a simple REPL session
- [ ] 7.0 Documentation
  - [ ] 7.1 Explain usage and environment setup in `README.md`
