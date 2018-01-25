# Introduction to Rust
## Installation
### Linux & Mac
To install Rust, just copie the following line into the terminal.

```
curl https://sh.rustup.rs -sSf | sh
```

You also need to add the binaries to the `PATH`, you can do that by adding the following line into the `.bashrc`.

```
export PATH=$PATH:~/.cargo/bin
```

### Windows
You can download and execute the `.msi` file [here](https://static.rust-lang.org/dist/rust-1.23.0-i686-pc-windows-msvc.msi).

### IDE
Rust has the RLS (Rust Language Server) that allows IDE functionnalities into almost every editors. You can download the Rust plugin for your favorite editor and install the Rust Language Server by executing the following lines into a terminal or cmd.

```
rustup update
rustup component add rls-preview rust-analysis rust-src
```