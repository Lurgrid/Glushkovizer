# Glushkovizer

Program that transforms a regular expression into a Glushkov automaton and then
transforms it into a grammar

## Formal definition

The formal definition of data types used by this library is available in French
[here](./glushkovizer/doc/formal.pdf)

## Usage

**Library example :**

```bash
$ cd glushkovizer
$ cargo run --example simple_json
```

**CLI version (example) :**

```bash
$ cd glushkovizer
$ cargo build --example cli --release
$ ./target/release/examples/cli
Please enter a regular expression - Press Ctrl + D to quit
(a+b).a*.b*.(a+b)*
Concat(Concat(Concat(Or(Symbol('a'), Symbol('b')), Repeat(Symbol('a'))), Repeat(Symbol('b'))), Repeat(Or(Symbol('a'), Symbol('b'))))
Enter a filename to save the automata - Press Ctrl + D to not save
toto
Saved !
Please enter a regular expression - Press Ctrl + D to quit
```

After execution, the `svg` images were saved.

**GTK-4 version :**

Theoretically, the `gtk4` version can run on Windows if all dependencies are
properly installed, but has not been tested. Has only been tested on a Linux

```bash
$ cd ./glushkovizer-gtk4
$ cargo build --release
$ ./target/release/glushkovizer-gtk4
# or
$ cargo build --release --no-default-features --features no-adwaita
$ ./target/release/glushkovizer-gtk4
```

### Run Dependencies

- `dot 9.0 >=` _(May work on an earlier version, but has not been tested)_

  _Click [here](https://graphviz.org/download/) to install it_

For the `gtk4` version :

- `gtk4 4.14 >=`

  _(Has also been tested in 4.6.9)_

- `libadwaita 1.5 >=`

  _optional_

  _(Has also been tested in 1.1.7)_

### Build Dependencies

For the `gtk4` version :

- `blueprint-compiler 0.10 >=` _(May work on an earlier version, but has not been tested)_

  _optional with the feature `no-blp`_

- `gcc 14.1 >=` _(May work on an earlier version, but has not been tested)_

- `gtk4 devel 4.14 >=` For installation, please refer to the [book](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html)

- `libadwaita devel 1.5 >=` For installation, please refer to the [book](https://gtk-rs.org/gtk4-rs/stable/latest/book/libadwaita.html)

  _optional with the feature `no-adwaita`_

## License

GPLv3

---

> GitHub [@Lurgrid](https://github.com/Lurgrid)
