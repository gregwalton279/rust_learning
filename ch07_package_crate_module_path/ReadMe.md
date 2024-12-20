# 1. 包package和crate
### 新建package包
```bash
cargo new xxx

tree
.
├── Cargo.toml
└── src
    └── main.rs
```

> 约定：
> 
> 1. src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。
>
> 2. 如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，且 src/lib.rs 是 crate 根。
     
crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。

> 举例：
> 
> 有了一个只包含 src/main.rs 的包，意味着它只含有一个名为 my-project 的二进制 crate。
> 
> 如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个库和一个二进制项，且名字都与包相同。
> 
> 通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。



### 新建lib命令
```bash
cargo new --lib xxx
```

