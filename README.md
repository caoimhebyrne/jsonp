# `jsonp`

A simple JSON parser written in Rust.

I'm designing this to pretty print some JSON files, but in the future, I'd like to be able to make a more usable alternative to `jq`. (Let's be real, who can say that they're confident with using `jq`!?)

## Usage

```shell
$ cargo install -- path .
  Installing jsonp v0.1.0
    Finished release [optimized] target(s) in 0.00s
  Installing ~/.cargo/bin/jsonp
    Installed package `jsonp v0.1.0` (executable `jsonp`)

$ jsonp ./examples/simple.json
Object(
    {
        "exponential": Number(23.59),
        "abc": Number(123.0),
        "hello": String("world"),
        "test": Object(
            {
                "value": Null,
                "boolean": Boolean(true),
                "array": Array(
                    [
                        Number(1.0),
                        Number(2.0),
                        Number(3.0),
                    ],
                ),
                "another_boolean": Boolean(false),
            },
        ),
        "decimal": Number(123.55435),
    },
)
```

## License

This project is licensed under the [MIT license](https://choosealicense.com/licenses/mit).
