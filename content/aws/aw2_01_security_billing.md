@@lesson aw2-security-billing Security, Monitoring, Billing and Support
@@text
## Shared responsibility

AWS is responsible for security **of** the cloud: data centres, hardware, the network, the hypervisor, managed services' infrastructure. You're responsible for security **in** the cloud: your data, IAM, OS patches on EC2, security groups, encryption choices, application code. The line moves with the service: on EC2 you patch the OS; on Lambda or S3, AWS does.

## IAM

- **Users** (people or apps with long-term credentials), **groups** (attach permissions to many users), **roles** (temporary credentials **assumed** by EC2 instances, Lambda functions, other accounts, or federated users: prefer roles to access keys).
- **Policies** are JSON documents: `Effect`, `Action`, `Resource`, optional `Condition`. An explicit **Deny** always wins; anything not allowed is denied.
- **Least privilege**, MFA everywhere, no root use, rotate or avoid access keys, **IAM Identity Center** for single sign-on across accounts, **IAM Access Analyzer**, credential reports.
- **AWS Organizations**: many accounts under one management account, **consolidated billing**, and **SCPs** (service control policies) that set maximum permissions for accounts.

```json
{
  "Version": "2012-10-17",
  "Statement": [{
    "Effect": "Allow",
    "Action": ["s3:GetObject", "s3:PutObject"],
    "Resource": "arn:aws:s3:::sakura-reports/*"
  }]
}
```

## Security services

| Service | Does |
|---|---|
| **KMS** | manages encryption keys; **CloudHSM** for dedicated hardware |
| **Secrets Manager** | stores and rotates passwords and API keys (Parameter Store for config) |
| **ACM** | free TLS certificates for ALB, CloudFront |
| **Shield** | DDoS protection (Standard is free and automatic; Advanced is paid, with a response team) |
| **WAF** | web application firewall rules on CloudFront, ALB, API Gateway |
| **GuardDuty** | **threat detection** from logs (CloudTrail, VPC Flow Logs, DNS) |
| **Inspector** | **vulnerability scanning** of EC2, containers, Lambda |
| **Macie** | finds **sensitive data** (PII) in S3 |
| **Security Hub** | central view of security findings and best-practice checks |
| **Detective** | investigates the root cause of findings |
| **Artifact** | AWS's compliance reports (ISO, SOC, PCI) |
| **Cognito** | sign-up and sign-in for your app's users |

## Monitoring and governance

- **CloudWatch**: metrics, logs, **alarms** (e.g. CPU > 80% → scale or notify), dashboards.
- **CloudTrail**: who did **what API call**, when, from where: the audit log.
- **AWS Config**: resource configuration history and compliance rules ("are all EBS volumes encrypted?").
- **Trusted Advisor**: checks for cost, performance, security, fault tolerance and service limits.
- **Systems Manager**: patching, run commands, Session Manager (shell access without SSH ports).
- **CloudFormation** (infrastructure as code in YAML/JSON) and the **CDK**.

## Billing and support

- **Pricing Calculator** (estimate before building), **Cost Explorer** (analyse spending), **Budgets** (alerts), **Cost and Usage Reports** (detailed data), **cost allocation tags**, **Compute Optimizer** (rightsizing).
- Free tier: always free, 12 months free, and trials.
- Data **into** AWS is free; data **out** to the internet costs money.
- Support plans: **Basic** (free: docs, forums, some Trusted Advisor checks), **Developer** (business-hours email), **Business** (24/7 phone, full Trusted Advisor), **Enterprise On-Ramp** (pool of TAMs), **Enterprise** (a dedicated **Technical Account Manager**, 15-minute response for critical issues).
- **AWS Marketplace** sells third-party software; **AWS Professional Services** and **Partners** help with projects; **re:Post** is the community Q&A.

@@quiz
Under the shared responsibility model, who patches the guest operating system on an EC2 instance?
* The customer
- AWS
- Both, alternating months
- Nobody: EC2 patches itself
? On managed services like RDS or Lambda, AWS patches.

@@quiz
An application on EC2 needs to read from S3. What's the most secure way to give it access?
* Attach an IAM role to the instance
- Put an IAM user's access keys in the code
- Make the bucket public
- Use the root account's keys
? Roles provide temporary, automatically rotated credentials.

@@quiz
Which service records who deleted an S3 bucket and when?
* AWS CloudTrail
- Amazon CloudWatch metrics
- AWS Trusted Advisor
- Amazon Inspector
? CloudTrail logs API calls.

@@quiz
Which service uses machine learning to detect threats like crypto-mining or calls from known malicious IPs, by analysing logs?
* Amazon GuardDuty
- Amazon Inspector
- Amazon Macie
- AWS Artifact
? Inspector scans for vulnerabilities; Macie finds sensitive data in S3.

@@quiz
Where can a company download AWS's ISO 27001 and SOC 2 compliance reports for its auditors?
* AWS Artifact
- AWS Config
- AWS Security Hub
- AWS Marketplace
? Artifact provides on-demand compliance documents.

@@quiz
Which support plan includes a dedicated Technical Account Manager?
* Enterprise
- Business
- Developer
- Basic
? Enterprise On-Ramp gets access to a pool of TAMs.

@@quiz
A company with 20 AWS accounts wants one bill and the ability to prevent any account from using regions outside Japan. What should it use?
* AWS Organizations with consolidated billing and SCPs
- IAM groups in each account
- Separate credit cards per account
- AWS Budgets only
? SCPs set the maximum permissions for member accounts.

@@quiz
A team wants an email when monthly spending is forecast to exceed ¥50,000. Which tool?
* AWS Budgets
- Cost Explorer only
- Pricing Calculator
- CloudTrail
? Budgets send alerts on actual or forecast cost.

@@flashcards
shared responsibility :: AWS: security OF the cloud; customer: security IN the cloud
IAM user / group / role :: Long-term identity / set of users / temporary assumed permissions
IAM policy :: JSON: Effect, Action, Resource, Condition; explicit Deny wins
IAM Identity Center :: SSO across AWS accounts
Organizations / SCP :: Multi-account management / maximum permission guardrails
KMS :: Encryption key management
Secrets Manager :: Stores and rotates secrets
Shield :: DDoS protection (Standard free)
WAF :: Web application firewall
GuardDuty :: Threat detection from logs
Inspector :: Vulnerability scanning
Macie :: Finds PII in S3
Artifact :: Compliance reports
CloudWatch :: Metrics, logs, alarms
CloudTrail :: API call audit log
AWS Config :: Configuration history and compliance rules
Trusted Advisor :: Best-practice checks
Cost Explorer / Budgets :: Analyse costs / alert on costs
Pricing Calculator :: Estimate costs before building
Enterprise support :: Dedicated TAM, 15-minute critical response

@@drill
Which service is the audit log of every API call in an AWS account?
= CloudTrail
= AWS CloudTrail
? Turn on an organization trail and send it to a protected S3 bucket.
