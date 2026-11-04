@@lesson aw2-serverless Serverless, Containers and Application Integration
@@text
Modern AWS architectures decouple components with queues and events and run code on Lambda or containers instead of managing servers. The Solutions Architect exam tests when to use SQS vs SNS vs EventBridge vs Kinesis, Lambda's limits and scaling, and the ECS/EKS/Fargate options.

## Lambda

- Runs code in response to events; you pay per **request** and per **GB-second** (memory × duration, billed per millisecond).
- Limits to know: timeout up to **15 minutes**; memory **128 MB – 10,240 MB** (CPU scales with memory); `/tmp` up to 10 GB; deployment package 50 MB zipped / 250 MB unzipped, or a **container image** up to 10 GB; default **1,000 concurrent executions per Region** (soft limit).
- **Invocation types**: **synchronous** (API Gateway, ALB: caller waits), **asynchronous** (S3, SNS, EventBridge: Lambda retries twice, then a **DLQ or on-failure destination**), **event source mapping** (SQS, Kinesis, DynamoDB Streams: Lambda polls in batches).
- **Cold starts**: a new execution environment initialises; reduce with smaller packages, init code outside the handler, **provisioned concurrency**, or **SnapStart** (Java, Python, .NET).
- **Reserved concurrency** caps (and guarantees) a function's concurrency; protects downstream databases. Use **RDS Proxy** for database connections.
- Configuration via environment variables; secrets from Secrets Manager/Parameter Store; **layers** for shared code; **function URLs** for simple HTTPS endpoints; VPC access for private resources (needs NAT or endpoints for internet/AWS APIs).

## API Gateway

**REST APIs** (full features: usage plans, API keys, request validation, caching, WAF), **HTTP APIs** (cheaper, faster, JWT authorisers), **WebSocket APIs**. Authorisation: IAM, Cognito user pools, Lambda authorisers. Throttling (10,000 requests/s account default, with burst), stages, canary deployments.

## Messaging and events

| Service | Pattern | Key facts |
|---|---|---|
| **SQS Standard** | queue, one consumer group pulls | at-least-once, best-effort ordering, nearly unlimited throughput |
| **SQS FIFO** | ordered queue | exactly-once processing, ordering per **message group**, lower throughput |
| **SNS** | pub/sub push | fan-out to SQS, Lambda, HTTP, email, SMS; filter policies |
| **EventBridge** | event bus with rules | content-based routing, SaaS sources, schedules, archive/replay |
| **Kinesis Data Streams** | real-time ordered stream | shards, multiple consumers replay data (retention 24 h – 365 days) |
| **Amazon Data Firehose** | delivery | loads streams into S3, Redshift, OpenSearch, Splunk (near real time) |
| **Amazon MQ** | managed ActiveMQ/RabbitMQ | for existing apps using AMQP/MQTT/JMS |
| **Step Functions** | workflow orchestration | state machines, retries, parallel branches, human approval |

SQS details: **visibility timeout** (default 30 s, max 12 h) hides a message while a consumer works; if it isn't deleted in time it reappears; **long polling** (`WaitTimeSeconds` up to 20) reduces empty responses; **retention** 4 days default, up to 14; **dead-letter queue** after `maxReceiveCount` failures; **delay queues**. Scale consumers (an ASG) on `ApproximateNumberOfMessagesVisible` per instance.

**Fan-out**: SNS topic → several SQS queues, so each subscriber processes every message independently.

## Containers

| Service | What it is |
|---|---|
| **ECR** | private container registry with image scanning |
| **ECS** | AWS container orchestrator: **task definitions**, services, cluster |
| **EKS** | managed Kubernetes control plane |
| **Fargate** | serverless compute for ECS/EKS: no EC2 instances to manage, pay per vCPU/GB-second |
| ECS/EKS on **EC2** | you manage the instances (cheaper at scale, GPUs, special needs) |
| **App Runner** | simplest path from image/source to a scaled HTTPS service |

ECS: **task role** (permissions for the app) vs **task execution role** (pull images, write logs). Services integrate with ALB (dynamic host ports), service auto scaling and Service Connect. EKS suits teams standardised on Kubernetes or hybrid/multi-cloud portability.

## Choosing compute

Lambda (event-driven, short, spiky, low ops) → Fargate (containers without servers, long-running) → ECS/EKS on EC2 (control and cost at scale) → EC2 (full control, legacy, licensing). **Elastic Beanstalk** deploys web apps on EC2 with less setup.

@@quiz
A job takes 25 minutes. Can it run as a single Lambda invocation?
- Yes, with 10 GB of memory
* No, Lambda's maximum timeout is 15 minutes; use Fargate, Batch or Step Functions to split it
- Yes, with provisioned concurrency
- Only from SQS
? Hard limit of 15 minutes.

@@quiz
An order must be processed by billing, shipping and analytics independently. Which pattern?
- One SQS queue with three consumers
* SNS topic fanning out to three SQS queues
- Kinesis with one shard
- A Lambda function calling all three
? Each queue gets its own copy of every message.

@@quiz
SQS messages are processed twice because processing takes 45 seconds. What should you change?
* Increase the visibility timeout above the processing time
- Enable long polling
- Use a delay queue
- Reduce retention
? Messages reappear when the visibility timeout expires.

@@quiz
Transactions for each account must be processed strictly in order. Which service?
- SQS Standard
* SQS FIFO with the account ID as message group ID
- SNS Standard
- EventBridge
? Ordering is per message group.

@@quiz
A Lambda function overloads a database during traffic spikes. What limits its concurrency?
- Provisioned concurrency
* Reserved concurrency
- A bigger memory setting
- Lambda layers
? Reserved concurrency caps simultaneous executions.

@@quiz
A team wants to run containers without managing EC2 instances. Which option?
- ECS on EC2
* ECS or EKS on Fargate
- Elastic Beanstalk on EC2
- Lightsail
? Fargate is serverless container compute.

@@flashcards
Lambda timeout :: 15 minutes max
Lambda memory :: 128 MB – 10 GB
Lambda concurrency default :: 1,000 per Region
Lambda pricing :: requests + GB-seconds
async invoke failures :: retries, DLQ / destination
cold start fixes :: provisioned concurrency, SnapStart
reserved concurrency :: cap and guarantee
HTTP API vs REST API :: cheaper vs more features
SQS Standard :: at-least-once, best-effort order
SQS FIFO :: ordered per message group
visibility timeout :: default 30 s, max 12 h
SQS retention :: 4 days default, 14 max
long polling :: up to 20 s
DLQ :: after maxReceiveCount
SNS :: pub/sub fan-out
EventBridge :: content-based event routing
Kinesis Data Streams :: ordered shards, replay
Firehose :: delivery to S3/Redshift/OpenSearch
Step Functions :: workflow state machines
ECS task role :: app permissions
task execution role :: pull images, write logs
Fargate :: serverless containers

@@drill
What is Lambda's maximum timeout in minutes?
= 15
? Longer jobs need another service.

@@drill
What is the default SQS visibility timeout in seconds?
= 30
? Maximum 12 hours.

@@exercise Estimate Lambda cost and concurrency
For each function compute the monthly cost with the example prices (illustrative: $0.20 per million requests, $0.0000166667 per GB-second, billed duration rounded up to the next millisecond) after subtracting the monthly free tier (1M requests and 400,000 GB-seconds) from the account total, and the peak concurrency (= peak requests per second × average duration in seconds, rounded up). Flag functions whose peak concurrency exceeds their reserved concurrency, and the account if total peak concurrency exceeds 1,000.
@@lang python
@@starter
import math

functions = [
    {"name": "thumbnail", "memory_mb": 1024, "avg_ms": 850.4, "requests": 3_000_000, "peak_rps": 120, "reserved": 200},
    {"name": "api-orders", "memory_mb": 512, "avg_ms": 120.2, "requests": 25_000_000, "peak_rps": 6000, "reserved": None},
    {"name": "nightly-report", "memory_mb": 3008, "avg_ms": 540000, "requests": 30, "peak_rps": 0.01, "reserved": 10},
    {"name": "webhook", "memory_mb": 256, "avg_ms": 3000, "requests": 400_000, "peak_rps": 80, "reserved": 100},
]
@@expected
thumbnail          2,553,000 GB-s  peak concurrency 103
api-orders         1,512,500 GB-s  peak concurrency 722
nightly-report        47,588 GB-s  peak concurrency 6
webhook              300,000 GB-s  peak concurrency 240  THROTTLED: peak 240 > reserved 100
Account: 28,400,030 requests, 4,413,088 GB-s, about $72.36/month
Total peak concurrency 1071 exceeds the 1,000 default: request a quota increase
@@requires
math.ceil(
400_000
@@hint
GB-seconds per function = `requests * math.ceil(avg_ms) / 1000 * memory_mb / 1024`. Apply the free tier to the totals: `max(0, total_requests - 1_000_000)` and `max(0, total_gbs - 400_000)`.
@@solution
import math

functions = [
    {"name": "thumbnail", "memory_mb": 1024, "avg_ms": 850.4, "requests": 3_000_000, "peak_rps": 120, "reserved": 200},
    {"name": "api-orders", "memory_mb": 512, "avg_ms": 120.2, "requests": 25_000_000, "peak_rps": 6000, "reserved": None},
    {"name": "nightly-report", "memory_mb": 3008, "avg_ms": 540000, "requests": 30, "peak_rps": 0.01, "reserved": 10},
    {"name": "webhook", "memory_mb": 256, "avg_ms": 3000, "requests": 400_000, "peak_rps": 80, "reserved": 100},
]

total_requests = 0
total_gbs = 0.0
total_peak = 0
for f in functions:
    gbs = f["requests"] * math.ceil(f["avg_ms"]) / 1000 * f["memory_mb"] / 1024
    peak = math.ceil(f["peak_rps"] * f["avg_ms"] / 1000)
    total_requests += f["requests"]
    total_gbs += gbs
    total_peak += peak
    flag = ""
    if f["reserved"] is not None and peak > f["reserved"]:
        flag = f"  THROTTLED: peak {peak} > reserved {f['reserved']}"
    print(f"{f['name']:<15} {gbs:>12,.0f} GB-s  peak concurrency {peak}{flag}")

billable_requests = max(0, total_requests - 1_000_000)
billable_gbs = max(0, total_gbs - 400_000)
cost = billable_requests / 1_000_000 * 0.20 + billable_gbs * 0.0000166667
print(f"Account: {total_requests:,} requests, {total_gbs:,.0f} GB-s, about ${cost:,.2f}/month")
print(f"Total peak concurrency {total_peak}" + (" exceeds the 1,000 default: request a quota increase" if total_peak > 1000 else " within the 1,000 default"))

@@exercise Simulate SQS visibility timeouts and a DLQ
Simulate one queue consumed by a worker. Each receive hides the message for `VISIBILITY` seconds; the worker needs `work[msg]` seconds and deletes the message only if it finishes within the timeout, otherwise the message becomes visible again (a new receive). Each `fail` message raises an error every time. After `MAX_RECEIVE` receives without deletion, the message moves to the DLQ. Process messages in FIFO order of visibility (time, then id) starting at t=0, one at a time, and print each event. Print the DLQ contents at the end.
@@lang python
@@starter
import heapq

VISIBILITY = 30
MAX_RECEIVE = 3
work = {"m1": 10, "m2": 45, "m3": 5, "m4": 20}
fail = {"m3"}
@@expected
t=  0 m1 receive #1: processed and deleted at 10
t= 10 m2 receive #1: not finished in 30s, visible again at 40
t= 40 m3 receive #1: error, visible again at 70
t= 45 m4 receive #1: processed and deleted at 65
t= 65 m2 receive #2: not finished in 30s, visible again at 95
t= 95 m3 receive #2: error, visible again at 125
t=100 m2 receive #3: not finished in 30s, visible again at 130
t=130 m3 receive #3: error, visible again at 160
t=135 m2 moved to DLQ after 3 receives
t=160 m3 moved to DLQ after 3 receives
DLQ: m2, m3
@@requires
heapq
MAX_RECEIVE
@@hint
Keep a heap of `(visible_at, msg)` and a clock. Pop the next message, set `clock = max(clock, visible_at)`, count the receive; a failure or a too-long job puts it back at `clock + VISIBILITY` (the worker gives up after the timeout, so the clock advances by `min(work, VISIBILITY)` for slow jobs and by `work` for successes).
@@solution
import heapq

VISIBILITY = 30
MAX_RECEIVE = 3
work = {"m1": 10, "m2": 45, "m3": 5, "m4": 20}
fail = {"m3"}

queue = [(0, msg) for msg in work]
heapq.heapify(queue)
receives = {msg: 0 for msg in work}
dlq = []
clock = 0
while queue:
    visible_at, msg = heapq.heappop(queue)
    if receives[msg] >= MAX_RECEIVE:
        dlq.append(msg)
        print(f"t={max(clock, visible_at):>3} {msg} moved to DLQ after {receives[msg]} receives")
        continue
    clock = max(clock, visible_at)
    receives[msg] += 1
    start = clock
    if msg in fail:
        clock += work[msg]
        print(f"t={start:>3} {msg} receive #{receives[msg]}: error, visible again at {start + VISIBILITY}")
        heapq.heappush(queue, (start + VISIBILITY, msg))
    elif work[msg] > VISIBILITY:
        clock += VISIBILITY
        print(f"t={start:>3} {msg} receive #{receives[msg]}: not finished in {VISIBILITY}s, visible again at {start + VISIBILITY}")
        heapq.heappush(queue, (start + VISIBILITY, msg))
    else:
        clock += work[msg]
        print(f"t={start:>3} {msg} receive #{receives[msg]}: processed and deleted at {clock}")
print("DLQ:", ", ".join(dlq) or "empty")
