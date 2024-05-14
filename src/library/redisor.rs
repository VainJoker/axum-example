use deadpool_redis::{Connection, Pool, Runtime};
use redis::AsyncCommands;

use crate::library::{
    cfg,
    error::{InnerResult, RedisorError},
};

pub type Redis = Connection;
pub struct Redisor {
    pub pool: Pool,
}

impl Redisor {
    pub fn init() -> Self {
        let cfg = cfg::config();
        let redis_url = cfg.app.redis_url.clone();
        let deadpool = deadpool_redis::Config::from_url(redis_url);
        match deadpool.create_pool(Some(Runtime::Tokio1)) {
            Ok(pool) => {
                tracing::info!("🚀 Connection to the redis is successful!");
                Self { pool }
            }
            Err(err) => {
                panic!("💥 Failed to connect to the redis: {err:?}");
            }
        }
    }

    pub async fn get_conn(&self) -> InnerResult<Redis> {
        Ok(self.pool.get().await.map_err(RedisorError::PoolError)?)
    }

    pub async fn get(&self, key: &str) -> InnerResult<Option<String>> {
        let mut conn = self.get_conn().await?;
        let result: Option<String> =
            conn.get(key).await.map_err(RedisorError::ExeError)?;
        Ok(result)
    }

    pub async fn set(&self, key: &str, value: &str) -> InnerResult<()> {
        let mut conn = self.get_conn().await?;
        conn.set::<_, _, ()>(key, value)
            .await
            .map_err(RedisorError::ExeError)?;
        Ok(())
    }

    pub async fn del(&self, key: &str) -> InnerResult<()> {
        let mut conn = self.get_conn().await?;
        conn.del::<_, ()>(key)
            .await
            .map_err(RedisorError::ExeError)?;
        Ok(())
    }

    pub async fn set_ex(
        &self,
        key: &str,
        value: &str,
        ttl: u64,
    ) -> InnerResult<()> {
        let mut conn = self.get_conn().await?;
        conn.set_ex::<_, _, ()>(key, value, ttl)
            .await
            .map_err(RedisorError::ExeError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use deadpool_redis::redis::cmd;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_redisor_init() {
        cfg::init(&"./fixtures/config.toml".to_string());
        let redis_url = cfg::config().app.redis_url.clone();
        let deadpool = deadpool_redis::Config::from_url(redis_url);
        let pool = deadpool.create_pool(Some(Runtime::Tokio1)).unwrap();
        let redisor = Redisor { pool };
        let mut conn = redisor.get_conn().await.unwrap();
        cmd("SET")
            .arg(&["deadpool/test_key", "42"])
            .query_async::<_, ()>(&mut conn)
            .await
            .unwrap();
        // conn.set("key","value").await.unwrap();
        // assert_eq!(conn.get::<&str,String>("key").await.unwrap(), "value");
    }
}
