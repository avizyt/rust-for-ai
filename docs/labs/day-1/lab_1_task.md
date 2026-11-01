# Day 1 — Rust Fluency: Think Like the Compiler

**Today's mission:** Recover your intuition for ownership, borrowing, and lifetimes by writing and experimenting with Rust code.

We'll approach this like an engineering lab, not a lecture. I'll introduce a concept, show you a small experiment, and give you a task to solve. You'll write the code in Zed, run it, and share your results. I'll review your reasoning and help you identify any gaps.

**Time budget:** 2 hours  
**Difficulty:** Intermediate  
**Language:** Rust  
**Environment:** Zed + Cargo + rust-analyzer

By the end of today's session, you should be more comfortable answering a fundamental question:

> When Rust code passes data between functions, who owns that data, who can access it, and how long is that access valid?

---

# Lab 1 — Set up your workspace (10 minutes)

Let's create a dedicated project for today's experiments.

Open Zed's integrated terminal and run:

```powershell
cargo new rust-fluency-lab
cd rust-fluency-lab
zed .
```

If `zed .` doesn't work from your terminal, open the `rust-fluency-lab` directory through Zed's Open Folder command.

Your project should look like this:

```text
rust-fluency-lab/
├── Cargo.toml
└── src/
    └── main.rs
```

Open `src/main.rs` and replace its contents with:

```rust
fn main() {
    let model = String::from("llama-3.2-1b");

    println!("Selected model: {model}");
}
```

Run the program:

```powershell
cargo run
```

Expected output:

```text
Selected model: llama-3.2-1b
```

Good. You now have a clean environment for experimentation.

**Our rule for today:** Before running code, predict what will happen. Then run it and compare your prediction with the compiler's behavior.

---

# Lab 2 — Ownership: Who owns the data? (20 minutes)

Imagine you're building an inference service.

A request arrives containing a model name:

```rust
let model = String::from("llama-3.2-1b");
```

The string occupies memory. The variable `model` owns the `String` value.

Now suppose another part of your program needs to take ownership of that value.

Consider this code:

```rust
fn main() {
    let model = String::from("llama-3.2-1b");

    let selected_model = model;

    println!("{selected_model}");
}
```

This program compiles.

But why?

## 2.1 The move operation

When you write:

```rust
let selected_model = model;
```

Rust moves ownership of the `String` from `model` to `selected_model`.

After the move, `selected_model` is the variable you use to access that string.

Now modify the program:

```rust
fn main() {
    let model = String::from("llama-3.2-1b");

    let selected_model = model;

    println!("{selected_model}");
    println!("{model}");
}
```

Run:

```powershell
cargo check
```

The compiler should report an error involving a moved value.

### Think about this carefully

A `String` is not just its characters. Conceptually, it holds metadata such as a pointer, length, and capacity, while its character data is stored in a heap allocation.

When you move a `String`, ownership of that allocation transfers to the new variable. Rust prevents you from using the original variable as though it still owns the value.

This avoids problems such as two independently owned `String` values attempting to free the same allocation.

You can still explicitly duplicate the string:

```rust
let selected_model = model.clone();
```

But cloning a `String` generally allocates a new buffer and copies its contents. It is not equivalent to a move.

### Experiment A — Copy versus move

Test this code:

```rust
fn main() {
    let a = 42;
    let b = a;

    println!("{a}");
    println!("{b}");
}
```

Now compare it with the `String` example.

The difference is that `i32` implements the `Copy` trait, whereas `String` does not.

When a value of a `Copy` type is assigned, the value is copied rather than moved.

**Your first task:**

1. Run both examples.
2. Explain why the integer example compiles but the original string example doesn't allow reuse of `model`.
3. Modify the string example so that both variables can be printed successfully.

Don't move on until you can explain the difference in your own words.

---

# Lab 3 — Borrowing: Use data without taking ownership (20 minutes)

Now imagine your inference service needs to inspect a model name without taking ownership of it.

Should it clone the entire string every time?

Usually, no.

Instead, it can borrow the string.

Consider:

```rust
fn inspect_model(model: &String) {
    println!("Inspecting model: {model}");
}

fn main() {
    let model = String::from("llama-3.2-1b");

    inspect_model(&model);

    println!("Still available: {model}");
}
```

Notice the distinction:

```rust
inspect_model(&model);
```

The function receives a reference to the existing string, not ownership of the string itself.

The original variable remains usable after the function returns.

This is called **borrowing**.

## 3.1 Why `&str` is usually better than `&String`

Let's improve the function signature:

```rust
fn inspect_model(model: &str) {
    println!("Inspecting model: {model}");
}
```

The call still works:

```rust
let model = String::from("llama-3.2-1b");
inspect_model(&model);
```

It also works with a string literal:

```rust
inspect_model("qwen-small");
```

Why?

`&str` is a borrowed string slice. It allows the function to work with string data without requiring the caller to provide a `String` specifically.

This makes `&str` a more flexible interface for functions that only need to read text.

In ordinary application code, prefer `&str` over `&String` when you only need to inspect a string.

## 3.2 Mutable borrowing

Suppose we need to update a model name.

```rust
fn rename_model(model: &mut String) {
    model.push_str("-quantized");
}

fn main() {
    let mut model = String::from("llama-3.2-1b");

    rename_model(&mut model);

    println!("{model}");
}
```

Expected output:

```text
llama-3.2-1b-quantized
```

There are three important details:

- `mut model` allows the variable to be borrowed mutably.
- `&mut model` creates a mutable reference.
- `&mut String` tells the function it may modify the borrowed string.

Rust restricts mutable borrowing to help prevent conflicting access to the same data.

**Your second task:**

Write a function named `append_suffix` that:

1. Accepts a mutable reference to a `String`.
2. Appends `"-optimized"` to the string.
3. Does not return the modified string.
4. Allows `main` to print the modified value after the function returns.

Test it with `"llama-3.2-1b"`.

---

# Lab 4 — The borrow checker: Understanding the restrictions (20 minutes)

So far, we've seen ownership transfers and references.

Now let's investigate why Rust restricts certain borrowing patterns.

Consider:

```rust
fn main() {
    let mut requests = vec![10, 20, 30];

    let first = &requests[0];

    requests.push(40);

    println!("{first}");
}
```

Try compiling it.

Depending on the precise code and compiler analysis, you'll encounter a borrowing error: the immutable borrow of an element remains live when the code attempts to mutate the vector.

Why?

A `Vec<T>` can reallocate its backing memory when you push a new element. A reference into that vector must not remain usable if the referenced element's memory could have moved.

Rust rejects the conflicting operations.

The important point is that the restriction isn't arbitrary. It helps prevent references from becoming invalid.

## 4.1 The core borrowing rule

A useful mental model is:

At any given time, you can have:

- Multiple immutable references (`&T`), or
- One mutable reference (`&mut T`).

You cannot have both kinds of live reference to the same data simultaneously in a way that violates Rust's borrowing rules.

Consider another example:

```rust
fn main() {
    let mut score = 100;

    let first = &mut score;

    // Uncomment this:
    // let second = &mut score;

    *first += 10;

    println!("{first}");
}
```

Try uncommenting the second mutable reference.

Rust rejects the conflicting mutable borrows.

Now consider a different arrangement:

```rust
fn main() {
    let mut score = 100;

    {
        let first = &mut score;
        *first += 10;
    }

    let second = &mut score;
    *second += 20;

    println!("{score}");
}
```

This compiles because the first mutable borrow ends before the second begins.

The braces make the scopes explicit. Rust's non-lexical lifetime analysis can also end borrows when they're no longer used, even before the end of their enclosing scope.

**Your third task:**

Create a vector of inference latencies:

```rust
let mut latencies = vec![120, 240, 180];
```

Then:

1. Borrow the first latency immutably.
2. Try adding another latency to the vector.
3. Access the borrowed latency after the mutation.
4. Observe the compiler error.
5. Modify the program so it compiles without cloning the entire vector.

Hint: Think about when the borrowed value is last needed.

---

# Lab 5 — Lifetimes: How long is a reference valid? (20 minutes)

Lifetimes can feel abstract until you connect them to a practical API.

Suppose your application selects whichever model name is longer:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}
```

The function returns a reference to one of its inputs.

The lifetime annotation `'a` expresses a relationship between the input references and the returned reference.

It tells the compiler that the returned reference must not be used beyond the lifetime permitted by the referenced input data.

It does **not** extend the lifetime of either input.

## 5.1 Test the function

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let model_a = String::from("llama-3.2-1b");
    let model_b = String::from("qwen-small");

    let selected = longest(&model_a, &model_b);

    println!("Selected model: {selected}");
}
```

This works because both strings remain alive while the returned reference is used.

Now try returning a reference to a locally created string:

```rust
fn invalid_model_name() -> &str {
    let model = String::from("temporary-model");
    &model
}
```

The compiler rejects this because `model` is dropped when the function returns. Returning a reference to its contents would create a dangling reference.

### The practical lesson

When designing a Rust API, ask:

- Does the function need to own the data?
- Can it borrow existing data?
- Does the returned reference depend on an input reference?
- Will the referenced value remain alive for as long as the caller needs it?

These questions matter when designing parsers, caches, data structures, and APIs that work with model metadata.

**Your fourth task:**

Implement `longest` yourself without looking at the reference implementation.

Then explain why this function needs a lifetime relationship between its inputs and output.

---

# Lab 6 — Mini engineering challenge (20 minutes)

Let's put the concepts together.

Build a small model-name utility.

### Requirements

Implement these functions:

```rust
fn normalize_model_name(model: &str) -> String
```

Returns a normalized, owned model name.

For example:

```text
Input:  "  Llama-3.2-1B  "
Output: "llama-3.2-1b"
```

```rust
fn is_supported_model(model: &str) -> bool
```

Returns `true` for model names beginning with `"llama-"` or `"qwen-"`.

```rust
fn append_optimized_suffix(model: &mut String)
```

Appends `"-optimized"` to the supplied string.

```rust
fn longest_model_name<'a>(a: &'a str, b: &'a str) -> &'a str
```

Returns a reference to the longer model name. In a tie, return the first argument.

### Constraints

- Do not clone strings unnecessarily.
- Use borrowed references for read-only inputs.
- Return owned data when creating a new normalized string.
- Use mutable borrowing when modifying an existing string.
- Avoid `unwrap()` and `expect()` unless you have a specific reason for using them.

Write a `main` function that demonstrates all four operations.

Then run:

```powershell
cargo fmt
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

If Clippy identifies an issue, investigate it instead of suppressing the warning.

---

# Day 1 completion criteria

Before finishing, you should be able to explain the following without reading your notes:

| Question | Expected understanding |
|---|---|
| Why does assigning a `String` to another variable move it? | `String` is not `Copy`; ownership transfers. |
| Why can an `i32` be assigned and reused? | `i32` implements `Copy`. |
| When should you use `&str`? | When a function needs to read string data without owning it. |
| What does `&mut T` allow? | Exclusive mutable access to the borrowed value. |
| Why does Rust reject conflicting borrows? | To prevent unsafe access patterns and invalid references. |
| What does `'a` mean? | It expresses a lifetime relationship; it doesn't extend a value's lifetime. |
| When should you use `.clone()`? | When you genuinely need an independent copy of a value. |

## Let's begin with your first experiment

Don't try to complete all six labs at once. We'll work through them interactively.

**Start with Lab 2.** Put this code in `src/main.rs`:

```rust
fn main() {
    let model = String::from("llama-3.2-1b");

    let selected_model = model;

    println!("{selected_model}");
    println!("{model}");
}
```

Run `cargo check` in Zed.

**Tell me two things:**

1. What error does the compiler report?
2. In your own words, why do you think Rust rejects this code?

I'll review your explanation before we move to borrowing.
