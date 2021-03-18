/// A sequence of calls to this function split `string` into tokens, which
/// are sequences of contiguous characters separated by any of the characters
/// that are part of `delimiter`.
///
/// # Parameters
///
/// `string` - String to truncate. Notice that the string is modified
///             by being broken into smaller strings (tokens).
///
/// `delimiter` - Containing the delimiter characters. These can be different
///               from one call to another.
///
/// # Return Value
///
/// If a token is found, a pointer to the beginning of the token, otherwise
/// an empty string. An empty string is always returned when the end of the
/// string is reached in the string being scanned.
///
/// # Example
///
/// ```rust
/// # use strtok::strtok;
///
/// # fn main() {
/// let mut hello_world = "Hello world!";
/// let hello = strtok(&mut hello_world, ' ');
///
/// assert_eq!(hello, "Hello");
/// // assert_eq!(hello_world, "world");
/// # }
/// ```
pub fn strtok<'a>(string: &'_ mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = string.find(delimiter) {
        let prefix = &string[..i];
        let suffix = &string[(i + delimiter.len_utf8())..];
        *string = suffix;
        prefix
    } else {
        let prefix = *string;
        *string = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strtok() {
        let mut hello_world = "Hello world!";
        let hello = strtok(&mut hello_world, ' ');

        assert_eq!(hello, "Hello");
        assert_eq!(hello_world, "world!");
    }
}
