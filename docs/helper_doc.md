# Set up Zed for a productive Rust development workflow

Since you're using Windows and Zed, I'd configure your environment around **Cargo, rust-analyzer, compiler feedback, automated tests, and fast keyboard-driven workflows**.

The objective is to make your daily Rust practice almost frictionless: open a project, write code, inspect compiler diagnostics, run tests, and repeat without constantly switching tools.

Zed already supports Rust through `rust-analyzer`, including code completion, diagnostics, and navigation. We'll configure those capabilities instead of adding unnecessary extensions. [Zed](https://zed.dev/languages/rust?utm_source=chatgpt.com)

## 1. Install and verify the Rust toolchain

Open Zed's integrated terminal with **Ctrl + `** or use PowerShell separately.

Run:

```powershell
rustc --version
cargo --version
rustup --version
rustup component list --installed
```

You should have these components installed:

- `rust-analyzer` — Rust language-server support.
- `rustfmt` — automatic code formatting.
- `clippy` — additional compiler-assisted linting.

If any are missing, install them:

```powershell
rustup component add rust-analyzer rustfmt clippy
```

Update the stable toolchain if necessary:

```powershell
rustup update stable
rustup default stable
```

Verify that Cargo can create and compile a project:

```powershell
cargo new rust-zed-lab
cd rust-zed-lab
cargo check
cargo test
```

**Expected result:** Both commands should complete successfully. You should see one passing test from Cargo's generated template.

If a command isn't recognized, check whether the Rust toolchain's `bin` directory is available on your Windows `PATH`, then restart Zed so it inherits the updated environment.

## 2. Configure rust-analyzer and editor intelligence

Open Zed's command palette with **Ctrl + Shift + P**, search for `zed: open settings`, and open `settings.json`.

Add the following configuration, merging it into your existing settings rather than replacing them.

```json
{
  "format_on_save": "on",
  "ensure_final_newline_on_save": true,
  "tab_size": 4,
  "hard_tabs": false,
  "inlay_hints": {
    "enabled": true,
    "show_type_hints": true,
    "show_parameter_hints": true
  },
  "languages": {
    "Rust": {
      "format_on_save": "on"
    }
  },
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "check": {
          "command": "clippy"
        },
        "checkOnSave": true,
        "inlayHints": {
          "bindingModeHints": {
            "enable": true
          },
          "closureReturnTypeHints": {
            "enable": "always"
          },
          "lifetimeElisionHints": {
            "enable": "skip_trivial",
            "useParameterNames": true
          },
          "maxLength": 80
        }
      }
    }
  }
}
```

Zed's Rust integration uses `rust-analyzer`, and the editor supports Rust-specific language-server settings, including additional inlay hints. See the [official Rust configuration documentation](https://github.com/zed-industries/zed/blob/main/docs/src/languages/rust.md). [Zed](https://zed.dev/languages/rust?utm_source=chatgpt.com)

### Why these settings matter

**1. Inlay hints**

Consider:

```rust
fn calculate_tokens(prompt: &str) -> usize {
    prompt.split_whitespace().count()
}

fn main() {
    let prompt = "Explain attention";
    let count = calculate_tokens(prompt);
    println!("{count}");
}
```

Inlay hints can help you inspect inferred types and parameter names without explicitly annotating every variable.

This is particularly useful during your Rust refresher because you'll see how the compiler infers types while you write code.

**2. Clippy on save**

Setting the Rust analyzer's check command to `clippy` requests additional linting when your code is checked. This can help you identify questionable patterns early.

You can still run Clippy manually from the terminal to verify that the entire project passes.

**3. Lifetime hints**

The lifetime hint configuration helps expose lifetime information where rust-analyzer considers it useful. This will be valuable during Day 1 of your training.

**4. Automatic formatting**

Rustfmt keeps your code consistently formatted. That means you can focus on understanding ownership, API design, and correctness rather than manually aligning code.

### Verify the setup

Create a Rust file and deliberately introduce a type error:

```rust
fn main() {
    let count: u32 = "hello";
    println!("{count}");
}
```

Zed should display a diagnostic identifying the type mismatch. Restore the code afterward.

If diagnostics do not appear, check that:
- You opened the Cargo project root, not just an individual `.rs` file.
- `rust-analyzer` is installed and available to Zed.
- The language server started successfully.

---

## 3. Create a reusable Rust task menu

This is the most important productivity improvement for your week-long journey.

Instead of typing `cargo check`, `cargo test`, and `cargo clippy` repeatedly, define them once in Zed.

Zed supports project-specific task definitions in `.zed/tasks.json` and can execute them through its task picker. See the [official Tasks documentation](https://zed.dev/docs/tasks). [Zed](https://zed.dev/docs/tasks?utm_source=chatgpt.com)

### Step 1: Create the task file

Inside your Rust project, create:

```text
rust-zed-lab/
├── Cargo.toml
├── Cargo.lock
├── src/
└── .zed/
    └── tasks.json
```

If `.zed` doesn't exist, create the directory.

### Step 2: Add these tasks

Paste the following into `.zed/tasks.json`:

```json
[
  {
    "label": "Rust: Run",
    "command": "cargo run",
    "save": "all",
    "use_new_terminal": false,
    "allow_concurrent_runs": false
  },
  {
    "label": "Rust: Run Release",
    "command": "cargo run --release",
    "save": "all",
    "use_new_terminal": false
  },
  {
    "label": "Rust: Check",
    "command": "cargo check",
    "save": "all",
    "use_new_terminal": false
  },
  {
    "label": "Rust: Test All",
    "command": "cargo test",
    "save": "all",
    "use_new_terminal": false
  },
  {
    "label": "Rust: Test Verbose",
    "command": "cargo test -- --nocapture",
    "save": "all",
    "use_new_terminal": false
  },
  {
    "label": "Rust: Clippy",
    "command": "cargo clippy --all-targets --all-features -- -D warnings",
    "save": "all",
    "use_new_terminal": false
  },
  {
    "label": "Rust: Format",
    "command": "cargo fmt",
    "save": "all",
    "use_new_terminal": false
  },
  {
    "label": "Rust: Format Check",
    "command": "cargo fmt --check",
    "save": "all",
    "use_new_terminal": false
  },
  {
    "label": "Rust: Build Release",
    "command": "cargo build --release",
    "save": "all",
    "use_new_terminal": false
  },
  {
    "label": "Rust: Full Quality Check",
    "command": "cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test",
    "save": "all",
    "use_new_terminal": false
  }
]
```

These commands are appropriate for a standard Cargo project. The full quality check uses shell command chaining; if your configured shell does not support `&&`, run the individual tasks or adapt the command to your shell.

**Important:** `Rust: Run` works for projects with a binary target. If you create a library-only project, use `Rust: Check` and `Rust: Test All` instead.

### Step 3: Run your first task

1. Open the project in Zed.
2. Press **Ctrl + Shift + R**.
3. Search for `Rust: Check`.
4. Run the task.
5. Inspect the terminal output.

Repeat with `Rust: Test All`.

You can also search for `task: spawn` in the command palette to open the task picker.

### Step 4: Use task rerun

After executing a task, use the command palette to search for `task: rerun`.

This reruns the most recently spawned task, making it useful when you're fixing a compiler error or repeatedly testing the same function.

**Recommended habit:** Keep `Rust: Check` and `Rust: Test All` as your two most frequently used tasks. Use Clippy and the full quality check before committing larger changes.

---

## 4. Add keyboard shortcuts for Rust development

Zed lets you customize keybindings in `keymap.json`. On Windows, open the command palette and search for **`zed: open keymap file`**. The file is typically located at `%APPDATA%\Zed\keymap.json`. [Zed](https://zed.dev/docs/key-bindings?utm_source=chatgpt.com)

I'd start with these shortcuts:

| Action | Shortcut on Windows | Why you'll use it |
|---|---|---|
| Command palette | `Ctrl + Shift + P` | Access any Zed command |
| Find file | `Ctrl + P` | Open `main.rs`, `Cargo.toml`, etc. |
| Find in project | `Ctrl + Shift + F` | Search across your Rust codebase |
| Go to symbol | `Ctrl + Shift + O` | Find functions and types |
| Toggle terminal | `Ctrl + `` | Access the terminal |
| Spawn task | `Ctrl + Shift + R` | Run Cargo tasks |
| Debug panel | `Ctrl + Shift + D` | Open the debugging interface |
| Go to definition | `F12` | Navigate to a function or type definition |
| Format document | Search for `editor: format` in the command palette | Format Rust code |
| Open keymap | Search for `zed: open keymap file` | Customize shortcuts |

The standard shortcuts are documented in [Zed's Getting Started guide](https://zed.dev/docs/getting-started) and [keybindings reference](https://zed.dev/docs/key-bindings). [Zed](https://zed.dev/docs/getting-started?utm_source=chatgpt.com)

### My recommended custom bindings

Add the following entries to your existing `keymap.json` array:

```json
[
  {
    "context": "Editor",
    "bindings": {
      "ctrl-alt-r": "task::Spawn",
      "ctrl-alt-x": "task::Rerun"
    }
  }
]
```

These give you:
- **Ctrl + Alt + R:** Open the task picker.
- **Ctrl + Alt + X:** Rerun the previous task.

Keep your existing keybindings and merge this object into the existing array. Don't replace the whole file if you already have custom bindings.

If a shortcut doesn't work, open the keymap editor and search for the corresponding action to confirm its current name and any conflicts.

### The workflow I recommend

During your daily Rust practice:

1. Write your implementation.
2. Run `Ctrl + Alt + R` and select `Rust: Check`.
3. Fix compiler diagnostics.
4. Run `Ctrl + Alt + X` to rerun the previous task when appropriate.
5. Execute `Rust: Test All` when your implementation is ready.

This creates a tight feedback loop without forcing you to leave the editor.

---

## 5. Make the terminal work for you

For your workflow, use Zed's integrated terminal for Cargo commands and keep the editor focused on source code.

Open it using `Ctrl + ``.

Useful commands:

```powershell
# Check compilation without producing an executable
cargo check

# Compile and execute
cargo run

# Execute tests
cargo test

# Run tests and show their printed output
cargo test -- --nocapture

# Apply formatting
cargo fmt

# Run Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build an optimized executable
cargo build --release
```

### A practical terminal layout

Use the editor for `src/main.rs` and your other source files, with the terminal panel available when you need compiler output.

You don't need a complicated terminal setup yet. A single terminal for Cargo tasks is sufficient for most of the week's exercises.

If you later work on an async service, keep one terminal for running the server and another for sending requests or running tests.

Zed's integrated terminal and task system are designed to support this workflow. [Running and testing in Zed](https://zed.dev/docs/running-testing). [Zed](https://zed.dev/docs/running-testing?utm_source=chatgpt.com)

---

## 6. Enable useful Rust navigation and code inspection

A significant part of Rust development is understanding code you didn't write, especially when working with crates and large codebases.

Make these actions part of your workflow.

### Go to definition

Place the cursor on a function or type and press `F12`.

For example:

```rust
fn main() {
    let values = vec![10, 20, 30];
    let total: i32 = values.iter().sum();

    println!("{total}");
}
```

Navigate to `sum` or `iter` to explore the standard library's definitions and documentation where supported.

### Find references

When you want to understand where a function is used, use Zed's **Find References** action from the command palette.

This is particularly useful when modifying an existing Rust module because you can identify which callers might be affected by your changes.

### Rename symbols

Use Zed's rename-symbol action when refactoring a function, struct, or variable. Rust-aware renaming is safer than replacing text across the entire project.

### Inspect types

Hover over an expression or use inlay hints to inspect inferred types.

For example:

```rust
let values = vec![1, 2, 3];
let first = values.first();
```

The second variable has type `Option<&i32>`, not `i32`.

Being able to inspect this immediately helps reinforce Rust's type system as you work.

---

## 7. Configure debugging—but understand the current limitation

For Rust debugging, there is an important distinction between compiler diagnostics and interactive debugging.

- **Compiler diagnostics:** Provided through rust-analyzer.
- **Tests and execution:** Available through Cargo tasks.
- **Interactive debugging:** Depends on a supported debugger adapter and the current state of Zed's Rust debugging integration.

Zed's Rust language page currently describes native Rust debugging support as limited, so don't make a fully integrated debugger a prerequisite for starting your learning plan. Check the latest [Zed Rust documentation](https://zed.dev/languages/rust) for the current status. [Zed](https://zed.dev/languages/rust?utm_source=chatgpt.com)

In the meantime, you can debug effectively using:

```rust
dbg!(&values);
```

And:

```rust
println!("Current value: {value:?}");
```

For a more structured approach, use tests to isolate the behavior you're investigating:

```rust
#[test]
fn calculates_total_correctly() {
    let values = [10, 20, 30];
    let total: i32 = values.iter().sum();

    assert_eq!(total, 60);
}
```

Run the test with:

```powershell
cargo test calculates_total_correctly
```

This is particularly useful when learning ownership, iterators, and error handling because you can reproduce a problem in a small test instead of debugging the entire application.

---

## 8. Install only one optional extension

Zed already has built-in Rust support, so you don't need a collection of extensions to get started.

One optional extension worth considering is **CargoTom**, which provides assistance with `Cargo.toml` dependencies, versions, and features.

You can find it through Zed's [Rust extensions page](https://zed.dev/languages/rust). [Zed](https://zed.dev/languages/rust?utm_source=chatgpt.com)

I would avoid spending time customizing themes, installing unrelated extensions, or building an elaborate plugin setup before the week starts.

Your main productivity gains will come from rust-analyzer, Cargo tasks, navigation, and consistent testing.

---

# 9. Your final Zed workflow

Here is the workflow I'd use throughout your seven-day Rust sprint.

| Step | Action | Tool |
|---|---|---|
| 1 | Open the project and select a source file | `Ctrl + P` |
| 2 | Write the implementation | Editor + rust-analyzer |
| 3 | Inspect types and diagnostics | Inlay hints + compiler feedback |
| 4 | Check compilation | `Rust: Check` task |
| 5 | Correct errors and warnings | Editor + Clippy |
| 6 | Test behavior | `Rust: Test All` task |
| 7 | Format code | `Rust: Format` task |
| 8 | Verify the final change | `Rust: Full Quality Check` task |

### Before starting Day 1

Complete this checklist:

- [ ] `rustc`, `cargo`, and `rustup` work in the terminal.
- [ ] `rust-analyzer`, `rustfmt`, and Clippy are installed.
- [ ] Zed displays Rust diagnostics.
- [ ] Inlay hints are visible.
- [ ] The project-specific `.zed/tasks.json` file is configured.
- [ ] `Rust: Check` runs successfully.
- [ ] `Rust: Test All` runs successfully.
- [ ] Custom task shortcuts work.
- [ ] You can navigate to a definition and find references.

## One final recommendation

**Keep your editor configuration stable throughout the seven days.** Resist the temptation to spend your practice sessions continuously adjusting Zed.

Once this setup works, the next step is to create your actual `rust-workload-inspector` project and begin Day 1. Use the compiler as a learning instrument: make predictions about ownership and borrowing, compile your code, examine the diagnostics, and explain the result before moving forward.

That habit will improve your Rust fluency far more than memorizing shortcuts alone.
