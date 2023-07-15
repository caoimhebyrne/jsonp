# `jsonp`

A simple JSON parser written in Rust.

I'm designing this to pretty print some JSON files, but in the future, I'd like to be able to make a more usable alternative to `jq`. (Let's be real, who can say that they're confident with using `jq`!?)

## Usage

At the moment, `jsonp` is configured to read from `./examples/simple.json`

```
$ cargo run

Object(
    {
        "abc": Number(
            123,
        ),
        "test": Object(
            {},
        ),
        "hello": String(
            "world",
        ),
    },
)
```

## License

This project is licensed under the [MIT license](https://choosealicense.com/licenses/mit).
