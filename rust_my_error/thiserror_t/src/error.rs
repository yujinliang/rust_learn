use thiserror::Error;

/// WordCountError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum WordCountError {
    /// Represents an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    #[error("Source contains no data")]
    EmptySource,

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { source: std::io::Error },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

//对于库的外部使用者只暴露统一的WordCountError, 任何库底层爆出的错误，如:std::io::Error等都会被封装成
//WordCountError中定义的相应高级错误Case。 这样库的用户就可明确地区分出错误出自那个库。