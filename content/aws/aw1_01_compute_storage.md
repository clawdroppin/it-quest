@@lesson aw1-compute-storage Compute and Storage
@@text
## EC2 and friends

**EC2** gives you virtual servers (**instances**). Families: general purpose (`t`, `m`), compute optimized (`c`), memory optimized (`r`, `x`), storage optimized (`i`), accelerated (`p`, `g`: GPUs). An **AMI** is the image an instance starts from; **user data** runs a script on first boot.

| Pricing option | Best for |
|---|---|
| **On-Demand** | short, unpredictable workloads; no commitment |
| **Savings Plans** | commit to an amount per hour for 1 or 3 years (flexible across instance types) |
| **Reserved Instances** | commit to an instance type in a region for 1 or 3 years: up to ~72% off |
| **Spot** | spare capacity, up to ~90% off, but can be **interrupted** with 2 minutes' notice: batch jobs, CI, rendering |
| **Dedicated Hosts** | a whole physical server for you: licensing (per-socket) and compliance |
| **Capacity Reservations** | guarantee capacity in an AZ |

- **Auto Scaling groups** add and remove instances by load (min, desired, max).
- **Elastic Load Balancing**: **ALB** (HTTP/HTTPS, layer 7, path and host routing), **NLB** (TCP/UDP, layer 4, extreme performance), GWLB (appliances).
- **Lambda**: run code without servers, triggered by events (an S3 upload, an API call, a schedule); pay per request and millisecond; up to 15 minutes per run.
- Containers: **ECS** (AWS's orchestrator), **EKS** (managed Kubernetes), **Fargate** (serverless containers), **ECR** (image registry).
- Others: **Elastic Beanstalk** (PaaS: upload code, AWS runs it), **Lightsail** (simple VPS with fixed prices), **Batch**.

## Storage

| Service | Type | Notes |
|---|---|---|
| **S3** | object storage | buckets of objects, 11 nines (99.999999999%) durability, unlimited size; static websites |
| **EBS** | block storage | a virtual disk for **one** EC2 instance in **one AZ**; snapshots go to S3 |
| **Instance store** | block | physically attached, very fast, **lost** when the instance stops |
| **EFS** | file (NFS) | shared by many Linux instances across AZs, grows automatically |
| **FSx** | file | Windows File Server (SMB, AD), Lustre (HPC), NetApp ONTAP, OpenZFS |
| **Storage Gateway** | hybrid | on-premises access to cloud storage |
| **AWS Backup** | | central backup policies across services |
| **Snow family** | physical devices | move terabytes to petabytes offline |

S3 storage classes, from hot to cold: **Standard**, **Intelligent-Tiering** (moves objects automatically), **Standard-IA** and **One Zone-IA** (infrequent access), **Glacier Instant Retrieval**, **Glacier Flexible Retrieval** (minutes to hours), **Glacier Deep Archive** (cheapest, 12+ hours). **Lifecycle rules** move or delete objects by age. Also: **versioning**, **replication** (cross-region), **Object Lock** (WORM: ransomware protection), presigned URLs, **Block Public Access** (on by default: keep it on unless you mean to publish).

@@quiz
A nightly video-rendering batch job can be interrupted and restarted at any time. Which EC2 option is cheapest?
* Spot Instances
- On-Demand
- Dedicated Hosts
- Reserved Instances for 3 years
? Spot is up to ~90% off, in exchange for possible interruption.

@@quiz
A database server will run 24/7 for at least three years. How can costs be cut most?
* A 3-year Reserved Instance or Savings Plan
- Spot Instances
- On-Demand
- A bigger instance
? Long, steady workloads are what commitments are for.

@@quiz
Several Linux EC2 instances in different AZs need the same shared file system. Which service?
* Amazon EFS
- Amazon EBS
- Instance store
- S3 Glacier
? EBS volumes attach to one instance in one AZ (multi-attach is a special case).

@@quiz
Log files must be kept 7 years for audits and are almost never read. Which S3 class is cheapest?
* S3 Glacier Deep Archive
- S3 Standard
- S3 Standard-IA
- S3 Intelligent-Tiering
? Retrieval takes hours, which is fine for rare audits.

@@quiz
Which load balancer routes HTTP requests by URL path, like /api to one group and /images to another?
* Application Load Balancer
- Network Load Balancer
- Gateway Load Balancer
- Route 53
? ALB works at layer 7.

@@quiz
Code should run whenever an image is uploaded to S3, without managing servers. What service fits?
* AWS Lambda
- EC2 with cron
- Amazon EFS
- AWS Snowball
? Lambda functions are triggered by events and billed per use.

@@quiz
Data on an instance store volume after the instance is stopped is...
* Lost
- Kept in S3
- Moved to EBS
- Kept for 30 days
? Instance store is ephemeral; use EBS for data that must persist.

@@flashcards
EC2 :: Virtual servers
AMI :: Machine image an instance boots from
On-Demand :: Pay per second, no commitment
Reserved Instances / Savings Plans :: 1-3 year commitment, big discounts
Spot :: Spare capacity, up to 90% off, interruptible
Dedicated Host :: Whole physical server: licensing, compliance
Auto Scaling group :: Adds/removes instances with demand
ALB / NLB :: Layer 7 HTTP / layer 4 TCP-UDP load balancers
Lambda :: Serverless functions, event-driven, up to 15 minutes
ECS / EKS / Fargate :: AWS containers / managed Kubernetes / serverless containers
Elastic Beanstalk :: PaaS for web apps
S3 :: Object storage, 11 nines durability
EBS :: Block storage for one instance in one AZ
EFS :: Shared NFS file system
FSx for Windows :: Managed SMB file server with AD
Glacier Deep Archive :: Cheapest archive, hours to retrieve
S3 lifecycle rule :: Move or delete objects by age
S3 Object Lock :: Write once, read many

@@drill
Which AWS service provides serverless functions?
= Lambda
= AWS Lambda
? Pay per request and duration.
