use anyhow::Result;
use kresp::RespType;
use crate::{Redis, error::RedisError};

impl Redis {
    pub async fn hdel(&mut self, key: &[u8], field: &[u8]) -> Result<bool> {
        let command = "HDEL".as_bytes();
        match self.command(vec![command, key, field]).await? {
            RespType::Integer(1) => Ok(true),
            RespType::Integer(0) => Ok(false),
            _ => Err(RedisError::ProtocolError.into())
        }
    }

    pub async fn hexists(&mut self, key: &[u8], field: &[u8]) -> Result<bool> {
        let command = "HEXISTS".as_bytes();
        match self.command(vec![command, key, field]).await? {
            RespType::Integer(1) => Ok(true),
            RespType::Integer(0) => Ok(false),
            _ => Err(RedisError::ProtocolError.into())
        }
    }

    pub async fn hget(&mut self, key: &[u8], field: &[u8]) -> Result<RespType> {
        let command = "HGET".as_bytes();
        self.command(vec![command, key, field]).await
    }

    // HGETALL

    // HINCRBY

    // HINCRBYFLOAT

    // HKEYS

    // HLEN

    // HMGET

    // HMSET

    // HRANDFIELD

    // HSCAN

    pub async fn hset(&mut self, key: &[u8], field: &[u8], value: &[u8]) -> Result<i64> {
        let command = "HSET".as_bytes();
        match self.command(vec![command, key, field, value]).await? {
            RespType::Integer(i) => Ok(i),
            _ => Err(RedisError::ProtocolError.into())
        }
    }

    pub async fn hsetnx(&mut self, key: &[u8], field: &[u8], value: &[u8]) -> Result<bool> {
        let command = "HSETNX".as_bytes();
        match self.command(vec![command, key, field, value]).await? {
            RespType::Integer(1) => Ok(true),
            RespType::Integer(0) => Ok(false),
            _ => Err(RedisError::ProtocolError.into())
        }
    }

    // HSTRLEN

    // HVALS
}
