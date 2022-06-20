use anyhow::Result;
use kresp::{RespType, RespParser};
use multistream::Client;

use crate::error::RedisError;

pub struct Redis {
    pub connection: Client,
    parser: RespParser
}

impl Redis {
    pub async fn connect<T>(host: T) -> Result<Self> 
    where T: ToString {
        let host = host.to_string();
        let connection = Client::connect(host).await?;

        Ok(Redis{ connection, parser: RespParser::default() })
    }

    pub async fn read(&mut self) -> Result<Vec<RespType>> {
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
}
