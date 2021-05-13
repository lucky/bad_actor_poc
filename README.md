This is a proof-of-concept of exfiltrating secrets from a developer's laptop.
When `innocent_app` is opened in VSCode, for example, the editor runs enough
of the Rust toolchain to expand the nefarious `make_answer!` macro, which opens
`.ssh/id_rsa` and sends its contents to `localhost:8080`.

**This shows a trivial example of exfiltrating secrets just by the developer
opening up the source code in an editor.**

To test:

 * Listen on port 8080 locally, for example with `nc -lk 8080`
 * Open up the `innocent_app` in VSCode\* (other editors currently untested)

Once open, VSCode will analyze and index the code, including the expansion of
macros, then you should see the contents of your `.ssh/id_rsa` private key in
the `nc` window.

*This assumes you have the rust toolchain available on your machine.
