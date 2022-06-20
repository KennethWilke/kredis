use anyhow::Result;
use kresp::RespType;
use crate::{Redis, error::RedisError};

impl Redis {
    pub async fn append(&mut self, key: &[u8], value: &[u8]) -> Result<bool> {
        let command = "APPEND".as_bytes();
        match self.command(vec![command, key, value]).await? {
            RespType::Integer(1) => Ok(true),
            RespType::Integer(0) => Ok(false),
            _ => Err(RedisError::ProtocolError.into())
        }
    }

    // DECR

    // DECRBY

    pub async fn get(&mut self, key: &[u8]) -> Result<RespType> {
        let command = "GET".as_bytes();
        self.command(vec![command, key]).await
    }

    // GETDEL

    // GETEX

    // GETRANGE

    // GETSET

    // INCR

    // INCRBY

    // INCRBYFLOAT

    // LCS

    // MGET

    // MSET

    // MSETNX

    // PSETEX

    pub async fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        let command = "SET".as_bytes();
        self.command(vec![command, key, value]).await?;
        Ok(())
    }

    // SETEX

    // SETNX

    // SETRANGE

    // STRLEN

    // SUBSTR
}
