use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("received unexpected reply from server")]
    ProtocolError
}
