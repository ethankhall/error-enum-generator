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

#[derive(Debug, PartialEq, Eq, ErrorEnum)]
#[error_enum(prefix = "RNT")]
pub enum RuntimeErrors {
    #[error_enum(description = "This was a bug")]
    Bug(String),
    #[error_enum(description = "Panic!")]
    Panic,
    #[error_enum(description = "Error")]
    Error { message: String },
}

#[cfg(test)]
mod test {
    use super::*;

    fn return_runtime_error() -> Result<(), RuntimeErrors> {
        Err(RuntimeErrors::Panic)
    }

    fn will_translate_to_cli_error() -> Result<(), CliErrors> {
        return_runtime_error()?;
        unreachable!();
    }

    #[test]
    fn validate_error_cast() {
        assert_eq!(will_translate_to_cli_error(), Err(CliErrors::Runtime(RuntimeErrors::Panic)));
    }
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

    // Verify counts work
    let error = CliErrors::Config(ConfigErrors::ConfigParseError("missing foo".to_string()));
    assert_eq!("CFG-002", error.get_error_code());
    assert_eq!(
        error.description(), 
        "Unable to parse config file."
    );
    assert_eq!(
        format!("{}", error), 
        "(CFG-002): Unable to parse config file. Detailed Error: \"missing foo\""
    );

    // --- Runtime Errors
    let error = CliErrors::Runtime(RuntimeErrors::Bug("my bad...".to_string()));
    assert_eq!("RNT-001", error.get_error_code());
    assert_eq!(
        error.description(), 
        "This was a bug."
    );
    assert_eq!(
        format!("{}", error), 
        "(RNT-001): This was a bug. Detailed Error: \"my bad...\""
    );

    // Without a param
    let error = CliErrors::Runtime(RuntimeErrors::Panic);
    assert_eq!("RNT-002", error.get_error_code());
    assert_eq!(
        error.description(), 
        "Panic!."
    );
    assert_eq!(
        format!("{}", error), 
        "(RNT-002): Panic!."
    );

    // Was a C struct
    let error = CliErrors::Runtime(RuntimeErrors::Error { message: "some message".to_string() });
    assert_eq!("RNT-003", error.get_error_code());
    assert_eq!(
        error.description(), 
        "Error."
    );
    assert_eq!(
        format!("{}", error), 
        "(RNT-003): Error. Detailed Error: \"some message\""
    );
}
