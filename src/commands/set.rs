use anyhow::Result;
use kresp::RespType;
use crate::{Redis, error::RedisError};

impl Redis {
    pub async fn sadd(&mut self, key: &[u8], values: &[&[u8]]) -> Result<i64> {
        let command = "SADD".as_bytes();
        let mut command = vec![command, key];
        for value in values {
            command.push(value);
        }
        match self.command(command).await? {
            RespType::Integer(i) => Ok(i),
            _ => Err(RedisError::ProtocolError.into())
        }
    }

    // SCARD

    // SDIFF

    // SDIFFSTORE

    // SINTER

    // SINTERCARD

    // SINTERSTORE

    // SISMEMBER

    pub async fn smembers(&mut self, key: &[u8]) -> Result<RespType> {
        let command = "SMEMBERS".as_bytes();
        self.command(vec![command, key]).await
    }

    // SMISMEMBER

    // SMOVE

    // SPOP

    // SRANDMEMBER

    // SREM

    // SSCAN

    // SUNION

    // SUNIONSTORE
}
