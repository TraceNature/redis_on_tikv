* clone

```
git clone https://github.com/tikv/client-rust.git
```

* build 

```
cargo build --release
```

* use lib
Copy libtikv_client.rlib to your project directory, just like project/libs

```
cp release/libtikv_client.rlib ~/project/libs
```

* configure Cargo.toml
  
```
tikv-client = { extern = "./libs/libtikv_client.rlib" }
tikv-client = { git = "https://github.com/tikv/client-rust" }
```