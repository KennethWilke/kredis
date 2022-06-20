use anyhow::Result;
use kredis::Redis;
use kresp::RespType;

#[tokio::test]
async fn connection() -> Result<()> {
    let mut redis = Redis::connect("tcp://localhost:6379").await?;
    redis.ping().await?;
    Ok(())
}

#[tokio::test]
async fn strings_basic() -> Result<()> {
    let mut redis = Redis::connect("tcp://localhost:6379").await?;
    let key = "kredis-integration-test:string";
    let value = "what say you, integration test?".as_bytes();

    // First, check if this key lingered from a previous, failed test
    let key_count = redis.exists(&[key.as_bytes()]).await?;
    if key_count == 1 {
        println!("Test key {} still exists, will attempt delete", key);
        let deleted = redis.del(&[key.as_bytes()]).await?;
        assert_eq!(deleted, 1);
    }

    redis.set(key.as_bytes(), value).await?;
    match redis.get(key.as_bytes()).await? {
        RespType::BulkString(string) => {
            assert_eq!(string, value);
        }
        _ => {
            panic!("Expected BulkString response");
        }
    }

    let key_count = redis.exists(&[key.as_bytes()]).await?;
    assert_eq!(key_count, 1);

    let deleted = redis.del(&[key.as_bytes()]).await?;
        assert_eq!(deleted, 1);

    let key_count = redis.exists(&[key.as_bytes()]).await?;
    assert_eq!(key_count, 0);
    Ok(())
}
