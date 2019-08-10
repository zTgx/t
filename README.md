# t 
Generate Cargo project template in Rust

NOTE:
`git` required.

# Usage
```
git clone https://github.com/zTgx/t.git
cd t
cargo install --path .
templateme --new proj_name
```
Template project's src tree :  
```
├── Cargo.toml
├── docs
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── .gitignore
├── examples
└── src
```

and in `Cargo.toml` includings:  
```
[package]
name = "proj_name"
version = "0.1.0"
authors = ["Replace Me"]
edition = "2018"
repository = "https://github.com/Replace Me.git"
readme = "README.md"
keywords = [ "Replace Me" ]
categories = [ "Replace Me" ]
license = "MIT/Apache-2.0"
exclude = [ "/.travis.yml" ]
description = "Replace Me"

[dependencies]
```  
and in `.gitignore` includings:  
```
/target
**/*.rs.bk
Cargo.lock
```

and others includings default contents.  


# Have Fun.
