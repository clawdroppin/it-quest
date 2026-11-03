@@lesson m8-binary Binary Data, Buffers and Cryptography
@@concepts files, networking
@@text
Network packets, file formats, hashes, encryption and images are all **bytes**, not text. JavaScript handles bytes with `ArrayBuffer`, typed arrays and Node's `Buffer`, and does cryptography with `node:crypto` (or the standard Web Crypto API).

## Bytes in JavaScript

```javascript
const buf = Buffer.from("IT✓", "utf8");
console.log(buf, buf.length, "bytes for", "IT✓".length, "characters");
console.log(buf.toString("hex"), buf.toString("base64"), Buffer.from("SVTinJM=", "base64").toString());

const bytes = new Uint8Array([192, 168, 1, 10]);
console.log(bytes.join("."), Buffer.from(bytes.buffer).readUInt32BE(0));
```

- An `ArrayBuffer` is raw memory. **Typed arrays** (`Uint8Array`, `Int16Array`, `Float32Array`...) are views of it with a fixed element type.
- `Buffer` is Node's subclass of `Uint8Array` with extra helpers: encodings (`utf8`, `hex`, `base64`, `base64url`, `latin1`) and number readers/writers.
- Text ↔ bytes in standard JavaScript: `new TextEncoder().encode(str)` and `new TextDecoder().decode(bytes)`.

## Reading binary formats

Binary formats have fields at fixed offsets, in **big-endian** (network order, most significant byte first) or **little-endian** (x86, most file formats on Windows):

```javascript
const header = Buffer.alloc(8);
header.writeUInt16BE(0xCAFE, 0);       // magic number
header.writeUInt8(2, 2);               // version
header.writeUInt8(0b101, 3);           // flags
header.writeUInt32LE(1024, 4);         // payload length, little-endian
console.log(header.toString("hex"));
console.log(header.readUInt16BE(0).toString(16), header.readUInt8(2), header[3] & 0b100 ? "flag 3 set" : "", header.readUInt32LE(4));

const view = new DataView(header.buffer, header.byteOffset, header.length);
console.log(view.getUint32(4, true));  // DataView: the standard way; true = little-endian
```

Bitwise operators work on 32-bit integers: `&` (and), `|` (or), `^` (xor), `~` (not), `<<`, `>>` (signed), `>>>` (unsigned). Use `>>> 0` to turn a result back into an unsigned number.

## Hashing

A **hash** maps any data to a fixed-size fingerprint. Use it to verify downloads, detect changed files and deduplicate:

```javascript
const crypto = require("node:crypto");
console.log(crypto.createHash("sha256").update("hello").digest("hex"));
const h = crypto.createHash("sha256");
for (const chunk of ["hel", "lo"]) h.update(chunk);          // streaming: same result
console.log(h.digest("hex").slice(0, 16));
console.log(crypto.randomUUID(), crypto.randomBytes(8).toString("hex"));
```

- **SHA-256** for integrity. MD5 and SHA-1 are broken for security purposes (still fine as non-security checksums).
- **HMAC** (`crypto.createHmac("sha256", key)`) proves a message came from someone with the key: webhook signatures, signed cookies, API request signing.
- Compare secrets with `crypto.timingSafeEqual`, not `===`, to avoid timing attacks.
- **Passwords** need a slow, salted algorithm: `crypto.scrypt`, or argon2/bcrypt libraries. Never store a plain SHA-256 of a password.
- Random values for security (tokens, IDs, keys) come from `crypto.randomBytes`/`randomUUID`, **never** `Math.random()`.

## Encryption

Use **authenticated encryption** (AES-256-GCM): it both hides data and detects tampering.

```javascript
const crypto = require("node:crypto");
const key = crypto.randomBytes(32);
function encrypt(text) {
  const iv = crypto.randomBytes(12);                        // never reuse an IV with the same key
  const cipher = crypto.createCipheriv("aes-256-gcm", key, iv);
  const data = Buffer.concat([cipher.update(text, "utf8"), cipher.final()]);
  return Buffer.concat([iv, cipher.getAuthTag(), data]).toString("base64");
}
function decrypt(b64) {
  const raw = Buffer.from(b64, "base64");
  const decipher = crypto.createDecipheriv("aes-256-gcm", key, raw.subarray(0, 12));
  decipher.setAuthTag(raw.subarray(12, 28));
  return Buffer.concat([decipher.update(raw.subarray(28)), decipher.final()]).toString("utf8");
}
const secret = encrypt("db password: hunter2");
console.log(secret.length > 20, decrypt(secret));
```

Don't invent your own cryptography or formats. Use well-known algorithms, libraries and protocols (TLS, age, libsodium), and keep keys out of source code.

@@quiz
Which should you use to generate a session token?
- `Math.random().toString(36)`
* `crypto.randomBytes(32).toString("base64url")`
- `Date.now()`
- An incrementing counter
? Math.random is predictable; crypto random is not.

@@quiz
How should passwords be stored?
- Encrypted with AES
- As a SHA-256 hash
* With a slow, salted password hash like scrypt, bcrypt or argon2
- In plain text in a private database
? Fast hashes let attackers try billions of guesses per second.

@@quiz
What does AES-GCM's authentication tag provide?
* Detection of any tampering with the encrypted data
- Compression
- Faster encryption
- A shorter key
? Decryption fails if the data was modified.

@@flashcards
ArrayBuffer :: raw binary memory
Uint8Array :: typed array view of bytes
Buffer :: Node's byte array with encodings and number helpers
Big-endian :: most significant byte first (network order)
`DataView` :: read/write numbers at offsets with chosen endianness
SHA-256 :: secure hash for integrity
HMAC :: keyed hash proving authenticity
`timingSafeEqual` :: constant-time comparison of secrets
scrypt / argon2 / bcrypt :: slow, salted password hashing
AES-256-GCM :: authenticated encryption

@@exercise Parse an IPv4 packet header
Write `parseIpv4Header(buf)` for a `Buffer` holding an IPv4 header and return:

`{ version, ihl, totalLength, ttl, protocol, source, destination, checksumOk }`

- byte 0: version (high 4 bits), IHL = header length in 32-bit words (low 4 bits)
- bytes 2–3: total length (big-endian); byte 8: TTL; byte 9: protocol number, returned as a name for 1 → `"ICMP"`, 6 → `"TCP"`, 17 → `"UDP"`, else the number
- bytes 12–15 source, 16–19 destination (dotted strings)
- `checksumOk`: the ones' complement sum of all 16-bit words of the header (IHL × 4 bytes, **including** the checksum field) folded to 16 bits must equal `0xFFFF`

Throw `Error("not IPv4")` if the version isn't 4 or the buffer is shorter than IHL × 4.
@@starter
function parseIpv4Header(buf) {
  throw new Error("TODO: not done yet");
}

@@tests
const pkt = Buffer.from("450000730000400040110000c0a80001c0a800c7", "hex");
let sum = 0;
for (let i = 0; i < 20; i += 2) sum += pkt.readUInt16BE(i);
while (sum > 0xffff) sum = (sum & 0xffff) + (sum >>> 16);
pkt.writeUInt16BE(~sum & 0xffff, 10);
check(parseIpv4Header(pkt), { version: 4, ihl: 5, totalLength: 115, ttl: 64, protocol: "UDP", source: "192.168.0.1", destination: "192.168.0.199", checksumOk: true }, "valid header");
const corrupted = Buffer.from(pkt);
corrupted[8] = 1;
check(parseIpv4Header(corrupted).checksumOk, false, "corrupted");
let msg = "";
try { parseIpv4Header(Buffer.from("6000", "hex")); } catch (e) { msg = e.message; }
check(msg, "not IPv4", "IPv6 or short");
@@hint
`const version = buf[0] >> 4; const ihl = buf[0] & 0x0f;`. Addresses: `[...buf.subarray(12, 16)].join(".")`. For the checksum, add `buf.readUInt16BE(i)` for every even `i` below `ihl * 4`, then fold with `while (sum > 0xffff) sum = (sum & 0xffff) + (sum >>> 16);`.
@@solution
function parseIpv4Header(buf) {
  if (buf.length < 1) throw new Error("not IPv4");
  const version = buf[0] >> 4;
  const ihl = buf[0] & 0x0f;
  if (version !== 4 || buf.length < ihl * 4 || ihl < 5) throw new Error("not IPv4");
  const names = { 1: "ICMP", 6: "TCP", 17: "UDP" };
  let sum = 0;
  for (let i = 0; i < ihl * 4; i += 2) sum += buf.readUInt16BE(i);
  while (sum > 0xffff) sum = (sum & 0xffff) + (sum >>> 16);
  return {
    version,
    ihl,
    totalLength: buf.readUInt16BE(2),
    ttl: buf[8],
    protocol: names[buf[9]] ?? buf[9],
    source: [...buf.subarray(12, 16)].join("."),
    destination: [...buf.subarray(16, 20)].join("."),
    checksumOk: sum === 0xffff,
  };
}

@@exercise Signed webhooks
Webhook senders sign the body so receivers can trust it. Write:

- `sign(secret, body, timestamp)`: returns `"t=<timestamp>,v1=<hex>"` where hex is the HMAC-SHA256 of `"<timestamp>.<body>"` with `secret`
- `verify(secret, body, header, now, toleranceSec = 300)`: parses the header, returns `{ ok: true }` if the signature matches **and** the timestamp is within `toleranceSec` of `now` (both in seconds); otherwise `{ ok: false, reason }` with reason `"malformed"`, `"expired"` or `"bad signature"`. Compare signatures with `crypto.timingSafeEqual` (check lengths first: it throws on different lengths).
@@starter
const crypto = require("node:crypto");

function sign(secret, body, timestamp) {
  throw new Error("TODO: not done yet");
}

function verify(secret, body, header, now, toleranceSec = 300) {
  throw new Error("TODO: not done yet");
}

@@tests
const body = JSON.stringify({ event: "ticket.created", id: 42 });
const header = sign("whsec_test", body, 1762171200);
check(/^t=1762171200,v1=[0-9a-f]{64}$/.test(header), true, "format");
check(verify("whsec_test", body, header, 1762171260), { ok: true }, "valid");
check(verify("whsec_test", body + " ", header, 1762171260), { ok: false, reason: "bad signature" }, "tampered body");
check(verify("other", body, header, 1762171260), { ok: false, reason: "bad signature" }, "wrong secret");
check(verify("whsec_test", body, header, 1762171200 + 301), { ok: false, reason: "expired" }, "replayed later");
check(verify("whsec_test", body, "v1=abc", 1762171200), { ok: false, reason: "malformed" }, "malformed");
check(verify("whsec_test", body, "t=1762171200,v1=abcd", 1762171200), { ok: false, reason: "bad signature" }, "short signature");
@@hint
Parse with `Object.fromEntries(header.split(",").map((p) => p.split("=")))`; require `t` (digits) and `v1`. Compute the expected hex with `crypto.createHmac("sha256", secret).update(`${t}.${body}`).digest("hex")`. Compare `Buffer.from(v1)` with `Buffer.from(expected)` only if the lengths match.
@@solution
const crypto = require("node:crypto");

function hmac(secret, timestamp, body) {
  return crypto.createHmac("sha256", secret).update(`${timestamp}.${body}`).digest("hex");
}

function sign(secret, body, timestamp) {
  return `t=${timestamp},v1=${hmac(secret, timestamp, body)}`;
}

function verify(secret, body, header, now, toleranceSec = 300) {
  const parts = Object.fromEntries(header.split(",").map((p) => {
    const eq = p.indexOf("=");
    return [p.slice(0, eq), p.slice(eq + 1)];
  }));
  if (!/^\d+$/.test(parts.t ?? "") || !parts.v1) return { ok: false, reason: "malformed" };
  const timestamp = Number(parts.t);
  if (Math.abs(now - timestamp) > toleranceSec) return { ok: false, reason: "expired" };
  const expected = Buffer.from(hmac(secret, timestamp, body));
  const given = Buffer.from(parts.v1);
  if (given.length !== expected.length || !crypto.timingSafeEqual(given, expected)) {
    return { ok: false, reason: "bad signature" };
  }
  return { ok: true };
}
