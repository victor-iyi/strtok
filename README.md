# Split string into tokens

![CI](https://github.com/victor-iyi/strtok/workflows/CI/badge.svg)
![Audit](https://github.com/victor-iyi/strtok/workflows/Audit/badge.svg)

`fn strtok(string: &str, delimiters: char) -> &str;`

A sequence of calls to this function split `string` into tokens, which are sequence
of contiguous characters separated by any of the characters that are part of `delimiters`.

On a first call, the function expects a `&str` as argument for `string`, whose
first character is used as the starting location to scan tokens. In subsequent
calls, the function expects a null pointer and uses the position right after the
end of the last token as the new starting location for scanning.

To determine the beginning and the end of a token, the efunction first scans from
the starting location for the first charcter **not** contained in `delimiters`
(which becomes the *beginning of the token*). And then scans starting from this
*beginning of the token* for the first character contained in `delimiters`,
which becomes the *end of the token*. The scan also stops when the end of the
`string` is reached.

The *end of token* is automatically replaced by a null-character, and the
*beginning of the token* is returned by the function.

The point where the last token was found is kept internally by the function to
be used on the next call (particular library implemenations are not required to
avoid data races).

## Parameters

`string` - `&str`
  Notice that this string is modified by being broken into smaller strings (tokens)
  Alternatively, a null pointer may be specified, in which case the function
  continues scanning where a previous successful call to the function ended.

`delimiters` - `&str`
  String containing the delimiter characters.
  These can be different from one call to another.

## Return Vale

If a token is found, a pointer to the beginning of the token.
Otherwise, a *null pointer*.

A *null pointer* is always returned when the end of the string (i.e., a null
character) is reached in the string being scanned.

## Example

```rust
use strtok::strtok;

fn main() {
  let mut hello_world: &mut &str = "Hello World!";

  let hello = strtok(&mut hello_world, ' ');

  println!("hello = {}", &hello);
  println!("hello_world = {}", &hello_world);

  assert_eq!(hello, "Hello");
  assert_eq!(hello_world, "world!");
}
```

## Contribution

You are very welcome to modify and use them in your own projects.

Please keep a link to the [original repository]. If you have made a fork with substantial modifications that you feel may be useful, then please [open a new issue on GitHub][issues] with a link and short description.

[original repository]: https://github.com/victor-iyi/strtok
[issues]: https://github.com/victor-iyi/strtok/issues

## License (MIT)

This project is opened under the [MIT License][license] which allows very broad use for both private and commercial purposes.

A few of the images used for demonstration purposes may be under copyright. These images are included under the "fair usage" laws.

[license]: ./LICENSE
