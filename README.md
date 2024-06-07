# Glushkovizer

Manipulation, regular expression and automaton library. Allows conversion of
regular expressions into automata and analysis of automata and their orbits.

## Formal definition

The formal definition of data types used by this library is available in French
[here](./glushkovizer/doc/formal.pdf)

## Examples

To see an example of a graphics application using this library, go to this
repository :

> https://github.com/Lurgrid/GlushkovizerApp-GTK4

```bash
$ cargo run --example simple_json
```

**CLI version (example) :**

```bash
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

### Run Dependencies

- `dot 9.0 >=` _(May work on an earlier version, but has not been tested)_

  _Click [here](https://graphviz.org/download/) to install it_

## License

GPLv3

---

> GitHub [@Lurgrid](https://github.com/Lurgrid)
