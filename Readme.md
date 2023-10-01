<div align="center">
    <img src="https://github.com/sunface/rust-by-practice/blob/master/en/assets/header.jpg?raw=true">
</div>

    
<p align="center">Practice Rust with challenging examples, exercises and projects</p>

This fork from https://github.com/sunface/rust-by-practice


## Reading online

- [https://practice.rs](https://practice.rs)

## Features

Part of our examples and exercises are borrowed from [Rust By Example](https://github.com/rust-lang/rust-by-example), thanks for your great works!

Although they are so awesome, we have our own secret weapons :)

- There are three parts in each chapter: examples, exercises and practices

- Besides examples, we have `a lot of exercises`, you can Read, Edit and Run them ONLINE

- Covering nearly all aspects of Rust, such as async/await, threads, sync primitives, optimizing, standard libraries, tool chain, data structures and algorithms etc.

- Every exercise has its own solutions

- The overall difficulties are a bit higher and from easy to super hard: easy 🌟 medium 🌟🌟 hard 🌟🌟🌟 super hard 🌟🌟🌟🌟

We use [mdbook](https://rust-lang.github.io/mdBook/) building our exercises. You can run locally with below steps:

- Clone the repo
```shell
$ git clone git@github.com:sunface/rust-by-practice.git
```
- Install mdbook using Cargo
```shell
$ cargo install mdbook
```

- For Book in English
```shell
$ cd rust-by-practice && mdbook serve en/
```

- For Book in Chinese
```shell
$ cd rust-by-practice && mdbook serve zh-CN/
```
