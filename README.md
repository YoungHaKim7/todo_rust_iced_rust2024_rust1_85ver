# link

- iced 관련
  - [(외부링크)Rust iced eBook](https://book.iced.rs/)
    - https://iced.rs/
  - [Font세팅 및 크기 조절](#font-사이즈--세팅)
  - [macro column이해](#macro-column)
  - [iced widget이해](#icedwidget)
  - 

- TodoApp정보
  - [TodoApp_작성 버젼](#rust-icedgui-verrust-editon-2024)
  - [nightly 디버깅 팁](#run--debugging)


- 문제해결 bug & fix
  - [Hey Rustaceans! Got a question? Ask here](#hey-rustaceans-got-a-question-ask-here-162023)

<hr />


# Rust Iced(GUI) Github 공식[|🔝|](#link)

- https://github.com/iced-rs/iced

<hr />

# rust iced(GUI) ver.(Rust Editon 2024)[|🔝|](#link)
```toml
# 250312 작성함
iced = "0.14.0-dev"

# 이걸로 test중
iced = { version ="0.14.0-dev" , git = "https://github.com/iced-rs/iced", rev ="fd5ed0d"}
```

- rust version

```bash
$ rustc --version --verbose

rustc 1.87.0-nightly (efea9896f 2025-03-08)
binary: rustc
commit-hash: efea9896f506baa08f40444e07774e827646d57a
commit-date: 2025-03-08
host: aarch64-apple-darwin
release: 1.87.0-nightly
LLVM version: 20.1.0
```


# iced::widget[|🔝|](#link)
# Macro column[|🔝|](#link)
- https://docs.rs/iced/latest/iced/widget/macro.column.html

<hr />

# Font 사이즈 & 세팅[|🔝|](#link)

- https://docs.rs/iced/latest/iced/settings/struct.Settings.html


<hr />

# Run & Debugging[|🔝|](#link)

- nightly backtrace debugging

```bash
$ RUSTFLAGS="-Zmacro-backtrace" cargo r
```

- https://github.com/rustwasm/wasm-pack/issues/666
- https://github.com/rust-lang/cargo/issues/7233


- Describe the problem you are trying to solve
Currently `cargo test --target wasm32-unkown-unkown` will try to compile all examples. This is a problem if there is code that uses dev-dependencies that are not available on this target, like networking crates.

- Describe the solution you'd like
This already works for files in the tests directory, but not for examples:
I would like to see an example ignored when it has: `#![ cfg(not( target_arch = "wasm32" )) ]` at the top of the file.

- Currently this throws an error like:

- I made some progress without modifying cargo:

- I tried `package.autoexamples = false + [[target."cfg(not( target_arch = \"wasm32\" ))".example]]` but that failed. It seems that the target selection is specific to dependencies?
 - I managed a workaround with `package.autoexamples = false`, a notwasm feature and using required-features = notwasm in the example section. The downside of this is that users can only run examples with: `cargo run --example basic --features notwasm`, which is unfortunate and also one needs to list all examples in `Cargo.toml`. I suppose this technique could also work with a nightly feature to allow tests to run in stable

# Hey Rustaceans! Got a question? Ask here (16/2023)![|🔝|](#link)
- https://www.reddit.com/r/rust/comments/12p5xh4/hey_rustaceans_got_a_question_ask_here_162023/?rdt=45360


<hr />

# Anti-Aliasing이해(안티에일리어싱)[|🔝|](#link)
- https://youtu.be/_c_ffkt9Lio?si=IDp_S1tC9CZOS15e

# 안티에일리어싱[|🔝|](#link)
최근 수정 시각: 2025-02-23 18: 27:14
- https://namu.wiki/w/%EC%95%88%ED%8B%B0%EC%97%90%EC%9D%BC%EB%A6%AC%EC%96%B4%EC%8B%B1



<hr />
