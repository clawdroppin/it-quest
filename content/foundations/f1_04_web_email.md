@@lesson f1-web-email How the Web and Email Work
@@text
"The website won't load" and "my email didn't arrive" are two of the most common things anyone in IT hears. Both are chains of several steps, and troubleshooting means finding the broken link. This lesson walks through each chain.

## What happens when you open a web page

You type `https://portal.example.com/tickets?id=9` and press Enter:

1. **URL parsing**: scheme `https`, host `portal.example.com`, path `/tickets`, query `id=9`. The default port for HTTPS is 443 (HTTP is 80).
2. **DNS**: the browser asks a resolver for the host's IP address (checking caches first).
3. **TCP connection** to that IP on port 443 (the three-way handshake: SYN, SYN-ACK, ACK). Newer HTTP/3 uses QUIC over UDP instead.
4. **TLS handshake**: the server proves its identity with a **certificate**, and both sides agree encryption keys.
5. **HTTP request**: `GET /tickets?id=9` with headers (cookies, the browser type, accepted languages).
6. **HTTP response**: a **status code**, headers, and the body (HTML).
7. The browser parses the HTML and requests everything else it references: CSS, JavaScript, images, fonts, often from other hosts (CDNs), each going through the same steps.

## HTTP status codes

| Code | Meaning | What it usually tells you |
|---|---|---|
| **200** OK | success | |
| **301 / 302** | moved (permanent / temporary) | redirects, e.g. http → https |
| **304** | not modified | the cached copy is still good |
| **400** | bad request | the client sent something malformed |
| **401** | unauthorised | not logged in |
| **403** | forbidden | logged in but no permission |
| **404** | not found | wrong URL, or deleted page |
| **429** | too many requests | rate-limited |
| **500** | internal server error | the application crashed |
| **502 / 504** | bad gateway / gateway timeout | a proxy or load balancer couldn't reach the app behind it |
| **503** | service unavailable | overloaded or in maintenance |

4xx means the **client** side did something the server won't accept; 5xx means the **server** side failed. A 403 for one user and 200 for another is a permissions issue, not an outage.

## HTTPS and certificates

A **TLS certificate** binds a domain name to a public key, signed by a trusted **Certificate Authority** (CA), such as Let's Encrypt, DigiCert or a company's own internal CA. The browser checks the name matches, the dates are valid, and the chain leads to a CA it trusts. Errors you'll see:

- **Expired certificate**: someone forgot to renew it (automate renewals!).
- **Name mismatch**: the certificate is for `www.example.com` but you visited `example.com`.
- **Untrusted issuer**: an internal CA not installed on that device, or a network device intercepting traffic.
- A **wrong clock** on the PC makes valid certificates look expired or not yet valid.

## Proxies, CDNs and load balancers

- A **forward proxy** sits between staff and the internet (filtering, logging).
- A **reverse proxy / load balancer** sits in front of web servers, spreading requests over several of them and terminating TLS.
- A **CDN** (Cloudflare, Akamai, CloudFront) caches content in data centres near users.

These are why "the site works for me but not for them" can be true: different proxies, caches or regions.

## How email travels

You send a message from `ana@example.com` to `ken@client.jp`:

1. Your mail app (Outlook) hands the message to your organisation's mail server using **SMTP** (submission on port **587** with TLS).
2. Your server looks up the **MX** record for `client.jp` in DNS to find their mail server.
3. It delivers the message to that server with **SMTP** (port **25**).
4. The receiving server runs spam and malware checks, then stores it in Ken's mailbox.
5. Ken's app fetches it with **IMAP** (port 993 with TLS) or, in Microsoft 365/Exchange, with Exchange's own protocols. **POP3** (995) downloads and usually deletes from the server; it's rarely used now.

### Proving who sent it: SPF, DKIM, DMARC

Email "From" addresses are trivially forged, so receiving servers check three DNS-published policies:

- **SPF**: a TXT record listing which servers may send mail for the domain.
- **DKIM**: the sending server signs the message; the public key is in DNS.
- **DMARC**: tells receivers what to do when SPF/DKIM fail (`none`, `quarantine`, `reject`) and where to send reports.

Missing or wrong records are why a company's legitimate mail (newsletters from a marketing service, scanner-to-email) lands in spam, and why phishing that spoofs your domain gets through.

### "My email didn't arrive"

1. Is it in **spam/junk** or **quarantine**?
2. **Message trace** in the mail admin portal (Microsoft 365, Google Workspace) shows exactly where it went: delivered, filtered, bounced.
3. A **bounce** (NDR) message has a code: `550 5.1.1` user unknown, `552` mailbox full, `5.7.x` rejected by policy (SPF/DMARC, spam).
4. Check the **MX** and SPF/DKIM/DMARC records for the domain.

## Tools

- Browser **developer tools** (F12) → Network tab: every request with its status code and timing.
- `curl -I https://example.com` shows the response headers and status code.
- `nslookup -type=mx example.com` shows mail servers; `nslookup -type=txt example.com` shows SPF.
- Online checkers (MXToolbox) test mail DNS records and blocklists.

@@quiz
A page returns 403 for one user but works for others. What's the most likely cause?
- The server is down
* That user lacks permission; it's an access issue, not an outage
- DNS is broken
- The certificate expired
? 403 = authenticated but forbidden.

@@quiz
A user's browser says every HTTPS site has an invalid certificate. Other PCs are fine. What should you check first?
* The PC's date and time
- The company firewall
- The web servers' certificates
- The user's password
? A wrong clock makes valid certificates look expired.

@@quiz
Which DNS record tells other mail servers where to deliver mail for a domain?
- A
- CNAME
* MX
- PTR
? Mail exchanger.

@@quiz
What does DMARC do?
- Encrypts email
* Tells receivers what to do with mail that fails SPF/DKIM checks, and where to report
- Stores mailboxes
- Replaces SMTP
? Policies: none, quarantine, reject.

@@quiz
What does a 502 Bad Gateway usually mean?
- The user typed a wrong URL
* A proxy or load balancer couldn't get a valid response from the application server behind it
- The user isn't logged in
- The page moved
? Look at the application behind the proxy.

@@flashcards
URL parts :: scheme, host, port, path, query
HTTPS port :: 443 (HTTP 80)
TLS certificate :: binds a domain to a key, signed by a CA
200 / 301 / 404 :: OK / moved / not found
401 / 403 :: not logged in / not allowed
500 / 502 / 503 :: app error / bad gateway / unavailable
reverse proxy :: in front of web servers (TLS, load balancing)
CDN :: caches content near users
SMTP :: sending/relaying mail (25, 587)
IMAP :: reading mail on the server (993)
MX record :: where to deliver a domain's mail
SPF :: allowed sending servers (TXT)
DKIM :: cryptographic signature on mail
DMARC :: policy when checks fail
message trace :: follow one email through the system
NDR / bounce :: non-delivery report with a code

@@drill
What port does HTTPS use by default?
= 443
? HTTP is 80.

@@drill
Which HTTP status code means "not found"?
= 404
? 4xx: the client side.

@@drill
What port does SMTP submission from a mail client use (with STARTTLS)?
= 587
? Server-to-server SMTP is 25.

@@exercise Parse URLs and explain status codes
Use `urllib.parse.urlsplit` to print the scheme, host, port (the explicit one, or 443/80 by default for https/http), path and query of each URL. Then write `explain(code)` returning `"success"` for 2xx, `"redirect"` for 3xx, `"client error"` for 4xx and `"server error"` for 5xx, and print the explanation for each log entry's code.
@@lang python
@@starter
from urllib.parse import urlsplit

urls = ["https://portal.example.com/tickets?id=9", "http://intranet.local:8080/wiki/VPN", "https://example.jp"]
log = [("GET /", 200), ("GET /old", 301), ("GET /admin", 403), ("GET /api", 502)]
@@expected
https://portal.example.com/tickets?id=9: scheme=https host=portal.example.com port=443 path=/tickets query=id=9
http://intranet.local:8080/wiki/VPN: scheme=http host=intranet.local port=8080 path=/wiki/VPN query=-
https://example.jp: scheme=https host=example.jp port=443 path=/ query=-
GET / -> 200 success
GET /old -> 301 redirect
GET /admin -> 403 client error
GET /api -> 502 server error
@@requires
urlsplit(
def explain
// 100
@@hint
`u = urlsplit(url)`; `u.port or (443 if u.scheme == "https" else 80)`; `u.path or "/"`. For the status class: `code // 100`.
@@solution
from urllib.parse import urlsplit

urls = ["https://portal.example.com/tickets?id=9", "http://intranet.local:8080/wiki/VPN", "https://example.jp"]
log = [("GET /", 200), ("GET /old", 301), ("GET /admin", 403), ("GET /api", 502)]

for url in urls:
    u = urlsplit(url)
    port = u.port or (443 if u.scheme == "https" else 80)
    print(f"{url}: scheme={u.scheme} host={u.hostname} port={port} path={u.path or '/'} query={u.query or '-'}")

def explain(code):
    return {2: "success", 3: "redirect", 4: "client error", 5: "server error"}.get(code // 100, "unknown")

for request, code in log:
    print(f"{request} -> {code} {explain(code)}")

@@exercise Read an SPF record
An SPF record looks like `v=spf1 ip4:203.0.113.0/24 include:spf.protection.outlook.com -all`. Write `spf_allows(record, ip)` that returns `"pass"` if the IP is inside any `ip4:` network in the record (use `ipaddress`), and otherwise the result of the final mechanism: `-all` → `"fail"`, `~all` → `"softfail"`, `?all` → `"neutral"`. (Ignore `include:`; real checkers follow it into other DNS records.) Test the three sending IPs against both records.
@@lang python
@@starter
import ipaddress

records = {
    "example.com": "v=spf1 ip4:203.0.113.0/24 ip4:198.51.100.7 include:spf.protection.outlook.com -all",
    "shop.example": "v=spf1 ip4:192.0.2.10 ~all",
}
senders = ["203.0.113.45", "198.51.100.7", "192.0.2.10"]
@@expected
example.com from 203.0.113.45: pass
example.com from 198.51.100.7: pass
example.com from 192.0.2.10: fail
shop.example from 203.0.113.45: softfail
shop.example from 198.51.100.7: softfail
shop.example from 192.0.2.10: pass
@@requires
def spf_allows
ip4:
ip_network(
@@hint
Split the record on spaces. For each part starting with `ip4:`, `ipaddress.ip_network(part[4:], strict=False)` (a single address becomes a /32). Check `ipaddress.ip_address(ip) in net`.
@@solution
import ipaddress

records = {
    "example.com": "v=spf1 ip4:203.0.113.0/24 ip4:198.51.100.7 include:spf.protection.outlook.com -all",
    "shop.example": "v=spf1 ip4:192.0.2.10 ~all",
}
senders = ["203.0.113.45", "198.51.100.7", "192.0.2.10"]
ALL = {"-all": "fail", "~all": "softfail", "?all": "neutral", "+all": "pass"}

def spf_allows(record, ip):
    addr = ipaddress.ip_address(ip)
    result = "neutral"
    for part in record.split():
        if part.startswith("ip4:"):
            if addr in ipaddress.ip_network(part[4:], strict=False):
                return "pass"
        elif part in ALL:
            result = ALL[part]
    return result

for domain, record in records.items():
    for ip in senders:
        print(f"{domain} from {ip}: {spf_allows(record, ip)}")
