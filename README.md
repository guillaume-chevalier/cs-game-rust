# Introduction to Rust
## Installation
### Linux & Mac
To install Rust, just copie the following line into the terminal.

```
curl https://sh.rustup.rs -sSf | sh
```

You also need to add the binaries to the `PATH`. You can do that by adding the following line into the `.bashrc`.

```
export PATH=$PATH:~/.cargo/bin
```

### Windows
You can download and execute the `.msi` file [here](https://static.rust-lang.org/dist/rust-1.23.0-i686-pc-windows-msvc.msi).

### Test
To test the installation you can create an hello world executable with cargo and run it.

```
cargo init --bin
cargo run
```

If there is no error message and you see `Hello, world!`, then you are all set !

### IDE
Rust has the RLS (Rust Language Server) that allows IDE functionalities into almost every editors. You can download the corresponding Rust plugin for your favorite editor and install the Rust Language Server by executing the following lines.

```
rustup update
rustup component add rls-preview rust-analysis rust-src
```
