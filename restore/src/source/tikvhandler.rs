use tikv_client::{Key, RawClient};

pub struct TiKVHandler {
    tikv_client: RawClient,
}