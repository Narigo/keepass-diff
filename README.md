# keepass-diff

This CLI-tool diffs two Keepass (.kdbx) files and prints their differences.

Usage:

```
cargo run <file-a> <file-b>
```

The CLI will ask for the password for both files individually.

![Example Screencast](docs/screencast.gif)

## Used libraries:

* [clap](https://github.com/clap-rs/clap) to read command line arguments.
* [rpassword](https://github.com/conradkdotcom/rpassword) to read the passwords.
* [keepass](https://github.com/sseemayer/keepass-rs) to read `.kdbx` files.
* [termcolor](https://github.com/BurntSushi/termcolor) to print with colors.

Password for the Keepass demo files: `demopass`

## Contributing

Care to help? I'm pretty new to Rust, so if anyone likes to help or teach me 
cool stuff, please reach out!
