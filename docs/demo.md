# Demo  演示步骤

## 启动redis2tikv 同步服务

## 通过redis客户端写入数据

redis-cli -h 114.67.76.82 -p 16375

set a a
set b b
set c c
sadd set1 1
sadd set1 2
sadd set1 3

smembers set1

## 停止同步服务

## 清空redis实例

## 执行restore 恢复redis 数据
