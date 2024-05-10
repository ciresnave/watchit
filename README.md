# WatchIt! :eye:

We've got our eyes on your files. WatchIt will run your callback when a file changes. It's easy to use and simple to understand. WatchIt is cross platform and works on Linux, BSD, Mac and Windows.

## Usage

Add watchit to your cargo.toml:

```toml
[dependencies]
    watchit = "0.1"
```

Create and instance of the Watcher with a callback:

```Rust
let mut watcher = Watcher::new(|event| println!(event));
```

Add a file to be watched:

```Rust
watcher.watch("file.txt");
```
