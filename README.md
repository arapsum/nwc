# nwc

A lightweight Rust implementation of the Unix `wc` (word count) utility.

> ⚠️ **Note:** Reading from `stdin` is not yet supported. All commands require a `<FILE>` argument.

---

##  Features

* Count **bytes**, **lines**, **words**, and **characters**
* Supports both short and long flags
* Defaults to standard `wc`-like output when no flags are provided
* Fast and minimal, written in Rust

---

##  Installation

### From source

```sh
git clone https://github.com/arapsum/nwc.git
cd nwc
cargo build --release
```

The binary will be available at:

```sh
./target/release/nwc
```

---

##  Usage

### Help

```sh
nwc --help
```

---

### Count bytes

```sh
nwc -c <FILE>
nwc --bytes <FILE>
```

---

### Count lines

```sh
nwc -l <FILE>
nwc --lines <FILE>
```

---

### Count words

```sh
nwc -w <FILE>
nwc --words <FILE>
```

---

### Count characters

```sh
nwc -m <FILE>
nwc --chars <FILE>
```

---

### Default behaviour

When no flags are specified, `nwc` outputs:

* line count
* word count
* byte count

This is equivalent to:

```sh
nwc -l -w -c <FILE>
```

---

##  Examples

```sh
nwc file.txt
# Output: 10 42 256 file.txt

nwc -w file.txt
# Output: 42 file.txt

nwc -l -c file.txt
# Output: 10 256 file.txt
```

---

##  Roadmap

* [ ] Add `stdin` support
* [ ] Support multiple files
* [ ] Match GNU `wc` output formatting more closely
* [ ] Add benchmarks

---

##  Contributing

Contributions, issues, and feature requests are welcome!

Feel free to fork the repo and submit a pull request.

---

##  License

MIT License. See `LICENSE` for details.

