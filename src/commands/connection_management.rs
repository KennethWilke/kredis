use anyhow::Result;
use kresp::RespType;
use crate::{Redis, error::RedisError};

impl Redis {
    // AUTH

    // CLIENT?

    // ECHO

    // HELLO

    pub async fn ping(&mut self) -> Result<()> {
        self.connection.send(b"PING\r\n").await?;
        let reply = self.read().await?;
        let reply = match reply.get(0) {
            Some(reply) => match reply {
                RespType::SimpleString(reply) => reply,
                _ => return Err(RedisError::ProtocolError.into())
            },
            None => return Err(RedisError::ProtocolError.into())
        };
        if reply != "PONG" {
            Err(RedisError::ProtocolError.into())
        } else {
            Ok(())
        }
    }

    // QUIT?

    // RESET?

    // SELECT
}
