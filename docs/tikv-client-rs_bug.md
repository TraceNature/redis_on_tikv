# tikv-client-rs bugs

* RawClient::new(pd_endpoints, None) 当log为全局配置时扔有日志输出
* 当put_with_ttl时，若key已存在，ttl不生效