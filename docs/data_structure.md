# 数据结构

* instances 用于防止任务用于实例名相同带来的数据混淆。每次新建相关任务会查询是否有重名
  instances：[instancename,...]

* 头信息用于描述key的属性

  ```
  ^instancename^dbname^keytyp^keyname
  ```

//Todo
//待定 开头字符以及位分隔符
## redis 数据结构到 TiKV 的映射关系

总体原则 redis key => instancename^db^type^description^keyname

### string

 *{instId}_{dbNum}_{commandType}_{keyName}

### list

* key格式
  instancename^db^l^keyname^ifrevers^index
  
  *{instId}_{dbNum}_{commandType}_{keyName}_{index}

  *redis01_0_l_list01_0_1

  index 0 记录metadata 例如 max 和 rmax


#### Todo

  - [] 如何解决Lpush问题

rmax 3=1 2=2 3=1
max 5

if idx < rmax {
  realindx = rmax - idx +1
}else{
  realindx= idx - rmax
}

异步rebuild

### hansh


* key格式
  *instancename^db^h^keyname^field
* value
  redis value

### set

* key格式
  *instancename^db^s^keyname^value

### sorted set

* key格式
  *instancename^db^z^keyname^number
* value
  zset score

### Bit arrays

* key格式
  instancename^db^b^keyname  
* value 由0和1组成的string

方案二

* key格式
  instancename^db^b^keyname^index  
* value 8进制或16进制数字 通过计算获得

### HyperLogLogs

### Streams

# ToDo

* 如何scan？rawkv and txn
如何reverse scan