enum DecoderStatus {
    NeedMoreBytes,
    SentenceReady(Vec<u8>),
    ChecksumError(u16, u16),
    SyntaxError,
}
