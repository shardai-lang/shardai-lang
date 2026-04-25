<div align="center">
    <img src="assets/logo-dark.png#gh-light-mode-only" alt="Shardai" width="450">
    <img src="assets/logo-light.png#gh-dark-mode-only" alt="Shardai" width="450">
    <br/><br/>
    <a href="LICENSE"><img src="https://img.shields.io/github/license/shardai-lang/shardai-lang"/></a>
    <a><img src="https://img.shields.io/badge/status-pre--alpha-orange"/></a>
</div>

Shardai is a managed programming language focused on speed, simplicity, and first-class developer experience.
Shardai is still heavily in the works. Feel free to check back later!

## Example
```
func fizzbuzz() {
    for i = 1, 100 {
        if i % 3 == 0 and i % 5 == 0 {
            print("FizzBuzz")
        } else if i % 3 == 0 {
            print("Fizz")
        } else if i % 5 == 0 {
            print("Buzz")
        } else {
            print(i)
        }
    }
}
```

## Status
Very early development. Expect breaking changes until 1.0.

## License
Licensed under the Apache License Version 2.0.

## Contributing
Check out the [contributing guide](CONTRIBUTING.md).