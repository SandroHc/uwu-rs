# uwu

A uwu implementation inspired by [Daniel Liu's uwu](https://github.com/Daniel-Liu-c0deb0t/uwu) that runs on all machines
and on the web.

This repository is divided into the following crates:

* [uwu-rs](https://crates.io/crates/uwu-rs) - the library
* [uwu_cli](https://github.com/SandroHc/uwu/tree/master/crates/uwu_cli) - the CLI tool
* [uwu_wasm](https://npmjs.com/package/uwu-rs) - the WebAssembly/WASM bundle for the web
* [www](https://uwu.sandro.dev) - the web version

## Quick start

### For Rust projects

Add the uwu library to your project with:

```shell
cargo add uwu-rs
```

Then use it:

```rust
let uwuified = uwu::Uwu::new().uwuify("Hello world!");
```

### For JavaScript projects

Add the uwu library to your project with:

```shell
npm install uwu-rs
```

Then use as follows in your code:

```javascript
import init, {uwuify} from 'uwu-rs';

init().then(() => {
    console.log(uwuify('I have been uwuified!'));
});
```

### For the CLI

Install the executable with:

```shell
cargo install uwu_cli
```

Then use it:

```shell
# directly from the arguments
uwu Hello world!

# from a file
uwu --file uwu.txt

# from stdin
uwu <<EOF
Life in uwu land,
Is fantastic!
EOF

# to a file
uwu --output uwu.out Hello world!
```
