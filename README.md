This is a proof-of-concept of exfiltrating secrets from a developer's laptop _at compile-time_.

The `bad_actor` crate exposes a `make_answer!` macro which is used by the `innocent_app` crate. When the latter is built, it can exfiltrate secrets:

```shell
[jared@localhost innocent_app]$ cargo build
   Compiling bad_actor v0.1.0 (/home/jared/Projects/bad_actor_poc/bad_actor)
   Compiling innocent_app v0.1.0 (/home/jared/Projects/bad_actor_poc/innocent_app)
Your aws credentials are: <<<REDACTED>>>

    Finished dev [unoptimized + debuginfo] target(s) in 1.11s
```