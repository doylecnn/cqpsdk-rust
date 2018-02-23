# Unofficial CQP SDK for Rust

This project is a SDK for [CQP](https://cqp.cc/) written in [Rust](http://rust-lang.org/).

The officially recognized Rust SDK is done by `doylecnn` and is linked in the Relative Projects section below.

## How To Use

Add following lines to your `Cargo.toml`:

```

[dependencies.cqpsdk]
git = "https://github.com/evshiron/cqpsdk-rust"

```

For more detailed information you can refer to [evshiron/PupuriumR](https://github.com/evshiron/PupuriumR/) as a demo.

## Documentation

You can generate some documentation locally with the following commands:

```bash

rustdoc ./src/mod.rs

# Open the generated pages in Mac OS X.
open ./doc/mod/index.html

```

Besides, the [evshiron/PupuriumR Wiki](https://github.com/evshiron/PupuriumR/wiki) might worth reading.

## Relative Projects

  * [doylecnn/cqpsdk-rust](https://github.com/evshiron/PupuriumR/)
    * [doylecnn/cqpsdk-rust-demo](https://github.com/doylecnn/cqpsdk-rust-demo)

## Thanks

Many thanks to `doylecnn` and `Coxxs`, without whom I can't make this "another" SDK happen.

## Disclaimer

I participated with the official Rust SDK with `doylecnn`, but we seperated because he wanted to make the code more of Rust style, while I love the raw C style API.

## The MIT License

Copyright © 2015 evshiron

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
