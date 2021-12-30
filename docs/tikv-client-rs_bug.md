# tikv-client-rs bugs

* RawClient::new(pd_endpoints, None) 当log为全局配置时扔有日志输出
* 当put_with_ttl时，若key已存在，ttl不生效

# tidb 安全漏洞

当pd和tidb暴露在公网时
Prometheus未授权访问漏洞，地址：http://114.67.120.120:9090/metrics 
存在Golang开放ppof调试接口，地址：http://114.67.120.120:2379/debug/pprof/
存在Golang开放ppof调试接口，地址：http://114.67.120.120:9090/debug/pprof/