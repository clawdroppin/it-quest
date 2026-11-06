@@lesson j1-values Numbers, Strings and Comparisons
@@concepts numbers, strings
@@text
## Numbers

JavaScript has one number type for whole numbers and decimals:

```javascript
console.log(10 / 4);          // 2.5
console.log(10 % 3);          // 1
console.log(2 ** 10);         // 1024
console.log(Math.round(2.567 * 10) / 10);   // 2.6
console.log((2.567).toFixed(1));             // "2.6" (a string!)
console.log(Math.floor(7 / 2), Math.ceil(7 / 2), Math.max(3, 9, 4));
console.log(0.1 + 0.2);       // 0.30000000000000004: floating point, like every language
```

Converting text to numbers:

```javascript
console.log(Number("42"), parseInt("443px"), parseFloat("2.5GB"));
console.log(Number("abc"));             // NaN: "not a number"
console.log(Number.isNaN(Number("x"))); // how to test for it
```

## Strings

```javascript
const fqdn = "  Web01.Contoso.Local ";
console.log(fqdn.trim().toLowerCase());
console.log(fqdn.trim().split(".")[0]);
console.log("error: disk full".includes("disk"));
console.log("web01".startsWith("web"), "app.log".endsWith(".log"));
console.log("a-b-c".replaceAll("-", ":"));
console.log("7".padStart(3, "0"));     // "007"
console.log("server".length, "server"[0], "server".at(-1));
```

## Comparisons: always use ===

```javascript
console.log(5 === 5, 5 === "5");   // true false
console.log(5 == "5");             // true: == converts types first. Avoid it.
console.log("10" < "9");           // true! Strings compare alphabetically
console.log(10 < 9);               // false
```

`===` (strict equality) and `!==` compare without converting. Using `==` leads to surprises like `0 == ""` being `true`.

## Truthy and falsy

In an `if`, these count as false: `false`, `0`, `""`, `null`, `undefined`, `NaN`. Everything else is true, including `"0"` and `[]`.

`null` means "no value, on purpose"; `undefined` means "never set". The `??` operator picks a default only for those two: `port ?? 80`.
## Numbers that look wrong

Because numbers are stored in binary floating point, decimals like 0.1 can't be stored exactly. For money, count in the **smallest unit** (yen, or cents) as whole numbers, and format only for display. When you need exact large integers, such as IDs from a database, JavaScript has **BigInt**: `12345678901234567890n`. Ordinary numbers are only exact up to `Number.MAX_SAFE_INTEGER` (about 9 quadrillion).

For display, `toLocaleString` formats numbers for a language and region: `(1234567.891).toLocaleString("en-US")` gives "1,234,567.891", and `Intl.NumberFormat("ja-JP", { style: "currency", currency: "JPY" })` formats yen.

@@quiz
What does `"5" + 3` evaluate to?
* `"53"`
- `8`
- `NaN`
- an error
? With a string on either side, `+` joins strings. `"5" - 3` would give 2, which is why mixing types is risky.

@@quiz
Which comparison should you use in JavaScript?
* `===`
- `==`
- `=`
- `equals()`
? `===` doesn't convert types, so `5 === "5"` is false.

@@quiz
`const port = 0; console.log(port || 8080, port ?? 8080);` prints...
* `8080 0`
- `0 0`
- `8080 8080`
- `0 8080`
? `||` replaces any falsy value (including 0). `??` only replaces null and undefined.

@@drill
Which operator compares two values without type conversion?
= ===
? == converts types first, with surprising results.

@@flashcards
=== :: strict equality: no type conversion
== :: loose equality: converts types first (avoid)
NaN :: "not a number"; test with Number.isNaN
?? :: default only for null or undefined
|| :: default for any falsy value
falsy values :: false, 0, "", null, undefined, NaN
toFixed(n) :: format with n decimals (returns a string)
padStart(3, "0") :: "7" becomes "007"

@@exercise Bytes to human
Write `humanSize(bytes)` that returns a string with one decimal: under 1024 → `"512 B"` (no decimal for bytes), under 1024² → `"1.5 KB"`, under 1024³ → `"MB"`, otherwise `"GB"`.
@@starter
function humanSize(bytes) {
  throw new Error("TODO: not done yet");
}

@@tests
check(humanSize(512), "512 B", "humanSize(512)");
check(humanSize(1536), "1.5 KB", "humanSize(1536)");
check(humanSize(5 * 1024 * 1024), "5.0 MB", "humanSize(5 MB)");
check(humanSize(1.3 * 1024 ** 3), "1.3 GB", "humanSize(1.3 GB)");
@@hint
Test the smallest unit first. For bytes, return a template literal with `${bytes} B`. For kilobytes, divide by 1024 and use `.toFixed(1)` inside the template literal, and so on with `1024 ** 2` and `1024 ** 3`.
@@solution
function humanSize(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 ** 2) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 ** 3) return `${(bytes / 1024 ** 2).toFixed(1)} MB`;
  return `${(bytes / 1024 ** 3).toFixed(1)} GB`;
}

@@exercise Ticket IDs
Write `ticketId(n)` that turns a number into a help desk ticket ID: `INC` followed by the number padded to **6** digits. `42` → `INC000042`.
@@starter
function ticketId(n) {
  throw new Error("TODO: not done yet");
}

@@tests
check(ticketId(42), "INC000042", "ticketId(42)");
check(ticketId(123456), "INC123456", "ticketId(123456)");
check(ticketId(7), "INC000007", "ticketId(7)");
@@hint
`"INC" + String(n).padStart(6, "0")`
@@solution
function ticketId(n) {
  return "INC" + String(n).padStart(6, "0");
}
