Prometheus.yaml
定义任务Job和实例Instance
```yaml
scrape_configs:
  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
  - job_name: 'node'
    static_configs:
      - targets: ['localhost:9100']
```


## 变量 获取prometheus.yaml 中的任务(Job)和实例(Instance)
* Job: 获取当前所有采集任务的名称： `label_values(up, job)` 

* Instance: 获取所有实例 `label_values({job="$Job"}, instance)`


## 函数
* rate函数可以直接计算区间向量v在时间窗口内平均增长速率
`rate(node_cpu[2m])`

## [Prometheus-Book](https://yunlzheng.gitbook.io/prometheus-book/parti-prometheus-ji-chu/promql/prometheus-query-language)


## [Grafana](https://yunlzheng.gitbook.io/prometheus-book/part-ii-prometheus-jin-jie/grafana/templating)

label_values(label)
label_values(metric, label)


## Grafana 配置实例
变量
1. `label_values(up, job)` 
2. `label_values({job="$Job"}, instance)`

Duration Or Latency
1. Histogram 数据通过Heatmap来展示数据分布

Total

各种指标配置 Duration、等可以参考:  Prometheus 2.0 Overview: 3662

## [Grafana 模板市场](https://grafana.com/grafana/dashboards)
* Prometheus 2.0 Overview: 3662 

```
kill -HUP 1
```

