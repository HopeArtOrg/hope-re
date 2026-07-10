---
name: tauri-command
description: Create a new Rust Tauri v2 command following Hope:RE error handling and module conventions
---

## Tauri Command Pattern

When creating a new Tauri command in Hope:RE, follow these conventions:

### Command Function
```rust
#[tauri::command]
pub async fn my_command(
    app: tauri::AppHandle,
    input_param: String,
) -> Result<MyOutput, String> {
    let result = some_operation(&input_param)
        .map_err(|e| format!("Failed to do operation: {}", e))?;

    let _ = app.emit("my-event", &result);

    log::info!("Operation completed successfully");
    Ok(result)
}
```

### Struct Definitions
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MyOutput {
    pub field: String,
    pub optional_field: Option<u32>,
}
```

### Registration in `lib.rs`
Add the command to the `tauri::Builder` invoke handler:
```rust
.invoke_handler(tauri::generate_handler![
    commands::my_command,
])
```

### Rules
- Always return `Result<T, String>` -- no custom error enums
- Use `.map_err(|e| format!("Descriptive message: {}", e))?` for error propagation
- Use `log::info!`, `log::error!` for logging (Tauri log plugin)
- Use `let _ = app.emit(...)` for non-critical event emissions
- Derive `Debug, Clone, serde::Serialize, serde::Deserialize` on data transfer structs
- Use `Option<T>` for optional fields
- Use `#[cfg(...)]` for platform-specific code
- Files are snake_case, types are PascalCase, constants are UPPER_SNAKE_CASE
- No comments in code -- self-documenting through clear naming
- No emojis anywhere in the codebase
