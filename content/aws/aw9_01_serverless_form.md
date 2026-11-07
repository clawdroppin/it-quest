@@lesson aw9-serverless-form Serverless IT Request Form
@@text
Time to build on AWS for real. Staff fill in a web form ("my laptop screen flickers"), and the request lands in a database for the IT team. No servers to patch, and it costs close to nothing:

```text
browser (site/index.html)  ──POST──▶  API Gateway (HTTP API)  ──▶  Lambda (handler.py)  ──▶  DynamoDB
                                        throttling, CORS          validates, stores          encrypted, PITR
```

Everything is defined in **`template.yaml`**, an **AWS SAM** template (CloudFormation with shortcuts for serverless). One `sam deploy` creates the table, the function, its **least-privilege IAM role** (write to one table, nothing else) and the API. One `sam delete` removes it all. That's **infrastructure as code**, and it's how real teams work.

## Design the code to be testable

The Lambda **entry point** (`lambda_handler`) only wires things together: it creates the DynamoDB table object and passes it, the time and a new ID into **`handle()`**. Everything interesting (`validate`, `build_item`, `handle`) is a plain function, so the tests can pass in a **fake table** and a fixed time and run in milliseconds, with no AWS account at all. Only after the tests pass do you deploy.

Things the handler must get right, because API Gateway and real users are messy:

- the body is a JSON **string** in `event["body"]`, sometimes **base64-encoded**;
- anything can be wrong: not JSON, a JSON list instead of an object, missing fields, `"priority": "2"` as text;
- **never trust the browser**: the HTML form validates too, but anyone can send requests with curl;
- responses need **CORS** headers, or browsers on another origin can't read them.
@@project it_request_form Serverless IT Request Form
Write `validate`, `build_item` and `handle` in `handler.py` until `python -m unittest` passes. Then follow `LAB.md` to deploy it with the SAM CLI, try it, look around the console, and **clean up** with `sam delete`.
@@file handler.py
"""Lambda function behind API Gateway: accepts IT requests and stores them in DynamoDB."""

import base64
import json
import os
import uuid
from datetime import datetime, timezone

CATEGORIES = {"hardware", "software", "access", "network", "other"}


def validate(payload):
    """Problems with a request, as messages (empty = valid):
      "name is required"                        (missing, empty or only spaces; at most 100 characters)
      "name is too long"
      "email is not valid"                      (needs one "@" with a "." somewhere after it)
      "category must be one of: access, hardware, network, other, software"
      "description must be 10-2000 characters"
      "priority must be 1-4"                    (optional; must be an int, not a bool)
    """
    raise NotImplementedError("TODO: validate")


def build_item(payload, now, new_id):
    """The DynamoDB item for a valid request: id, created (UTC, "2026-10-31T09:00:00Z"), status "open",
    name and description stripped, email lowercased, category, priority (default 3)."""
    raise NotImplementedError("TODO: build_item")


def response(status, body):
    return {
        "statusCode": status,
        "headers": {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"},
        "body": json.dumps(body),
    }


def handle(event, table, now, new_id):
    """Process an API Gateway event:
    400 {"errors": ["body must be a JSON object"]} if the body isn't a JSON object,
    400 {"errors": [...]} if validation fails,
    otherwise put the item into `table` and return 201 {"id": new_id}.
    The body may be base64-encoded (event["isBase64Encoded"])."""
    raise NotImplementedError("TODO: handle")


_table = None


def lambda_handler(event, context):
    """The entry point AWS calls. The table name comes from the TABLE_NAME environment variable."""
    global _table
    if _table is None:
        import boto3  # available in the Lambda runtime

        _table = boto3.resource("dynamodb").Table(os.environ["TABLE_NAME"])
    return handle(event, _table, datetime.now(timezone.utc), uuid.uuid4().hex)
@@file test_handler.py
import base64
import json
import unittest
from datetime import datetime, timedelta, timezone

from handler import build_item, handle, validate

GOOD = {"name": " Yuki Tanaka ", "email": "Yuki.Tanaka@Sakura.example", "category": "hardware",
        "description": "Laptop screen flickers when on battery.", "priority": 2}
NOW = datetime(2026, 10, 31, 18, 0, 0, tzinfo=timezone(timedelta(hours=9)))  # 18:00 in Tokyo


class FakeTable:
    def __init__(self):
        self.items = []

    def put_item(self, Item):
        self.items.append(Item)


class ValidateTests(unittest.TestCase):
    def test_good(self):
        self.assertEqual(validate(GOOD), [])
        self.assertEqual(validate({k: v for k, v in GOOD.items() if k != "priority"}), [])

    def test_everything_wrong(self):
        self.assertEqual(validate({"name": "  ", "email": "nope", "category": "coffee", "description": "short", "priority": 9}), [
            "name is required",
            "email is not valid",
            "category must be one of: access, hardware, network, other, software",
            "description must be 10-2000 characters",
            "priority must be 1-4",
        ])

    def test_emails(self):
        for bad in ["a@b", "@b.com", "a@@b.com", "a@b.", "", None]:
            self.assertIn("email is not valid", validate({**GOOD, "email": bad}), bad)
        self.assertEqual(validate({**GOOD, "email": "a.b+it@mail.co.jp"}), [])

    def test_long_name_and_bool_priority(self):
        self.assertEqual(validate({**GOOD, "name": "x" * 101}), ["name is too long"])
        self.assertEqual(validate({**GOOD, "priority": True}), ["priority must be 1-4"])
        self.assertEqual(validate({**GOOD, "priority": "2"}), ["priority must be 1-4"])


class ItemTests(unittest.TestCase):
    def test_item(self):
        self.assertEqual(build_item(GOOD, NOW, "abc123"), {
            "id": "abc123", "created": "2026-10-31T09:00:00Z", "status": "open",
            "name": "Yuki Tanaka", "email": "yuki.tanaka@sakura.example", "category": "hardware",
            "description": "Laptop screen flickers when on battery.", "priority": 2,
        })

    def test_default_priority(self):
        payload = {k: v for k, v in GOOD.items() if k != "priority"}
        self.assertEqual(build_item(payload, NOW, "x")["priority"], 3)


class HandleTests(unittest.TestCase):
    def call(self, body, encoded=False):
        table = FakeTable()
        if encoded:
            event = {"body": base64.b64encode(body.encode()).decode(), "isBase64Encoded": True}
        else:
            event = {"body": body}
        return handle(event, table, NOW, "id-1"), table

    def test_created(self):
        res, table = self.call(json.dumps(GOOD))
        self.assertEqual(res["statusCode"], 201)
        self.assertEqual(json.loads(res["body"]), {"id": "id-1"})
        self.assertEqual(res["headers"]["Access-Control-Allow-Origin"], "*")
        self.assertEqual(len(table.items), 1)
        self.assertEqual(table.items[0]["email"], "yuki.tanaka@sakura.example")

    def test_base64_body(self):
        res, table = self.call(json.dumps(GOOD), encoded=True)
        self.assertEqual(res["statusCode"], 201)

    def test_invalid_is_not_stored(self):
        res, table = self.call(json.dumps({**GOOD, "category": "coffee"}))
        self.assertEqual(res["statusCode"], 400)
        self.assertEqual(json.loads(res["body"])["errors"], ["category must be one of: access, hardware, network, other, software"])
        self.assertEqual(table.items, [])

    def test_not_json(self):
        for body in ["not json", "[1, 2]", "", None]:
            res, table = self.call(body) if body is not None else (handle({}, FakeTable(), NOW, "x"), None)
            self.assertEqual(res["statusCode"], 400, body)
            self.assertEqual(json.loads(res["body"]), {"errors": ["body must be a JSON object"]})


if __name__ == "__main__":
    unittest.main()
@@file template.yaml
AWSTemplateFormatVersion: "2010-09-09"
Transform: AWS::Serverless-2016-10-31
Description: IT request form - API Gateway, Lambda and DynamoDB (IT Quest build)

Resources:
  RequestsTable:
    Type: AWS::DynamoDB::Table
    Properties:
      BillingMode: PAY_PER_REQUEST          # pay per request: about zero cost for a lab
      AttributeDefinitions:
        - AttributeName: id
          AttributeType: S
      KeySchema:
        - AttributeName: id
          KeyType: HASH
      PointInTimeRecoverySpecification:
        PointInTimeRecoveryEnabled: true
      SSESpecification:
        SSEEnabled: true

  SubmitFunction:
    Type: AWS::Serverless::Function
    Properties:
      Runtime: python3.12
      Handler: handler.lambda_handler
      CodeUri: .
      MemorySize: 128
      Timeout: 5
      Environment:
        Variables:
          TABLE_NAME: !Ref RequestsTable
      Policies:
        - DynamoDBWritePolicy:               # least privilege: write to this one table only
            TableName: !Ref RequestsTable
      Events:
        Submit:
          Type: HttpApi
          Properties:
            Path: /requests
            Method: POST
            ApiId: !Ref RequestsApi

  RequestsApi:
    Type: AWS::Serverless::HttpApi
    Properties:
      CorsConfiguration:
        AllowOrigins: ["*"]
        AllowMethods: [POST]
        AllowHeaders: [content-type]
      DefaultRouteSettings:
        ThrottlingBurstLimit: 10             # protect the wallet from floods
        ThrottlingRateLimit: 5

Outputs:
  ApiUrl:
    Description: POST requests here
    Value: !Sub "https://${RequestsApi}.execute-api.${AWS::Region}.amazonaws.com/requests"
  TableName:
    Value: !Ref RequestsTable
@@file site/index.html
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>IT Request</title>
<style>
  body { font-family: system-ui, sans-serif; max-width: 34rem; margin: 2rem auto; padding: 0 1rem; color: #222; }
  label { display: block; margin-top: 1rem; font-weight: 600; }
  input, select, textarea { width: 100%; padding: .5rem; font: inherit; box-sizing: border-box; }
  button { margin-top: 1.2rem; padding: .6rem 1.2rem; font: inherit; }
  #result { margin-top: 1rem; }
  .ok { color: #176b2c; } .bad { color: #a11; }
</style>
</head>
<body>
<h1>IT Request</h1>
<p>Tell the IT team what you need. We'll reply by email.</p>
<form id="form">
  <label>Name <input name="name" required maxlength="100"></label>
  <label>Email <input name="email" type="email" required></label>
  <label>Category
    <select name="category">
      <option value="hardware">Hardware</option>
      <option value="software">Software</option>
      <option value="access">Access / accounts</option>
      <option value="network">Network</option>
      <option value="other">Other</option>
    </select>
  </label>
  <label>Priority
    <select name="priority">
      <option value="4">Low</option>
      <option value="3" selected>Normal</option>
      <option value="2">High</option>
      <option value="1">Critical: I can't work</option>
    </select>
  </label>
  <label>Description <textarea name="description" rows="5" required minlength="10"></textarea></label>
  <button type="submit">Send</button>
</form>
<div id="result"></div>
<script>
  // After "sam deploy", paste the ApiUrl output here.
  const API_URL = "https://REPLACE-ME.execute-api.ap-northeast-1.amazonaws.com/requests";

  const form = document.getElementById("form");
  const result = document.getElementById("result");
  form.addEventListener("submit", async (e) => {
    e.preventDefault();
    const data = Object.fromEntries(new FormData(form));
    data.priority = Number(data.priority);
    result.textContent = "Sending...";
    try {
      const res = await fetch(API_URL, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(data) });
      const body = await res.json();
      if (res.ok) {
        result.className = "ok";
        result.textContent = "Thanks! Your request number is " + body.id.slice(0, 8) + ".";
        form.reset();
      } else {
        result.className = "bad";
        result.textContent = (body.errors || ["Something went wrong"]).join(". ");
      }
    } catch (err) {
      result.className = "bad";
      result.textContent = "Couldn't reach the server: " + err;
    }
  });
</script>
</body>
</html>
@@file LAB.md
# Lab: deploy the IT request form to AWS

Everything here fits in the AWS free tier, but set a budget alert first anyway.

## 0. Safety first (once per account)
- Root user: MFA on, no access keys.
- Billing -> Budgets -> create a monthly cost budget of e.g. 1,000 yen with an email alert.
- Create an admin user in IAM Identity Center and use that from now on.

## 1. Tools
- AWS CLI v2, then `aws configure sso` (or `aws configure` for a lab user)
- AWS SAM CLI
- Python 3.12

Check: `aws sts get-caller-identity` shows your account.

## 2. Test locally first
    python -m unittest

## 3. Deploy
    sam build
    sam deploy --guided        # stack name: it-requests, region: ap-northeast-1, allow role creation: yes

Copy the `ApiUrl` output.

## 4. Try it
    curl -X POST "<ApiUrl>" -H "content-type: application/json" -d "{\"name\":\"Yuki\",\"email\":\"yuki@example.com\",\"category\":\"hardware\",\"description\":\"Laptop screen flickers on battery\"}"

Then paste the ApiUrl into site/index.html and open the page in a browser.
Look at the item in the DynamoDB console, and at the logs in CloudWatch -> Log groups -> /aws/lambda/...

## 5. Host the page (optional)
Create an S3 bucket, upload site/index.html, and put CloudFront in front of it with Origin Access Control
(keep the bucket private). That's the classic "static site on S3 + CloudFront" pattern.

## 6. Explore (each is an exam topic)
- IAM: open the function's role. Which actions does DynamoDBWritePolicy allow? Why not dynamodb:*?
- CloudWatch: create an alarm on the function's Errors metric that emails you.
- CloudTrail: find the event that created your table.
- Throttling: what happens with 50 requests in one second? (the API's throttling settings)
- Cost Explorer: what did this cost? (probably nothing)

## 7. Clean up
    sam delete

Check the console that the stack, table and log groups are gone.
@@text
> Tips: `email.partition("@")` splits at the first `@` into three parts. `isinstance(True, int)` is `True` in Python (bools are ints!), so check for `bool` first. `now.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")` gives the UTC timestamp. In `handle`, `json.loads` raises `json.JSONDecodeError` for bad input, and a valid JSON list still isn't a dict.
@@project it_request_form_finished Serverless IT Request Form: the finished version
The complete project. Next steps: a second function (GET /requests, protected by Cognito or an API key) that lists open requests for the IT team, an SNS topic that emails IT when a priority-1 request arrives, or a DynamoDB Stream that posts new requests to a Teams/Slack channel.
@@file handler.py
"""Lambda function behind API Gateway: accepts IT requests and stores them in DynamoDB."""

import base64
import json
import os
import uuid
from datetime import datetime, timezone

CATEGORIES = {"hardware", "software", "access", "network", "other"}


def validate(payload):
    """Problems with a request, as messages (empty = valid):
      "name is required"                        (missing, empty or only spaces; at most 100 characters)
      "name is too long"
      "email is not valid"                      (needs one "@" with a "." somewhere after it)
      "category must be one of: access, hardware, network, other, software"
      "description must be 10-2000 characters"
      "priority must be 1-4"                    (optional; must be an int, not a bool)
    """
    errors = []
    name = str(payload.get("name") or "").strip()
    if not name:
        errors.append("name is required")
    elif len(name) > 100:
        errors.append("name is too long")
    email = str(payload.get("email") or "")
    user, at, domain = email.partition("@")
    if not (user and at and "." in domain and "@" not in domain and not domain.endswith(".")):
        errors.append("email is not valid")
    if payload.get("category") not in CATEGORIES:
        errors.append("category must be one of: " + ", ".join(sorted(CATEGORIES)))
    description = str(payload.get("description") or "").strip()
    if not 10 <= len(description) <= 2000:
        errors.append("description must be 10-2000 characters")
    priority = payload.get("priority", 3)
    if isinstance(priority, bool) or not isinstance(priority, int) or not 1 <= priority <= 4:
        errors.append("priority must be 1-4")
    return errors


def build_item(payload, now, new_id):
    """The DynamoDB item for a valid request: id, created (UTC, "2026-10-31T09:00:00Z"), status "open",
    name and description stripped, email lowercased, category, priority (default 3)."""
    return {
        "id": new_id,
        "created": now.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "status": "open",
        "name": payload["name"].strip(),
        "email": payload["email"].strip().lower(),
        "category": payload["category"],
        "description": payload["description"].strip(),
        "priority": payload.get("priority", 3),
    }


def response(status, body):
    return {
        "statusCode": status,
        "headers": {"Content-Type": "application/json", "Access-Control-Allow-Origin": "*"},
        "body": json.dumps(body),
    }


def handle(event, table, now, new_id):
    """Process an API Gateway event:
    400 {"errors": ["body must be a JSON object"]} if the body isn't a JSON object,
    400 {"errors": [...]} if validation fails,
    otherwise put the item into `table` and return 201 {"id": new_id}.
    The body may be base64-encoded (event["isBase64Encoded"])."""
    raw = event.get("body") or ""
    if event.get("isBase64Encoded"):
        raw = base64.b64decode(raw).decode("utf-8")
    try:
        payload = json.loads(raw)
    except json.JSONDecodeError:
        payload = None
    if not isinstance(payload, dict):
        return response(400, {"errors": ["body must be a JSON object"]})
    errors = validate(payload)
    if errors:
        return response(400, {"errors": errors})
    table.put_item(Item=build_item(payload, now, new_id))
    return response(201, {"id": new_id})


_table = None


def lambda_handler(event, context):
    """The entry point AWS calls. The table name comes from the TABLE_NAME environment variable."""
    global _table
    if _table is None:
        import boto3  # available in the Lambda runtime

        _table = boto3.resource("dynamodb").Table(os.environ["TABLE_NAME"])
    return handle(event, _table, datetime.now(timezone.utc), uuid.uuid4().hex)
@@file test_handler.py
import base64
import json
import unittest
from datetime import datetime, timedelta, timezone

from handler import build_item, handle, validate

GOOD = {"name": " Yuki Tanaka ", "email": "Yuki.Tanaka@Sakura.example", "category": "hardware",
        "description": "Laptop screen flickers when on battery.", "priority": 2}
NOW = datetime(2026, 10, 31, 18, 0, 0, tzinfo=timezone(timedelta(hours=9)))  # 18:00 in Tokyo


class FakeTable:
    def __init__(self):
        self.items = []

    def put_item(self, Item):
        self.items.append(Item)


class ValidateTests(unittest.TestCase):
    def test_good(self):
        self.assertEqual(validate(GOOD), [])
        self.assertEqual(validate({k: v for k, v in GOOD.items() if k != "priority"}), [])

    def test_everything_wrong(self):
        self.assertEqual(validate({"name": "  ", "email": "nope", "category": "coffee", "description": "short", "priority": 9}), [
            "name is required",
            "email is not valid",
            "category must be one of: access, hardware, network, other, software",
            "description must be 10-2000 characters",
            "priority must be 1-4",
        ])

    def test_emails(self):
        for bad in ["a@b", "@b.com", "a@@b.com", "a@b.", "", None]:
            self.assertIn("email is not valid", validate({**GOOD, "email": bad}), bad)
        self.assertEqual(validate({**GOOD, "email": "a.b+it@mail.co.jp"}), [])

    def test_long_name_and_bool_priority(self):
        self.assertEqual(validate({**GOOD, "name": "x" * 101}), ["name is too long"])
        self.assertEqual(validate({**GOOD, "priority": True}), ["priority must be 1-4"])
        self.assertEqual(validate({**GOOD, "priority": "2"}), ["priority must be 1-4"])


class ItemTests(unittest.TestCase):
    def test_item(self):
        self.assertEqual(build_item(GOOD, NOW, "abc123"), {
            "id": "abc123", "created": "2026-10-31T09:00:00Z", "status": "open",
            "name": "Yuki Tanaka", "email": "yuki.tanaka@sakura.example", "category": "hardware",
            "description": "Laptop screen flickers when on battery.", "priority": 2,
        })

    def test_default_priority(self):
        payload = {k: v for k, v in GOOD.items() if k != "priority"}
        self.assertEqual(build_item(payload, NOW, "x")["priority"], 3)


class HandleTests(unittest.TestCase):
    def call(self, body, encoded=False):
        table = FakeTable()
        if encoded:
            event = {"body": base64.b64encode(body.encode()).decode(), "isBase64Encoded": True}
        else:
            event = {"body": body}
        return handle(event, table, NOW, "id-1"), table

    def test_created(self):
        res, table = self.call(json.dumps(GOOD))
        self.assertEqual(res["statusCode"], 201)
        self.assertEqual(json.loads(res["body"]), {"id": "id-1"})
        self.assertEqual(res["headers"]["Access-Control-Allow-Origin"], "*")
        self.assertEqual(len(table.items), 1)
        self.assertEqual(table.items[0]["email"], "yuki.tanaka@sakura.example")

    def test_base64_body(self):
        res, table = self.call(json.dumps(GOOD), encoded=True)
        self.assertEqual(res["statusCode"], 201)

    def test_invalid_is_not_stored(self):
        res, table = self.call(json.dumps({**GOOD, "category": "coffee"}))
        self.assertEqual(res["statusCode"], 400)
        self.assertEqual(json.loads(res["body"])["errors"], ["category must be one of: access, hardware, network, other, software"])
        self.assertEqual(table.items, [])

    def test_not_json(self):
        for body in ["not json", "[1, 2]", "", None]:
            res, table = self.call(body) if body is not None else (handle({}, FakeTable(), NOW, "x"), None)
            self.assertEqual(res["statusCode"], 400, body)
            self.assertEqual(json.loads(res["body"]), {"errors": ["body must be a JSON object"]})


if __name__ == "__main__":
    unittest.main()
@@file template.yaml
AWSTemplateFormatVersion: "2010-09-09"
Transform: AWS::Serverless-2016-10-31
Description: IT request form - API Gateway, Lambda and DynamoDB (IT Quest build)

Resources:
  RequestsTable:
    Type: AWS::DynamoDB::Table
    Properties:
      BillingMode: PAY_PER_REQUEST          # pay per request: about zero cost for a lab
      AttributeDefinitions:
        - AttributeName: id
          AttributeType: S
      KeySchema:
        - AttributeName: id
          KeyType: HASH
      PointInTimeRecoverySpecification:
        PointInTimeRecoveryEnabled: true
      SSESpecification:
        SSEEnabled: true

  SubmitFunction:
    Type: AWS::Serverless::Function
    Properties:
      Runtime: python3.12
      Handler: handler.lambda_handler
      CodeUri: .
      MemorySize: 128
      Timeout: 5
      Environment:
        Variables:
          TABLE_NAME: !Ref RequestsTable
      Policies:
        - DynamoDBWritePolicy:               # least privilege: write to this one table only
            TableName: !Ref RequestsTable
      Events:
        Submit:
          Type: HttpApi
          Properties:
            Path: /requests
            Method: POST
            ApiId: !Ref RequestsApi

  RequestsApi:
    Type: AWS::Serverless::HttpApi
    Properties:
      CorsConfiguration:
        AllowOrigins: ["*"]
        AllowMethods: [POST]
        AllowHeaders: [content-type]
      DefaultRouteSettings:
        ThrottlingBurstLimit: 10             # protect the wallet from floods
        ThrottlingRateLimit: 5

Outputs:
  ApiUrl:
    Description: POST requests here
    Value: !Sub "https://${RequestsApi}.execute-api.${AWS::Region}.amazonaws.com/requests"
  TableName:
    Value: !Ref RequestsTable
@@file site/index.html
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>IT Request</title>
<style>
  body { font-family: system-ui, sans-serif; max-width: 34rem; margin: 2rem auto; padding: 0 1rem; color: #222; }
  label { display: block; margin-top: 1rem; font-weight: 600; }
  input, select, textarea { width: 100%; padding: .5rem; font: inherit; box-sizing: border-box; }
  button { margin-top: 1.2rem; padding: .6rem 1.2rem; font: inherit; }
  #result { margin-top: 1rem; }
  .ok { color: #176b2c; } .bad { color: #a11; }
</style>
</head>
<body>
<h1>IT Request</h1>
<p>Tell the IT team what you need. We'll reply by email.</p>
<form id="form">
  <label>Name <input name="name" required maxlength="100"></label>
  <label>Email <input name="email" type="email" required></label>
  <label>Category
    <select name="category">
      <option value="hardware">Hardware</option>
      <option value="software">Software</option>
      <option value="access">Access / accounts</option>
      <option value="network">Network</option>
      <option value="other">Other</option>
    </select>
  </label>
  <label>Priority
    <select name="priority">
      <option value="4">Low</option>
      <option value="3" selected>Normal</option>
      <option value="2">High</option>
      <option value="1">Critical: I can't work</option>
    </select>
  </label>
  <label>Description <textarea name="description" rows="5" required minlength="10"></textarea></label>
  <button type="submit">Send</button>
</form>
<div id="result"></div>
<script>
  // After "sam deploy", paste the ApiUrl output here.
  const API_URL = "https://REPLACE-ME.execute-api.ap-northeast-1.amazonaws.com/requests";

  const form = document.getElementById("form");
  const result = document.getElementById("result");
  form.addEventListener("submit", async (e) => {
    e.preventDefault();
    const data = Object.fromEntries(new FormData(form));
    data.priority = Number(data.priority);
    result.textContent = "Sending...";
    try {
      const res = await fetch(API_URL, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(data) });
      const body = await res.json();
      if (res.ok) {
        result.className = "ok";
        result.textContent = "Thanks! Your request number is " + body.id.slice(0, 8) + ".";
        form.reset();
      } else {
        result.className = "bad";
        result.textContent = (body.errors || ["Something went wrong"]).join(". ");
      }
    } catch (err) {
      result.className = "bad";
      result.textContent = "Couldn't reach the server: " + err;
    }
  });
</script>
</body>
</html>
@@file LAB.md
# Lab: deploy the IT request form to AWS

Everything here fits in the AWS free tier, but set a budget alert first anyway.

## 0. Safety first (once per account)
- Root user: MFA on, no access keys.
- Billing -> Budgets -> create a monthly cost budget of e.g. 1,000 yen with an email alert.
- Create an admin user in IAM Identity Center and use that from now on.

## 1. Tools
- AWS CLI v2, then `aws configure sso` (or `aws configure` for a lab user)
- AWS SAM CLI
- Python 3.12

Check: `aws sts get-caller-identity` shows your account.

## 2. Test locally first
    python -m unittest

## 3. Deploy
    sam build
    sam deploy --guided        # stack name: it-requests, region: ap-northeast-1, allow role creation: yes

Copy the `ApiUrl` output.

## 4. Try it
    curl -X POST "<ApiUrl>" -H "content-type: application/json" -d "{\"name\":\"Yuki\",\"email\":\"yuki@example.com\",\"category\":\"hardware\",\"description\":\"Laptop screen flickers on battery\"}"

Then paste the ApiUrl into site/index.html and open the page in a browser.
Look at the item in the DynamoDB console, and at the logs in CloudWatch -> Log groups -> /aws/lambda/...

## 5. Host the page (optional)
Create an S3 bucket, upload site/index.html, and put CloudFront in front of it with Origin Access Control
(keep the bucket private). That's the classic "static site on S3 + CloudFront" pattern.

## 6. Explore (each is an exam topic)
- IAM: open the function's role. Which actions does DynamoDBWritePolicy allow? Why not dynamodb:*?
- CloudWatch: create an alarm on the function's Errors metric that emails you.
- CloudTrail: find the event that created your table.
- Throttling: what happens with 50 requests in one second? (the API's throttling settings)
- Cost Explorer: what did this cost? (probably nothing)

## 7. Clean up
    sam delete

Check the console that the stack, table and log groups are gone.

@@drill
Which AWS service runs code without managing servers?
= Lambda
= AWS Lambda
? Pay per request and execution time.

@@text
🎉 API Gateway, Lambda, DynamoDB, IAM roles with least privilege, CloudFormation/SAM, CloudWatch logs, throttling, encryption at rest, point-in-time recovery: one small project touches a large part of the Cloud Practitioner and Solutions Architect exams. Draw its architecture diagram for your portfolio, and explain the **shared responsibility** split for each piece.
