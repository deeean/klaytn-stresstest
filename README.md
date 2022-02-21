# klaytn-stresstest
> 🚨 주의: 실제 운영중인 서버에 사용하시면 안됩니다.

오토스케일링이 진행중일때 Liveness failure 를 일으키는지 테스트 합니다.

```
Total Request: 1000
Average Latency: 84.34ms
Healthy: 999
Unhealthy: 1
Block Number: 83869587

Total Request: 2000
Average Latency: 83.16ms
Healthy: 1999
Unhealthy: 1
Block Number: 83869590

Total Request: 3000
Average Latency: 82.58ms
Healthy: 2999
Unhealthy: 1
Block Number: 83869595

Total Request: 4000
Average Latency: 82.42ms
Healthy: 3998
Unhealthy: 2
Block Number: 83869598

Total Request: 5000
Average Latency: 82.81ms
Healthy: 4998
Unhealthy: 2
Block Number: 83869601

Total Request: 6000
Average Latency: 82.32ms
Healthy: 5998
Unhealthy: 2
Block Number: 83869604
```