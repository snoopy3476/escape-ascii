# Escape ASCII

![binary-example-0.png](https://raw.githubusercontent.com/snoopy3476/escape-ascii/main/assets/binary-example-0.jpg)
![binary-example-1.png](https://raw.githubusercontent.com/snoopy3476/escape-ascii/main/assets/binary-example-1.jpg)

An extremely tiny tool to convert stdin raw byte stream to printable escaped-ascii form, such as byte literals in source codes on many programming languages (e.g. `"\x1b\x04printable\x16\n"`).

This tool just help calling the rust function [slice::escape_ascii](https://doc.rust-lang.org/std/primitive.u8.html#method.escape_ascii) directly from command line.  
For more information about how this tool converts, check the rust official documentations of [std::ascii::escape_default](https://doc.rust-lang.org/std/ascii/fn.escape_default.html).



## Install

```shell
$ cargo install escape-ascii
```



## Example

- Get random 10 bytes and convert to escaped-ascii form
```shell
$ BYTES="$(cat /dev/urandom | head -c10)"
$ printf "%s" "$BYTES"; echo
��f�YڵH
$ printf "%s" "$BYTES" | escape-ascii; echo
\x8a\x8bf\xa1\x8f\x08Y\xda\xb5H
```



## Author
Kim Hwiwon \<kim.hwiwon@outlook.com\>



## License
The MIT License (MIT)
