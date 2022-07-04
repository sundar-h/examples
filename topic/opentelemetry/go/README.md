

直方图:
Histogram 会在一段时间范围内对数据进行采样（通常是请求持续时间或响应大小等），并将其计入可配置的存储桶（bucket）中。但这句话还是不太好理解，下面通过具体的示例来说明。
假设我们想监控某个应用在一段时间内的响应时间，最后监控到的样本的响应时间范围为 0s~10s。现在我们将样本的值域划分为不同的区间，即不同的 bucket，每个 bucket 的宽度是 0.2s。那么第一个 bucket 表示响应时间小于等于 0.2s 的请求数量，第二个 bucket 表示响应时间大于 0.2s 小于等于 0.4s 的请求数量，以此类推。
累计直方图:

Prometheus 的 histogram 是一种累积直方图，与上面的区间划分方式是有差别的，它的划分方式如下：还假设每个 bucket 的宽度是 0.2s，那么第一个 bucket 表示响应时间小于等于 0.2s 的请求数量，第二个 bucket 表示响应时间小于等于 0.4s 的请求数量，以此类推。也就是说，每一个 bucket 的样本包含了之前所有 bucket 的样本，所以叫累积直方图。


https://cloud.tencent.com/developer/article/1495303

## 应用
1. 计算5分钟之内的平均请求时间
```bash
// 五分钟之内的总时间/五分中之内的中个数
rate(http_request_duration_seconds_sum[5m]) 
/
rate(http_request_duration_seconds_count[5m])
 ```

https://grafana.com/tutorials/?utm_source=grafana_gettingstarted

## 实际常用汇总
* 每分钟请求成功率
* 每分钟请求耗时分区统计
* 每分钟httpCode分布
* 路由请求量/min
* 路由请求平均时延/min
* 路由失败请求时延/min
* path请求量/min
* path请求平均时延/min
* path失败请求时延/min
* 接口延时排序图

CPU使用率
内存使用率
当前温度
线程数/Goruotines数


## Graphana 可视化配置

https://yunlzheng.gitbook.io/prometheus-book/part-ii-prometheus-jin-jie/grafana/grafana-intro