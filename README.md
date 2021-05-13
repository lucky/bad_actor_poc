# TL;DR

Open `innocent_app` in [VSCode](https://code.visualstudio.com/)*, and the
contents of your `.ssh/id_rsa` file will be sent over TCP to
`localhost:8080`. **You don't even need to open any files in the project!**

*This assumes you have the rust toolchain available on your machine, and the
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
VSCode plugin.

# Exfiltrating secrets with Rust macros

This is a proof-of-concept of exfiltrating secrets from a developer's machine.
Originally, the target was exfiltrate at compile-time, but it became apparent it
was possible even before that step, i.e. during pre-processing.

## How it works

When `innocent_app` is opened in VSCode, the editor runs enough of the Rust
toolchain to expand the nefarious `make_answer!` macro, which opens
`.ssh/id_rsa` and sends its contents to `localhost:8080`.

# Run it yourself

* Clone this repo: `git clone https://github.com/lucky/bad_actor_poc.git`
* Listen on port 8080 locally, for example with `nc -lk 8080`
* Open up the `innocent_app` in VSCode with rust-analyzer plugin

Once open, VSCode will analyze and index the code, including the expansion of
macros, then you should see the contents of your `.ssh/id_rsa` private key in
the `nc` window.

You can trigger the same behavior at compile-time by running `cargo build`
in the `innocent_app` directory.

## Notes

**This may affect other editors.** VSCode and rust-analyzer were used to confirm
the attack vector, but are not exactly responsible for them. Any editor that
expands a proc macro can do this.

**There may be similar attacks for other languages.** For example, it may be
possible to attack Java annotation processing.