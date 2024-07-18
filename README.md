# Next Route Viz

A tool to print out all routes in a NEXT js repo.

<img width="246" alt="image" src="https://github.com/user-attachments/assets/5b7cee5b-d666-463c-a778-5e7657dde71f">

## Install

To install you need to clone the repository, since this is a rust project, you need to [install rust](https://doc.rust-lang.org/cargo/getting-started/installation.html), after installing rust, you need to open a terminal and run:

```shell
cargo install --path <path-to-repo>
```

Replace `path-to-repo` with the path to the folder where you cloned the repository. This will build and install the binary into your path. For more information on how cargo install works you can [check the docs](https://doc.rust-lang.org/cargo/commands/cargo-install.html).

If you don't want to install it you can still run

```shell
cargo run -- -p ~/path-to-repo-to-analyze
```

standing inside the repo path, but installing it makes the `next-route-viz` binary available anywhere.

## Commands

### Setting the repo path

You'll probably want to specify the place where your repo is (locally).

```shell
next-route-viz -p ~/path-to-repo 
```

If you don't, then it'll try to open the repo at your current directory.
