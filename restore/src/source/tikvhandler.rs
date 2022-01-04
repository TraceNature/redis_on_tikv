use tikv_client::{Key, KvPair, RawClient, Error, Value, IntoOwnedRange};

pub enum KvPairError {
    OptionError(String),
}

pub type Result<T, E = KvPairError> = std::result::Result<T, E>;

pub struct TiKVHandler {
    tikv_client: RawClient,
}

impl TiKVHandler {
    pub async fn new(pd_endpoints: Vec<&str>) -> Self {
        Self {
            tikv_client: RawClient::new(pd_endpoints, None).await.unwrap()
        }
    }

    pub async fn tikv_put(&self, key: String, val: String) -> tikv_client::Result<()> {
        println!("invoke put");
        self.tikv_client.put(Key::from(key), Value::from(val)).await
    }

    pub async fn tikv_remove(&self, key: String) -> tikv_client::Result<()> {
        self.tikv_client.delete(Key::from(key)).await
    }

    pub async fn tikv_remove_all(&self) -> tikv_client::Result<()> {
        // self.tikv_client.delete(Key::from(key)).await
        let range = "".."";
        self.tikv_client.delete_range(range.into_owned()).await
    }

    pub async fn tikv_get(&self, key: String) -> tikv_client::Result<Option<Value>> {
        self.tikv_client.get(key.to_owned()).await
    }

    pub async fn tikv_get_ttl_sec(&self, key: String) -> tikv_client::Result<Option<u64>> {
        self.tikv_client.get_key_ttl_secs(key.to_owned()).await
    }


    pub async fn tikv_put_with_ttl(&self, key: String, val: String, ttl: u64) -> tikv_client::Result<()> {
        self.tikv_client.put_with_ttl(Key::from(key), Value::from(val.as_str()), ttl).await
    }

    pub async fn prefix_scan(&self, start: String, end: String, limited: u32) -> Result<Vec<KvPair>> {
        let range = start..end;
        self.tikv_client.scan(range, limited).await.map_err(|e| {
            return KvPairError::OptionError(e.to_string());
        })
    }
}