# Error Enum Generator

This tool is used to automatically generate error codes, and messages
for an enum. The major intent of this is to make error in the CLI
easier to generate.

## Example Usage

```rust
use error_enum::{ErrorEnum, ErrorContainer, PrettyError};

#[derive(Debug, PartialEq, Eq, ErrorContainer)]
pub enum CliErrors {
    Config(ConfigErrors),
    Runtime(RuntimeErrors),
}

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "CFG")]
pub enum ConfigErrors {
    #[error_enum(description = "Unable to Load File from disk")]
    UnableToLoadFile(String),
    #[error_enum(description = "Unable to parse config file")]
    ConfigParseError(String)
}

#[test]
fn verify_display() {
    let error = CliErrors::Config(ConfigErrors::UnableToLoadFile("/foo/path".to_string()));
    assert_eq!("CFG-001", error.get_error_code());
    assert_eq!(
        error.description(), 
        "Unable to Load File from disk."
    );
    assert_eq!(
        format!("{}", error), 
        "(CFG-001): Unable to Load File from disk. Detailed Error: \"/foo/path\""
    );
}
```