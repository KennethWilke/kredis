use anyhow::Result;
use kresp::RespType;
use crate::{Redis, error::RedisError};

impl Redis {
    // COPY

    pub async fn del(&mut self, keys: &[&[u8]]) -> Result<i64> {
        let mut command = vec!["DEL".as_bytes()];
        for key in keys {
            command.push(key);
        }
        match self.command(command).await? {
            RespType::Integer(i) => Ok(i),
            _ => Err(RedisError::ProtocolError.into())
        }
    }

    // DUMP

    pub async fn exists(&mut self, keys: &[&[u8]]) -> Result<i64> {
        let mut command = vec!["EXISTS".as_bytes()];
        for key in keys {
            command.push(key);
        }
        match self.command(command).await? {
            RespType::Integer(i) => Ok(i),
            _ => Err(RedisError::ProtocolError.into())
        }
    }

    // EXPIRE

    // EXPIREAT

    // EXPIRETIME

    // KEYS

    // MIGRATE

    // MOVE

    // OBJECT?

    // PERSIST

    // PEXPIRE

    // PEXPIREAT

    // PEXPIRETIME

    // PTTL

    // RANDOMKEY

    // RENAME

    // RENAMENX

    // RESTORE

    // SCAN

    // SORT

    // SORT_RO

    // TOUCH

    // TTL

    // TYPE

    // UNLINK

    // WAIT
}
