# Escape ASCII

An extremly tiny tool to convert stdin byte stream to printable escaped-ascii form, such as byte literals in source codes on many programming languages (e.g. `"\x1b\x04printable\x16\n"`).

This tool just help calling the function [slice::escape_ascii](https://doc.rust-lang.org/std/primitive.u8.html#method.escape_ascii) directly from command line.  
For more information about how this tool converts, check the official documentations of [std::ascii::escape_default](https://doc.rust-lang.org/std/ascii/fn.escape_default.html).

## Install

```shell
$ cargo install escape-ascii
```

## Example

- Get random 10 bytes and convert to escaped-ascii form
```shell
$ BYTES="$(cat /dev/urandom | head -c10)"
$ printf "%s" "$BYTES"; echo
�"�d�֐�
$ printf "%s" "$BYTES" | escape-ascii; echo
\xbd\"\x18\xc6d\xaf\x04\xd6\x90\xa2
```
