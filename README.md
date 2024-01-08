# keypropdecode
A library for decoding windows file system element properties.  
Since Windows stores these properties as a number and each individual property is stored in a determined bit of that number, decoding it can bloat the code.  
This library attemps to solve this.
You can use this crate with different purposes:
1. You can use the unsigned 32 bit integer with `keypropdecode::from_number()` to get a handy struct with all the properties that number contains
2. You can use a `PathBuf` with `keypropdecode::from_file()` to get the properties without needing to extract the properties integer manually
3. Create an empty struct to change the properties as you please and get the integer with the properties you asigned with `Self.as_number()`
The `Display` implementation of the struct return a `String` identical as the one that prints with `GetChild-Item` in PowerShell, which are the most commonly used.  
For reference with all the file system element properties go to the [Microsoft File Attribute Constants Documentation](https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants)
## Example
```Rust
use keypropdecode::Props;
let mut props = Props::default();
props.archive(true).unwrap();
props.hidden(true);
assert_eq!(Props::try_from(r"hidden_file_example.txt").unwrap(), props);
```