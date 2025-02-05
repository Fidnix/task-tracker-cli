# task-tracker-cli
It's what its name indicates. It is an unserious and minimalist project that I've made from a project idea in roadmap.sh; despite, I'll be updating this project to learn how to work well with Rust

# Usage

## Requirements
* Rustc 1.83.0
* Cargo 1.83.0

## Build

```bash
# For dev versions
cargo build

# For release versions
cargo build --release

# Run
cargo run -- [PARAMS]
```

## Commands

```bash
# Adding a new task
cargo run -- add "Buy groceries"
# Output: Task added successfully (ID: 1)

# Updating and deleting tasks
cargo run -- update 1 "Buy groceries and cook dinner"
cargo run -- delete 1

# Marking a task as in progress or done
cargo run -- mark-in-progress 1
cargo run -- mark-done 1

# Listing all tasks
cargo run -- list

# Listing tasks by status
cargo run -- list done
cargo run -- list todo
cargo run -- list in-progress
```