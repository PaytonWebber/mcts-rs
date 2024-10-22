# Rust Monte Carlo Tree Search (MCTS) with Arena Allocator

[![Crates.io](https://img.shields.io/crates/v/mcts-rs.svg)](https://crates.io/crates/mcts-rs)
[![Documentation](https://docs.rs/mcts-rs/badge.svg)](https://docs.rs/mcts-rs)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.56%2B-orange.svg)

A Rust implementation of the Monte Carlo Tree Search (MCTS) algorithm using an arena allocator for efficient memory management. This project features a Tic-Tac-Toe game to showcase the MCTS algorithm in action.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Adding to Your Project](#adding-to-your-project)
  - [Running the Tic-Tac-Toe Example](#running-the-tic-tac-toe-example)
- [Implementation Details](#implementation-details)
  - [Arena Allocator](#arena-allocator)
  - [MCTS Algorithm](#mcts-algorithm)
  - [State Trait](#state-trait)
- [License](#license)

## Introduction

Monte Carlo Tree Search (MCTS) is a search algorithm used for decision-making processes. This project provides a Rust implementation of MCTS that efficiently manages memory using an arena allocator. By storing all nodes in a central arena, we avoid the overhead of reference counting and interior mutability, resulting in a more performant and idiomatic Rust codebase.

The `mcts-rs` crate is now available on [crates.io](https://crates.io/crates/mcts-rs), making it easy to include in your Rust projects.

The included Tic-Tac-Toe game serves as a practical example of how to use the MCTS library.

## Features

- **Efficient Memory Management**: Utilizes an arena allocator to store nodes, reducing allocation overhead.
- **Generic State Management**: Defines a `State` trait to allow MCTS to work with any game or decision process.

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

### Adding to Your Project

To include `mcts-rs` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
mcts-rs = "0.1.0"
```

Replace `"0.1.0"` with the latest version available on [crates.io](https://crates.io/crates/mcts-rs).

### Running the Tic-Tac-Toe Example

To run the Tic-Tac-Toe game where the MCTS algorithm plays against itself with a random starting move, clone the repository and use the following commands:

```bash
git clone https://github.com/PaytonWebber/mcts-rs.git
cd mcts-rs
cargo run --example tic_tac_toe
```

## Implementation Details

### Arena Allocator

An arena allocator is used to efficiently manage memory for the nodes in the MCTS tree. Nodes are stored in a vector, and their relationships are represented by indices rather than pointers or references. This approach avoids the need for reference counting (`Rc`) and interior mutability (`RefCell`), leading to cleaner and more efficient code.

**Benefits:**

- **Performance**: Reduced allocation overhead and improved cache locality.
- **Simplicity**: Simplifies ownership and borrowing by avoiding complex lifetime issues.
- **Safety**: Leverages Rust's safety guarantees without resorting to unsafe code.

### MCTS Algorithm

The MCTS algorithm consists of four main steps:

1. **Selection**: Starting from the root node, select child nodes based on the Upper Confidence Bound (UCB) until a leaf node is reached.
2. **Expansion**: If the leaf node is not a terminal state, expand it by adding all possible child nodes.
3. **Simulation**: Run a simulation from the expanded node to a terminal state by making random moves.
4. **Backpropagation**: Update the nodes along the path with the simulation result.

**Key Components:**

- **Node Struct** (`node.rs`): Represents a node in the search tree.
- **Arena Struct** (`arena.rs`): Stores all nodes and manages parent-child relationships between nodes.
- **MCTS Implementation** (`mod.rs`): Contains the logic for selection, expansion, simulation, and backpropagation.

### State Trait

The `State` trait abstracts the game logic, allowing the MCTS algorithm to work with any game or decision process that implements this trait. Here's the trait definition:

```rust
pub trait State {
    /// Checks if the specified player has won the game.
    fn player_has_won(&self, player: usize) -> bool;

    /// Determines if the current state is a terminal state (no further moves possible).
    fn is_terminal(&self) -> bool;

    /// Returns a vector of legal actions available from the current state.
    fn get_legal_actions(&self) -> Vec<(usize, usize)>;

    /// Returns the index of the player whose turn it is to play.
    fn to_play(&self) -> usize;

    /// Returns a new state resulting from applying the given action to the current state.
    fn step(&self, action: (usize, usize)) -> Self;

    /// Calculates and returns the reward for the specified player in the current state.
    fn reward(&self, player: usize) -> f32;

    /// Renders or prints the current state (useful for debugging or display purposes).
    fn render(&self);
}
```

By implementing this trait for your game or decision process, you can integrate it with the MCTS algorithm provided in this library. The `tic_tac_toe.rs` file offers an example implementation of the `State` trait for Tic-Tac-Toe.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
