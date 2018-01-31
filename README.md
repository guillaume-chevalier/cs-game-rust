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
mkdir test-install
cd test-install
cargo init --bin
cargo run
```

If there is no error and you can see `Hello, world!`, then you are all set and can delete the test project.

```
cd ..
rm -rf test-install
```