# minihead

## Overview
linuxのheadコマンドをRustで書き換えたプログラムです。
オプションには、-n,-cが使用できます。

## Requirement
- WSL2
- cargo

## Usage
```
$ git clone ...
$ cd minihead
$ cargo build --release
$ ./target/release/minihead sample.txt -n 3
Hello World!
thank you!

$ ./target/release/minihead sample.txt -c 5
Hello$
```

## About debug
debugモードでのbuildは以下の手順で行えます。
```
git clone ...
cd minihead
cargo build
```

## Reference
WIP

## Author
[twitter](https://twitter.com/anto_tohoku)

## Licence
[MIT](https://github.com/tm-hack/minihead/LICENCE)
