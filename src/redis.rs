use anyhow::Result;
use kresp::{RespType, RespParser};
use multistream::Client;

use crate::error::RedisError;

pub struct Redis {
    connection: Client,
    parser: RespParser
}

impl Redis {
    pub async fn connect<T>(host: T) -> Result<Self> 
    where T: ToString {
        let host = host.to_string();
        let connection = Client::connect(host).await?;

        Ok(Redis{ connection, parser: RespParser::default() })
    }

    async fn read(&mut self) -> Result<Vec<RespType>> {
        let reply = self.connection.recv().await?;
        self.parser.read(&reply)
    }

    pub async fn command(&mut self, command: Vec<&[u8]>) -> Result<RespType> {
        let mut request = Vec::new();
        for i in command {
            request.push(RespType::BulkString(i.into()))
        };
        self.connection.send(&RespType::Array(request).as_bytes()).await?;
        match self.read().await?.pop() {
            Some(result) => Ok(result),
            None => Err(RedisError::ProtocolError.into())
        }
    }

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

    pub async fn get(&mut self, key: &[u8]) -> Result<RespType> {
        let command = "GET".as_bytes();
        self.command(vec![command, key]).await
    }

    pub async fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        let command = "SET".as_bytes();
        self.command(vec![command, key, value]).await?;
        Ok(())
    }
}
