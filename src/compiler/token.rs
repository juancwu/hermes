pub enum Token {
    /// Represents illegal characters that shouldn't be in the .hermes syntax
    Illegal,
    MetadataBlock,
    RouteBlock,
    HeadersBlock,
}
