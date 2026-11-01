# Rust For AI
## Rust Engineering Fluency
### A 7-Day Hands-On Course for Building Real Software

**Duration:** 7 days  
**Daily commitment:** 2 hours  
**Total learning time:** 14 hours  
**Level:** Intermediate Rust  
**Primary tools:** Rust, Cargo, Zed Editor  
**Focus:** Practical programming, systems engineering, and AI infrastructure

---

## Table of Contents

- [1. Overview](#1-overview)
- [2. Who Is This Course For?](#2-who-is-this-course-for)
- [3. Learning Objectives](#3-learning-objectives)
- [4. Prerequisites](#4-prerequisites)
- [5. Development Environment](#5-development-environment)
- [6. How the Course Works](#6-how-the-course-works)
- [7. Seven-Day Roadmap](#7-seven-day-roadmap)
- [8. Main Project](#8-main-project-rust-ai-workload-inspector)
- [9. Development Workflow](#9-development-workflow)
- [10. Engineering Principles](#10-engineering-principles)
- [11. Progress Tracking](#11-progress-tracking)
- [12. Recommended Resources](#12-recommended-resources)
- [13. Definition of Completion](#13-definition-of-completion)
- [14. What's Next?](#14-whats-next)

---

## 1. Overview

Rust is a systems programming language designed to provide memory safety, performance, and concurrency without requiring a garbage collector.

However, knowing Rust syntax is different from being fluent enough to use Rust productively in real projects.

Developers often encounter problems such as:

- Forgetting ownership and borrowing rules.
- Struggling with lifetime annotations.
- Being uncertain about when to use references, clones, or owned values.
- Overcomplicating error handling.
- Writing imperative code when iterators would be more appropriate.
- Designing APIs that are difficult to extend.
- Finding concurrency and asynchronous programming confusing.
- Spending too much time looking up syntax instead of solving engineering problems.

This course addresses these problems through seven days of deliberate practice.

Instead of treating Rust as a collection of language features to memorize, we will use it to build a small software system and progressively improve its design.

The course emphasizes three complementary abilities:

1. **Language fluency:** Understand and apply Rust's core abstractions.
2. **Engineering fluency:** Organize, test, debug, and maintain Rust code.
3. **Systems thinking:** Understand how ownership, concurrency, performance, and resource management affect software architecture.

The objective is not to master every part of Rust in one week.

The objective is to become comfortable enough with the language to build independently, recognize your knowledge gaps, and learn advanced concepts through real projects.

---

## 2. Who Is This Course For?

This course is intended for developers who already understand basic programming and have some experience writing Rust.

It is particularly relevant to developers interested in:

- AI infrastructure and inference systems.
- High-performance data processing.
- Model-serving components.
- Backend services.
- Developer tools and command-line applications.
- Networking and asynchronous systems.
- Python–Rust interoperability.
- GPU and systems programming.

You do not need previous experience with AI infrastructure to complete the exercises.

The examples use AI workloads to make the exercises realistic, but the concepts apply to general-purpose Rust development.

### What this course is not

This is not a beginner's introduction to programming.

We will not spend significant time on variables, loops, basic conditionals, or elementary data structures.

We will also not attempt to cover every advanced Rust topic, such as unsafe Rust, advanced macro development, pinning, or complex asynchronous runtime internals.

Those topics can be explored after establishing a stronger foundation.

---

## 3. Learning Objectives

By the end of the course, you should be able to:

### Language fundamentals

- Explain ownership, borrowing, and move semantics.
- Distinguish `String` from `&str`.
- Use mutable and immutable references appropriately.
- Understand when lifetime annotations are necessary.
- Design types with structs and enums.
- Use pattern matching effectively.
- Apply traits, generics, and trait bounds.
- Work with `Option<T>` and `Result<T, E>`.
- Write idiomatic iterator-based transformations.

### Software engineering

- Organize code into modules.
- Define clear function and type interfaces.
- Manage dependencies using Cargo.
- Write unit and integration tests.
- Interpret compiler errors and Clippy diagnostics.
- Separate application logic from command-line interfaces.
- Handle malformed input and operational failures.
- Measure performance before optimizing.

### Concurrency and asynchronous programming

- Understand Rust's threading model.
- Use threads and channels for independent workloads.
- Recognize the roles of `Send`, `Sync`, and `Arc`.
- Understand the difference between concurrency and parallelism.
- Explain how futures and `.await` work conceptually.
- Implement basic asynchronous request processing.
- Apply concurrency limits and timeouts.

### Engineering judgment

You should also become better at deciding:

- When to borrow and when to take ownership.
- When cloning is justified.
- When to use traits versus concrete types.
- When to introduce concurrency.
- When asynchronous programming is appropriate.
- When performance optimization is worth the added complexity.

---

## 4. Prerequisites

Before beginning, ensure that you have:

- Basic familiarity with Rust syntax.
- Some experience creating Rust projects.
- A working understanding of functions, structs, and collections.
- A Windows, Linux, or macOS development environment.
- Approximately two uninterrupted hours each day.

You should be comfortable opening a terminal, editing source files, and running commands.

Prior experience with Python, C++, backend development, or AI applications is useful but not required.

---

## 5. Development Environment

This course uses Zed as the primary editor and Cargo as the project and build manager.

### 5.1 Install Rust

Install Rust using the official instructions:

https://www.rust-lang.org/tools/install

On Windows, the standard installation uses `rustup`. Follow any platform-specific prerequisites described by the official installer.

Verify the installation:

```bash
rustc --version
cargo --version
rustup --version
```

Use the stable toolchain unless a particular exercise requires something different.

```bash
rustup update stable
rustup default stable
```

### 5.2 Install development components

Install the components used throughout the course:

```bash
rustup component add rust-analyzer rustfmt clippy
```

These provide:

| Component | Purpose |
|---|---|
| `rust-analyzer` | Code completion, navigation, type information, and diagnostics |
| `rustfmt` | Automatic code formatting |
| `clippy` | Additional linting and code-quality checks |

### 5.3 Configure Zed

Official documentation:

- Rust support: https://zed.dev/languages/rust
- Tasks: https://zed.dev/docs/tasks
- Keybindings: https://zed.dev/docs/key-bindings
- Running and testing: https://zed.dev/docs/running-testing

Recommended editor capabilities:

- Rust language-server integration.
- Automatic formatting on save.
- Type and parameter hints.
- Compiler diagnostics.
- Go to definition and find references.
- Integrated terminal.
- Project-specific Cargo tasks.

Create a `.zed/tasks.json` file in your project when you want convenient commands for checking, testing, formatting, and building.

Suggested task labels:

```text
Rust: Run
Rust: Check
Rust: Test All
Rust: Clippy
Rust: Format
Rust: Build Release
Rust: Full Quality Check
```

Configure keyboard shortcuts only after confirming that the corresponding Zed commands work in your installation.

### 5.4 Verify the environment

Create a temporary project:

```bash
cargo new rust-fluency-check
cd rust-fluency-check
cargo check
cargo test
```

Both commands should complete successfully.

Once this works, the environment is ready.

---

## 6. How the Course Works

Each day follows a two-hour schedule.

| Duration | Activity | Purpose |
|---|---|---|
| 15 minutes | Recall practice | Identify what you remember without assistance |
| 20 minutes | Concept review | Recover important mental models |
| 55 minutes | Hands-on implementation | Apply the concepts in code |
| 20 minutes | Testing and debugging | Validate behavior and investigate failures |
| 10 minutes | Reflection | Record lessons and remaining gaps |

This schedule is a guide, not a rigid rule. Some exercises may require more time than others.

### The learning cycle

For every significant exercise:

1. Read the problem.
2. Predict the expected behavior.
3. Implement a solution without immediately looking up a complete answer.
4. Compile and run the code.
5. Inspect errors and unexpected results.
6. Explain why the implementation works.
7. Refactor if a clearer solution is available.

### Keep a learning journal

Create a file named `LEARNING_LOG.md`.

After each session, record:

```markdown
# Day N

## Concepts reviewed

- ...

## Exercises completed

- ...

## Compiler errors encountered

- Error:
- Root cause:
- Resolution:

## Concepts I can explain confidently

- ...

## Concepts I need to revisit

- ...

## Next session

- ...
```

The journal should capture recurring difficulties and useful insights rather than reproduce the course material.

---

## 7. Seven-Day Roadmap

## Day 1 — Ownership, Borrowing, and Lifetimes

**Objective:** Develop a reliable mental model of Rust's memory-safety rules.

### Topics

- Ownership and move semantics.
- The `Copy` and `Clone` traits.
- Borrowing and references.
- Mutable versus immutable references.
- `String` versus `&str`.
- Borrow checker diagnostics.
- Lifetime annotations.
- Reference validity.

### Hands-on exercises

1. Observe how ownership moves between variables.
2. Compare move semantics with `Copy`.
3. Borrow a string without transferring ownership.
4. Modify data through a mutable reference.
5. Investigate conflicting borrows of a vector.
6. Implement a function that returns one of two borrowed strings.
7. Explain why returning a reference to a local `String` is invalid.

### Mini-project

Implement a model-name utility with these functions:

```rust
fn normalize_model_name(model: &str) -> String;

fn is_supported_model(model: &str) -> bool;

fn append_optimized_suffix(model: &mut String);

fn longest_model_name<'a>(
    a: &'a str,
    b: &'a str,
) -> &'a str;
```

### Completion criteria

- [ ] Explain move semantics.
- [ ] Distinguish `Copy` from `Clone`.
- [ ] Choose between owned values and borrowed references.
- [ ] Explain mutable borrowing restrictions.
- [ ] Understand the purpose of lifetime annotations.
- [ ] Complete the model-name utility.

**Expected deliverable:** A working model-name utility and a written explanation of the ownership model.

---

## Day 2 — Types, Traits, Generics, and API Design

**Objective:** Design clear, reusable Rust interfaces.

### Topics

- Structs and methods.
- Enums and exhaustive pattern matching.
- `Option<T>` and `Result<T, E>`.
- Traits and associated types.
- Generic functions and trait bounds.
- `impl Trait` and `dyn Trait`.
- Static versus dynamic dispatch.
- Module organization.

### Hands-on exercises

1. Model an inference request using a struct.
2. Represent request states with an enum.
3. Implement exhaustive status handling.
4. Create a trait for processing requests.
5. Implement a token counter and a request validator.
6. Introduce a custom validation error type.
7. Separate types, validation, and processing into modules.

### Mini-project

Build a small inference-request processing library.

It should support:

- Typed inference requests.
- Explicit request states.
- Token-count calculations.
- Input validation.
- Multiple processor implementations.

### Completion criteria

- [ ] Use structs and enums appropriately.
- [ ] Explain exhaustive pattern matching.
- [ ] Implement a trait.
- [ ] Explain the difference between generic and dynamic dispatch.
- [ ] Return structured validation results.
- [ ] Organize the implementation into modules.

**Expected deliverable:** A modular inference-request library with at least two processing implementations.

---

## Day 3 — Error Handling, Iterators, and Data Processing

**Objective:** Write robust Rust code that processes real-world data.

### Topics

- `Result<T, E>` and `Option<T>`.
- The `?` operator.
- Custom error types.
- Error conversion and context.
- Iterator adapters.
- `map`, `filter`, `fold`, and `collect`.
- Streaming file processing.
- Numeric aggregation.
- Input validation.

### Hands-on exercises

1. Parse numeric values using `Result`.
2. Transform vectors using iterators.
3. Calculate mean and maximum latency.
4. Parse JSONL records one line at a time.
5. Recover from malformed records.
6. Track errors with line numbers.
7. Handle empty input and invalid numeric values.

### Mini-project

Implement a streaming JSONL processor that reads inference workload records.

Each record should include fields such as:

```json
{
  "request_id": "req-001",
  "model": "llama-3.2-1b",
  "prompt_tokens": 512,
  "completion_tokens": 128,
  "latency_ms": 240
}
```

The processor should validate records and accumulate statistics without retaining the entire input file in memory.

### Completion criteria

- [ ] Propagate errors appropriately.
- [ ] Use iterators for common transformations.
- [ ] Process input incrementally.
- [ ] Report malformed records.
- [ ] Avoid division by zero.
- [ ] Explain how memory usage depends on the processing strategy.

**Expected deliverable:** A streaming JSONL parser and workload-statistics component.

---

## Day 4 — Cargo, Modules, Testing, and Code Quality

**Objective:** Develop a maintainable Rust application.

### Topics

- Cargo project structure.
- Dependency management.
- Serialization with Serde.
- Command-line interfaces with Clap.
- Unit testing.
- Integration testing.
- Formatting and linting.
- Error reporting.
- Application and library boundaries.

### Hands-on exercises

1. Create a Cargo project.
2. Add the necessary dependencies.
3. Implement a command-line interface.
4. Write unit tests for parsing and validation.
5. Add integration tests.
6. Test malformed and empty input.
7. Run formatting and linting checks.
8. Build an optimized executable.

### Mini-project

Turn the JSONL processor into a CLI application.

Example:

```bash
cargo run -- analyze ./data/inference.jsonl --output report.json
```

The application should produce a JSON report containing workload statistics and error counts.

### Completion criteria

- [ ] Manage dependencies with Cargo.
- [ ] Serialize reports using Serde.
- [ ] Parse command-line arguments.
- [ ] Write unit and integration tests.
- [ ] Run Clippy without unresolved warnings.
- [ ] Produce a release build.

**Expected deliverable:** A tested, modular command-line application.

---

## Day 5 — Threads and Parallel Processing

**Objective:** Understand Rust's thread-safety guarantees and basic parallel programming.

### Topics

- `std::thread`.
- Thread handles and joining.
- `Send` and `Sync`.
- `Arc` and `Mutex`.
- Message passing with channels.
- Shared state versus local accumulation.
- Parallel workload partitioning.
- Performance measurement.

### Hands-on exercises

1. Spawn worker threads.
2. Partition a synthetic dataset.
3. Process each partition independently.
4. Return statistics from worker threads.
5. Merge worker results.
6. Handle worker failures.
7. Compare sequential and parallel execution.

### Mini-project

Implement a parallel mode for the workload processor.

Compare its output and performance with the sequential version.

Measure processing time and throughput across multiple dataset sizes.

### Completion criteria

- [ ] Explain `Send` and `Sync`.
- [ ] Spawn and join threads.
- [ ] Aggregate independent results.
- [ ] Handle worker failures.
- [ ] Explain the difference between concurrency and parallelism.
- [ ] Report measured performance without assuming parallelism is always faster.

**Expected deliverable:** A parallel processing implementation and a basic benchmark report.

---

## Day 6 — Async Rust and Service-Oriented Programming

**Objective:** Understand the foundations of asynchronous Rust.

### Topics

- `async fn` and `.await`.
- Futures.
- Tokio runtime fundamentals.
- Tasks versus operating-system threads.
- Concurrent I/O.
- Timeouts and cancellation.
- Concurrency limits.
- Backpressure.
- Structured error handling.

### Hands-on exercises

1. Configure Tokio.
2. Write asynchronous functions.
3. Simulate concurrent inference requests.
4. Compare sequential and concurrent waiting.
5. Limit concurrent work with a semaphore.
6. Introduce timeouts.
7. Handle failures and consider graceful shutdown.

### Mini-project

Build an asynchronous request-processing prototype.

It should simulate a downstream inference service and enforce a limit on the number of requests processed concurrently.

### Completion criteria

- [ ] Explain what an async function returns.
- [ ] Understand what `.await` does.
- [ ] Distinguish asynchronous execution from CPU parallelism.
- [ ] Run multiple asynchronous tasks.
- [ ] Apply a concurrency limit.
- [ ] Handle timeouts and failures.

**Expected deliverable:** An asynchronous request-processing prototype.

---

## Day 7 — Consolidation and Engineering Review

**Objective:** Combine the week's learning into a coherent, testable software component.

### Activities

1. Review important language concepts without notes.
2. Finish the workload-inspection CLI.
3. Refactor duplicated code.
4. Verify parsing and validation behavior.
5. Compare sequential and parallel implementations.
6. Run all tests and quality checks.
7. Write a clear README for the project.
8. Document remaining limitations and future improvements.

### Final project requirements

The final project should provide:

- Streaming JSONL ingestion.
- Typed record parsing.
- Input validation.
- Recoverable error handling.
- Workload statistics.
- JSON report generation.
- Unit and integration tests.
- A usable command-line interface.
- A reproducible build and test workflow.

Parallel processing should be included only if it can be implemented and validated properly.

The asynchronous prototype may remain a separate example.

### Completion criteria

- [ ] The application builds successfully.
- [ ] Tests pass.
- [ ] Invalid records are handled consistently.
- [ ] Reports contain correct statistics.
- [ ] The project has a clear module structure.
- [ ] Performance claims are supported by measurements.
- [ ] The README explains setup, usage, and limitations.

**Expected deliverable:** A working Rust project suitable for continued development.

---

## 8. Main Project: Rust AI Workload Inspector

The course revolves around a single project that evolves throughout the week.

### Problem statement

AI systems generate large quantities of inference-related data.

Engineers need tools to inspect workloads, validate records, calculate operational statistics, and identify unexpected behavior.

Build a command-line utility that processes JSONL records representing inference requests.

### Input format

Each line contains a JSON object.

Example:

```json
{"request_id":"req-001","model":"llama-3.2-1b","prompt_tokens":512,"completion_tokens":128,"latency_ms":240}
{"request_id":"req-002","model":"llama-3.2-1b","prompt_tokens":1024,"completion_tokens":256,"latency_ms":410}
{"request_id":"req-003","model":"qwen-small","prompt_tokens":256,"completion_tokens":64,"latency_ms":120}
```

### Core features

The final application should:

1. Read a JSONL file incrementally.
2. Deserialize records into typed Rust structures.
3. Validate required fields and numeric constraints.
4. Detect duplicate request IDs.
5. Track valid and invalid records.
6. Calculate token and latency statistics.
7. Produce a JSON report.
8. Report malformed input with useful context.
9. Include automated tests.

### Example report

```json
{
  "total_records": 1000,
  "valid_records": 985,
  "invalid_records": 15,
  "total_prompt_tokens": 250000,
  "total_completion_tokens": 85000,
  "mean_latency_ms": 245.6,
  "max_latency_ms": 1820
}
```

The values above are illustrative.

Define the exact meaning of each statistic and how invalid records affect the calculation. For example, mean latency should normally be calculated over valid records with valid latency measurements.

### Suggested project structure

```text
rust-ai-workload-inspector/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── data/
│   └── sample.jsonl
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs
│   ├── model.rs
│   ├── parser.rs
│   ├── validation.rs
│   ├── stats.rs
│   ├── report.rs
│   └── parallel.rs
└── tests/
    ├── cli_tests.rs
    └── integration_tests.rs
```

This is a target architecture, not a requirement to create every module immediately.

Introduce modules when they have a clear responsibility.

### Suggested dependencies

| Crate | Purpose |
|---|---|
| `serde` | Serialization and deserialization traits |
| `serde_json` | JSON processing |
| `clap` | Command-line argument parsing |
| `anyhow` | Application-level error context |

Use standard-library functionality where it is sufficient.

For library APIs requiring detailed error categorization, consider defining explicit error types rather than exposing generic application errors.

---

## 9. Development Workflow

Use the same quality checks throughout the course.

### Compile-check

```bash
cargo check
```

Checks the project for compilation errors without performing a complete executable build.

### Run the application

```bash
cargo run
```

### Run tests

```bash
cargo test
```

### Display test output

```bash
cargo test -- --nocapture
```

### Format the project

```bash
cargo fmt
```

### Check formatting

```bash
cargo fmt --check
```

### Run Clippy

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Build an optimized executable

```bash
cargo build --release
```

### Full quality check

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Run the commands individually, or combine them using shell-appropriate command chaining.

A successful quality check does not prove that the program is correct. It establishes a useful baseline of compilation, formatting, linting, and test validation.

---

## 10. Engineering Principles

These principles guide the entire course.

### 10.1 Prefer understanding over memorization

You do not need to memorize every Rust method or compiler message.

You need to understand the concepts well enough to identify the right abstraction and consult documentation when necessary.

### 10.2 Use the compiler as a learning tool

Compiler errors are valuable feedback.

When a borrow-checker error appears, determine:

- Which value is borrowed?
- Who owns it?
- Is the borrow immutable or mutable?
- When is the reference last used?
- Is the referenced value still alive?

Avoid changing code randomly until it compiles.

### 10.3 Borrow when ownership transfer is unnecessary

For read-only functions, references often produce more flexible interfaces.

For example:

```rust
fn inspect_model(model: &str) {
    println!("{model}");
}
```

Take ownership when the function genuinely needs to own the value, retain it, or transfer it elsewhere.

### 10.4 Avoid unnecessary cloning

Cloning may be appropriate when independent ownership is needed.

However, cloning large strings, collections, or buffers can introduce unnecessary allocation and copying.

Understand the ownership requirements before introducing `.clone()`.

### 10.5 Make errors explicit

Malformed input, unavailable files, and failed downstream operations should be handled intentionally.

Use `Result` and meaningful error context rather than relying on `unwrap()` throughout application code.

### 10.6 Test behavior, not implementation details

Tests should verify the expected behavior of your components.

For example, a statistics test should verify the actual totals and averages, not merely that a function returns without panicking.

### 10.7 Measure before optimizing

Rust provides strong performance capabilities, but not every implementation is automatically optimal.

Measure throughput, latency, and memory consumption against a defined workload before deciding whether a refactor or parallel implementation is beneficial.

### 10.8 Keep the architecture proportional to the problem

Avoid introducing traits, abstractions, threads, or asynchronous execution merely to demonstrate a language feature.

Every abstraction should have a reason to exist.

---

## 11. Progress Tracking

Use this checklist to track your progress.

| Day | Main topic | Deliverable | Status |
|---|---|---|---|
| 1 | Ownership, borrowing, lifetimes | Model-name utility | [ ] |
| 2 | Types, traits, generics | Request-processing library | [ ] |
| 3 | Error handling and iterators | Streaming JSONL parser | [ ] |
| 4 | Cargo, testing, architecture | Tested CLI | [ ] |
| 5 | Threading and parallelism | Parallel processor and measurements | [ ] |
| 6 | Async Rust | Async request-processing prototype | [ ] |
| 7 | Integration and review | Completed workload inspector | [ ] |

### Self-assessment

At the end of the course, score yourself on each skill.

Use the following scale:

- **0:** Cannot explain or implement it reliably.
- **1:** Can implement it with documentation or assistance.
- **2:** Can implement it independently and explain the design.

| Skill | Score |
|---|---:|
| Ownership and borrowing | /2 |
| Lifetimes | /2 |
| Structs, enums, and pattern matching | /2 |
| Traits and generics | /2 |
| Error handling | /2 |
| Iterators and collections | /2 |
| Modules and Cargo | /2 |
| Testing and debugging | /2 |
| Threading | /2 |
| Async fundamentals | /2 |
| **Total** | **/20** |

The score is a reflection tool, not a certification.

For each score below 2, identify one concrete exercise that would help you improve.

---

## 12. Recommended Resources

Use the official documentation as your primary reference.

### The Rust Book

https://doc.rust-lang.org/book/

A structured explanation of Rust's core concepts, including ownership, enums, error handling, generics, testing, and concurrency.

### Rust Standard Library

https://doc.rust-lang.org/std/

Reference documentation for standard-library types and APIs.

### Rust by Example

https://doc.rust-lang.org/rust-by-example/

Practical examples of language features.

### Cargo Book

https://doc.rust-lang.org/cargo/

Documentation for project management, dependencies, builds, and workspaces.

### Clippy

https://doc.rust-lang.org/clippy/

Documentation for Rust's linting tool.

### Serde

https://serde.rs/

Documentation for serialization and deserialization.

### Tokio

https://tokio.rs/

Documentation and tutorials for asynchronous Rust.

### Zed

https://zed.dev/docs/

Documentation for editor configuration, tasks, keybindings, and development workflows.

**Recommended approach:** Consult documentation when you need to understand a concept or resolve a specific problem. Avoid reading all the resources sequentially during this short course.

---

## 13. Definition of Completion

The course is complete when you can demonstrate that you have built the main project and understand its design.

At a minimum, you should be able to:

- Explain the ownership decisions in your implementation.
- Identify where references are used and why.
- Explain the error-handling strategy.
- Describe the module boundaries.
- Demonstrate that malformed input is handled correctly.
- Run the tests and explain what they verify.
- Explain whether parallel processing improved performance.
- Describe the purpose of the asynchronous prototype.
- Identify remaining technical limitations.

You do not need to master every advanced topic in Rust.

You should finish with a working project, better language fluency, and a clear plan for your next engineering challenge.

---

## 14. What's Next?

After completing this course, choose a project based on your engineering interests.

### Option A: Inference gateway

Build a Rust service that accepts inference requests, applies concurrency limits, tracks latency, and forwards requests to an existing model server.

**Focus:** Async Rust, networking, error handling, and service architecture.

### Option B: High-performance dataset processor

Extend the workload inspector into a reusable data-processing library.

**Focus:** Streaming, parallelism, memory allocation, profiling, and benchmarks.

### Option C: Python–Rust integration

Expose a computationally expensive Rust component to Python.

**Focus:** Foreign-function interfaces, Python interoperability, data conversion, and performance comparisons.

### Option D: Native AI infrastructure

Explore components such as token-processing utilities, inference schedulers, caching systems, or CPU-based tensor operations.

**Focus:** Systems design, resource management, data layout, and performance engineering.

Choose one project and deepen your knowledge through its actual requirements rather than attempting to study every advanced Rust topic at once.

---

## Final Note

Rust fluency comes from repeatedly applying a small set of powerful ideas: ownership, explicit types, predictable error handling, safe concurrency, and disciplined software design.

Use this course to strengthen those ideas through implementation.

**Write code. Predict behavior. Read compiler errors. Test your assumptions. Measure your results.**

The goal is not merely to finish seven days of exercises. It is to become a more independent Rust engineer who can build, debug, and maintain useful software.

### Suggested repository setup

Keep the README at the root of the course repository. I'd organize the learning material like this:

```text
rust-engineering-fluency/
├── README.md
├── LEARNING_LOG.md
├── day-01-ownership/
├── day-02-types-and-traits/
├── day-03-error-handling/
├── day-04-cargo-and-testing/
├── day-05-concurrency/
├── day-06-async-rust/
└── day-07-final-project/
```

This layout separates each day's experiments while leaving the final project free to develop its own structure.
