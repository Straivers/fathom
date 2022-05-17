#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The provided session token has either expired, or is otherwise invalid.
    #[error("the provided token is invalid, it may have expired")]
    InvalidToken,

    /// The provided user credentials are not valid.
    #[error("the provided user credentials are invalid")]
    InvalidCredential,
}

pub struct Token {
    pub key: u128,
}

pub trait Api: Sync + Send {
    /// Begins a user session by verifying the user's username and password.
    ///
    /// # Errors
    ///
    /// May return an `InvalidCredential` error if the username, password, or
    /// both are invalid.
    fn auth(&self, username: &str, password_hash: u128) -> Result<Token, Error>;

    /// Gets the user ID associated with the session token.
    ///
    /// # Errors
    ///
    /// May return an `InvalidToken` error if the token has expired, or is
    /// otherwise invalid.
    fn user(&self, token: Token) -> Result<u128, Error>;
}
