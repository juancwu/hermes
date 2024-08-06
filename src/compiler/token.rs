#[derive(Debug, Clone)]
pub enum Token {
    /// Represents illegal characters that shouldn't be in the .hermes syntax
    Illegal,
    EOF,
    /// This is a data block and it can be a key block that is reserved, or it is custom defined.
    /// A block is defined by the (type, identier) where the identifier can be an empty string for
    /// annonynous block.
    ///
    /// Available reserved blocks:
    /// metadata
    /// request
    /// container
    /// folder
    /// headers
    /// variables
    /// environment
    /// body
    /// queries
    Block(String, String),
    /// Typical identifier in any language. This will mostly just be
    /// block names that are used to reference to defined blocks or for reserved keywords.
    ///
    /// Keep in mind that identifier keywords only appear at the beginning of any line in a block.
    ///
    /// Available identifier keywords:
    /// type - type of hermes file, usually defined in a metadata block
    /// name - the type of a collection, request, or folder
    /// text - text type of multipart form data field
    /// file - file type of multipart form data field
    /// environment - use an enviroment
    /// add - add a single request
    /// include - include all requests from a given path
    Identifier(String),
    /// The SelectorOperator refers to the two colons "::".
    SelectorOperator,
    /// Refers to any raw value read from a hermes file. For example, the JSON body string would be
    /// a raw value, as well as the value of a query parameter.
    RawValue(String),
    /// Some blocks such as headers, queries, form-urlencoded, and mutipart-form can have enabled
    /// fields which are included in the request.
    StateEnabled,
    /// Some blocks such as headers, queries, form-urlencoded, and mutipart-form can have disabled
    /// fields which are included in the request.
    StateDisabled,
    CurlyLeft,
    CurlyRight,
    DoubleColon,
}
