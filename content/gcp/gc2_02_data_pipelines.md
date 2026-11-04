@@lesson gc2-data Data Pipelines and Analytics: Pub/Sub, Dataflow, Dataproc, BigQuery and Vertex AI
@@text
Google Cloud's strongest area is data. Case studies often include "ingest events from millions of devices, process them in real time, and analyse them", and you need to know which managed services do each step. This lesson covers the ingestion, processing, storage, orchestration and ML layers, with the facts exams like to test.

## The reference pipeline

**Ingest** (Pub/Sub, Storage Transfer, Datastream) → **process** (Dataflow, Dataproc, Cloud Run) → **store/analyse** (BigQuery, Bigtable, Cloud Storage) → **visualise/ML** (Looker, Vertex AI). Orchestrate with **Cloud Composer** (managed Apache Airflow) or **Workflows**.

## Pub/Sub

- Global, serverless messaging: **topics** and **subscriptions** (each subscription receives every message; several subscribers on one subscription share the work).
- Delivery: **pull**, **push** (HTTPS endpoint), **BigQuery** and **Cloud Storage** subscriptions (write directly).
- **At-least-once** by default; consumers must be **idempotent** or enable **exactly-once delivery** (pull, same region). **Ordering keys** keep messages with the same key in order.
- **Acknowledgement deadline** (default 10 s, up to 600 s): unacknowledged messages are redelivered. **Dead-letter topics** after max delivery attempts; **retry policies** with exponential backoff.
- Retention: unacknowledged messages kept **7 days** by default (up to 31 days); topic retention allows **seek/replay**.
- **Pub/Sub Lite** was retired in favour of Pub/Sub or Managed Service for Apache Kafka.

## Dataflow

- Managed **Apache Beam** runner for **batch and streaming** with the same code; autoscaling, dynamic work rebalancing, serverless.
- Streaming concepts: **windows** (fixed/tumbling, sliding, session), **watermarks** (how far event time has progressed), **triggers**, **late data** with allowed lateness. Event time vs processing time.
- **Templates** (classic and Flex) for reusable jobs, e.g. Pub/Sub → BigQuery.

## Dataproc

Managed **Spark/Hadoop/Hive/Presto**; clusters start in about 90 seconds, so create **ephemeral job-specific clusters**, store data in Cloud Storage (not HDFS), use **preemptible/Spot secondary workers**. **Dataproc Serverless** runs Spark without clusters. Choose Dataproc to lift existing Spark/Hadoop jobs; Dataflow for new pipelines.

## BigQuery for analytics

- Load (batch, free), **streaming** (Storage Write API), or query external data (BigLake, federated queries to Cloud SQL/Spanner).
- **Partitioning** (time/integer) and **clustering** reduce scanned data; **materialised views**, **BI Engine** for dashboards; **BigQuery ML** (`CREATE MODEL` with SQL); **Analytics Hub** for data sharing.
- Security: dataset/table IAM, **authorised views**, row-level access policies, column-level policy tags (Dataplex).
- **Dataplex** catalogues and governs data; **Datastream** replicates CDC from MySQL/PostgreSQL/Oracle into BigQuery.

## ML and AI

**Vertex AI**: AutoML, custom training, pipelines, Model Registry, endpoints for online prediction, batch prediction, Feature Store, **Model Garden** and Gemini models through **Vertex AI Studio** and APIs; pre-trained APIs (Vision, Speech-to-Text, Translation, Natural Language, Document AI). Choose pre-trained APIs first, AutoML when you have labelled data but no ML team, custom training for full control.

## Choosing quickly

| Clue | Service |
|---|---|
| "ingest millions of events per second, decouple" | Pub/Sub |
| "real-time and batch with one codebase", "windowing", "Apache Beam" | Dataflow |
| "existing Spark/Hadoop jobs" | Dataproc |
| "SQL analytics over petabytes", "data warehouse" | BigQuery |
| "low-latency lookups on time series" | Bigtable |
| "schedule and orchestrate DAGs", "Airflow" | Cloud Composer |
| "dashboards", "semantic model" | Looker / Looker Studio |
| "CDC replication into BigQuery" | Datastream |

@@quiz
Existing on-premises Spark jobs must move to Google Cloud with minimal code changes. Which service?
- Dataflow
* Dataproc
- BigQuery
- Cloud Run
? Managed Spark/Hadoop.

@@quiz
A subscriber takes 60 seconds to process each Pub/Sub message, and messages are delivered twice. What should you change?
- Add an ordering key
* Increase the acknowledgement deadline (or extend it while processing)
- Reduce topic retention
- Use push delivery
? Unacked messages are redelivered after the deadline.

@@quiz
Which concept in Dataflow streaming estimates how complete the data for a given event time is?
- Trigger
* Watermark
- Partition
- Ack deadline
? Late data arrives after the watermark.

@@quiz
Analysts want dashboards that query BigQuery with sub-second response. What helps?
* BI Engine (and materialised views)
- Dataproc
- Pub/Sub
- Nearline storage
? In-memory acceleration.

@@quiz
Pub/Sub delivers at least once. What must consumers do?
- Nothing
* Be idempotent (or enable exactly-once delivery)
- Use push subscriptions
- Disable retries
? Duplicates are possible.

@@flashcards
Pub/Sub :: global serverless messaging
subscription :: each receives every message
ack deadline :: default 10 s, max 600 s
message retention :: 7 days default, up to 31
ordering key :: in-order per key
dead-letter topic :: after max delivery attempts
Dataflow :: Apache Beam batch + streaming
window types :: fixed, sliding, session
watermark :: event-time progress
Dataproc :: managed Spark/Hadoop
ephemeral clusters :: per-job, data in GCS
Dataproc Serverless :: Spark without clusters
BigQuery ML :: models with SQL
BI Engine :: in-memory BI acceleration
authorised view :: share query results, not tables
Dataplex :: data governance and catalogue
Datastream :: CDC into BigQuery
Cloud Composer :: managed Airflow
Vertex AI :: ML platform
pre-trained APIs :: Vision, Speech, Translation, Document AI

@@drill
Which open-source SDK do Dataflow pipelines use?
= Apache Beam
= Beam
? Same code for batch and streaming.

@@drill
What is the default Pub/Sub acknowledgement deadline in seconds?
= 10
? Up to 600.

@@exercise Fixed windows with a watermark
Aggregate streaming events (event time in seconds, sensor, value) into 60-second fixed windows per sensor, in arrival order. The watermark is the maximum event time seen minus `ALLOWED_DELAY`. A window is emitted (printed) when the watermark passes its end; events that arrive for an already-emitted window are counted as late and dropped. At the end, flush the remaining windows. Print each emitted window's average and the late events.
@@lang python
@@starter
WINDOW = 60
ALLOWED_DELAY = 30
events = [(5, "s1", 20.0), (12, "s2", 30.0), (50, "s1", 22.0), (70, "s1", 25.0), (65, "s2", 31.0), (40, "s2", 29.0),
          (95, "s1", 24.0), (130, "s2", 33.0), (58, "s1", 99.0), (150, "s1", 26.0), (125, "s1", 23.0), (200, "s2", 35.0)]
@@expected
[  0, 60) s1: n=2 avg=21.0
[  0, 60) s2: n=2 avg=29.5
[ 60,120) s1: n=2 avg=24.5
[ 60,120) s2: n=1 avg=31.0
[120,180) s1: n=2 avg=24.5
[120,180) s2: n=1 avg=33.0
[180,240) s2: n=1 avg=35.0
Late events dropped: [(58, 's1', 99.0)]
@@requires
ALLOWED_DELAY
watermark
@@hint
Window start = `t // WINDOW * WINDOW`. Keep `open_windows[(sensor, start)] = [values]` and a set of emitted keys. After each event, emit (in sorted order) every open window whose `start + WINDOW <= watermark`.
@@solution
WINDOW = 60
ALLOWED_DELAY = 30
events = [(5, "s1", 20.0), (12, "s2", 30.0), (50, "s1", 22.0), (70, "s1", 25.0), (65, "s2", 31.0), (40, "s2", 29.0),
          (95, "s1", 24.0), (130, "s2", 33.0), (58, "s1", 99.0), (150, "s1", 26.0), (125, "s1", 23.0), (200, "s2", 35.0)]

open_windows = {}
emitted = set()
late = []
max_seen = 0

def emit(key):
    sensor, start = key
    values = open_windows.pop(key)
    emitted.add(key)
    print(f"[{start:>3},{start + WINDOW:>3}) {sensor}: n={len(values)} avg={sum(values) / len(values):.1f}")

for t, sensor, value in events:
    key = (sensor, t // WINDOW * WINDOW)
    if key in emitted:
        late.append((t, sensor, value))
    else:
        open_windows.setdefault(key, []).append(value)
    max_seen = max(max_seen, t)
    watermark = max_seen - ALLOWED_DELAY
    for k in sorted(open_windows, key=lambda k: (k[1], k[0])):
        if k[1] + WINDOW <= watermark:
            emit(k)

for k in sorted(open_windows, key=lambda k: (k[1], k[0])):
    emit(k)
print("Late events dropped:", late)

@@exercise Pub/Sub redelivery and dead-lettering
Simulate a pull subscription with ack deadline `ACK_DEADLINE` seconds, a dead-letter topic after `MAX_DELIVERIES`, and an idempotent consumer that remembers processed message IDs. Each message needs `work` seconds; jobs longer than the deadline aren't acknowledged in time and are redelivered (the work still completes, so the side effect happens once thanks to idempotency). Messages in `poison` always fail. Print every delivery, then the processed IDs and the dead-letter topic.
@@lang python
@@starter
ACK_DEADLINE = 10
MAX_DELIVERIES = 5
messages = {"m1": 2, "m2": 15, "m3": 1, "m4": 4}
poison = {"m3"}
@@expected
m1 delivery 1: processed, ack
m2 delivery 1: processed in 15s but ack deadline 10s passed, redelivered
m3 delivery 1: failed, nack
m4 delivery 1: processed, ack
m2 delivery 2: duplicate, already processed, ack
m3 delivery 2: failed, nack
m3 delivery 3: failed, nack
m3 delivery 4: failed, nack
m3 delivery 5: failed, sent to dead-letter topic
Processed once: m1, m2, m4
Dead-letter topic: m3
@@requires
MAX_DELIVERIES
processed
@@hint
Loop over a deque of `(msg, attempt)`. A slow message is processed (side effect recorded if not already) but redelivered; the second delivery is a duplicate that the idempotent consumer just acknowledges.
@@solution
from collections import deque

ACK_DEADLINE = 10
MAX_DELIVERIES = 5
messages = {"m1": 2, "m2": 15, "m3": 1, "m4": 4}
poison = {"m3"}

queue = deque((m, 1) for m in messages)
processed = []
dead_letter = []
while queue:
    msg, attempt = queue.popleft()
    if msg in poison:
        if attempt >= MAX_DELIVERIES:
            dead_letter.append(msg)
            print(f"{msg} delivery {attempt}: failed, sent to dead-letter topic")
        else:
            print(f"{msg} delivery {attempt}: failed, nack")
            queue.append((msg, attempt + 1))
        continue
    if msg in processed:
        print(f"{msg} delivery {attempt}: duplicate, already processed, ack")
        continue
    processed.append(msg)
    if messages[msg] > ACK_DEADLINE:
        print(f"{msg} delivery {attempt}: processed in {messages[msg]}s but ack deadline {ACK_DEADLINE}s passed, redelivered")
        queue.append((msg, attempt + 1))
    else:
        print(f"{msg} delivery {attempt}: processed, ack")
print("Processed once:", ", ".join(processed))
print("Dead-letter topic:", ", ".join(dead_letter) or "empty")
