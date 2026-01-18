A (mostly) libelf-compatible C-API implemented using [object](https://crates.io/crates/object).


This is a very early proof of concept.


Run libbpf fuzz tests (needs a checkout)

```bash
make -C tests fuzz_libbpf LIBBPF_CHECKOUT_DIR=/path/to/libbpf
```
