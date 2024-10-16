# file_traverser

`file_traverser` is a Rust crate that provides an efficient way to traverse file systems recursively while applying customizable filters to files. It supports both standard library `mpsc` and `crossbeam` channels, allowing you to choose the best communication method for your application.

## Features

- **Recursive Directory Traversal**: Explore directories and their subdirectories.
- **Customizable Filtering**: Apply user-defined filters to select specific files.
- **Channel Support**: Use either standard library `mpsc` or `crossbeam` channels for sending file paths.
- **Asynchronous Processing**: Designed for efficient asynchronous file handling.

## Installation

Add the following line to your `Cargo.toml`:

```toml
[dependencies]
file_traverser = "0.1"
```

## Usage

Here's a basic example of how to use file_traverser:

```rust
use std::path::PathBuf;
use std::sync::mpsc::channel;
use file_traverser::filter_and_send_files;

fn main() {
    let (tx, rx) = channel();

    // Define a filter function
    let filter = |path: &Path| path.extension().map(|ext| ext == "txt").unwrap_or(false);

    // Start traversing
    filter_and_send_files(&PathBuf::from("path/to/directory"), tx, filter);

    // Receive paths
    for received in rx {
        println!("Received file: {:?}", received);
    }
}
```