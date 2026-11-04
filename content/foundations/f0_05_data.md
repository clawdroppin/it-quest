@@lesson f0-data How Computers Store Text, Images and Files
@@text
Everything in a computer is bits. This lesson shows how those bits become letters, photos, sound and files, which explains a surprising number of everyday IT problems: garbled characters in a CSV export, an email attachment that's "too big", a photo that loses quality every time it's saved, a file that won't open because its extension is wrong.

## Text: from ASCII to Unicode

**ASCII** (1960s) gave each English letter, digit and symbol a number from 0 to 127: `A` = 65, `a` = 97, `0` = 48, space = 32. It has no `é`, no `ü`, no `日本`.

**Unicode** gives every character in every writing system a number (a **code point**), written `U+` and hex: `A` = U+0041, `é` = U+00E9, `日` = U+65E5, `😀` = U+1F600. Over 150,000 characters are defined.

Code points must still be stored as bytes. The **encoding** decides how:

- **UTF-8** (the standard for the web, Linux and modern files): ASCII characters take 1 byte, European accents 2 bytes, Japanese and Chinese 3 bytes, emoji 4 bytes. Plain English UTF-8 is identical to ASCII.
- **UTF-16**: used inside Windows and Java; 2 or 4 bytes per character.
- Older **legacy encodings**: Windows-1252 (Western Europe), **Shift_JIS** (Japanese Windows), EUC-JP.

### Mojibake

When a file is written in one encoding and read in another, you get **mojibake** (文字化け, "character transformation"): `cafÃ©` instead of `café`, or `譁�蟄怜喧縺�` instead of Japanese text. The fix is never to "repair the letters" but to open the file **with the correct encoding**. A classic case: Excel on Japanese Windows opens a UTF-8 CSV as Shift_JIS. Saving the CSV as "UTF-8 with BOM" (a marker at the start of the file) makes Excel detect it.

## Numbers

- Whole numbers are stored in binary in a fixed number of bytes: 1 byte holds 0–255, 4 bytes about ±2.1 billion, 8 bytes about ±9.2 quintillion.
- Decimal fractions are stored as **floating point**, which can't represent many decimals exactly: in most programming languages `0.1 + 0.2` gives `0.30000000000000004`. That's why money is stored as whole cents (or with decimal types), not as floats.

## Images

A **bitmap** (raster) image is a grid of pixels, each with a colour, usually 3 bytes (red, green, blue, 0–255 each). A 4000×3000 photo is 12 million pixels × 3 bytes = 36 MB uncompressed, which is why images are compressed:

| Format | Compression | Good for |
|---|---|---|
| **JPEG** | lossy | photos; quality drops each time you re-save |
| **PNG** | lossless | screenshots, logos, transparency |
| **WebP / AVIF** | lossy or lossless | web images, smaller than JPEG |
| **GIF** | lossless, 256 colours | simple animations |
| **HEIC** | lossy | iPhone photos; not every Windows app opens them |
| **SVG** | vector, not pixels | logos and icons that scale to any size |

**Lossy** compression throws away detail the eye barely notices; **lossless** keeps every bit, like a ZIP.

## Sound and video

Audio is sampled thousands of times per second (44,100 times for CD quality) and compressed (MP3, AAC, Opus). Video is a sequence of images compressed with a **codec** (H.264, H.265/HEVC, AV1) inside a **container** file (MP4, MKV, MOV). "The video plays but has no sound" or "won't play" is usually a missing codec, not a broken file.

## Compression and archives

**ZIP** (built into Windows and macOS), **7z** and **tar.gz** (Linux) bundle files and compress them losslessly. Text, CSV and logs compress very well (often 80–90% smaller); JPEGs, MP4s and already-zipped files barely shrink, because they're already compressed.

## Files and file types

- The **extension** (`.docx`, `.pdf`, `.exe`) tells the OS which program opens the file. It doesn't change the content: renaming `report.pdf` to `report.docx` doesn't convert it.
- Windows hides known extensions by default. IT staff should turn them on (File Explorer → View → Show → File name extensions), and so should anyone who opens email attachments: `invoice.pdf.exe` is a classic malware trick.
- Programs recognise many formats by their first bytes, the **magic number**: PDFs start with `%PDF`, ZIP files (including .docx and .xlsx, which are zipped XML) with `PK`, Windows programs with `MZ`.
- **Metadata**: files carry hidden information, such as the author in Office documents and GPS location in phone photos. Remove it before publishing (File → Info → Inspect Document in Office).

## Hashes: a fingerprint of a file

A **hash** function (SHA-256) turns any file into a fixed-size fingerprint. Change one bit and the hash changes completely. Uses: checking a download isn't corrupted or tampered with (compare with the hash on the vendor's site), detecting duplicate files, and storing passwords safely. On Windows: `Get-FileHash file.iso`; on Linux/macOS: `sha256sum file.iso`.

@@quiz
A CSV exported from a web system shows `cafÃ©` instead of `café` in Excel. What's wrong?
- The data is corrupted
* The UTF-8 file is being read with a different encoding; open or import it as UTF-8
- Excel doesn't support accents
- The file needs to be zipped
? This is mojibake: correct bytes, wrong encoding when reading.

@@quiz
Why does a JPEG lose quality each time it's edited and saved?
* JPEG uses lossy compression, discarding detail again on each save
- JPEGs shrink their resolution automatically
- The file system damages images
- Only PNGs can be saved
? Edit from a lossless master (PNG, TIFF, RAW) when possible.

@@quiz
A user renamed `budget.xlsx` to `budget.csv` and now it shows garbage. Why?
- CSV files can't hold numbers
* Renaming changes only the extension; the content is still a zipped Excel file, not CSV text
- The file was infected
- Excel needs updating
? Use Save As / Export to convert formats.

@@quiz
How many bytes does UTF-8 use for a typical Japanese kanji?
- 1
- 2
* 3
- 8
? ASCII is 1 byte, European accents 2, CJK characters 3, emoji 4.

@@flashcards
ASCII :: 7-bit encoding of English letters, digits and symbols
Unicode :: a number (code point) for every character
UTF-8 :: variable-length Unicode encoding; the web standard
Shift_JIS :: legacy Japanese Windows encoding
mojibake :: garbled text from the wrong encoding
BOM :: byte-order mark at the start of a text file
floating point :: approximate decimals; don't use for money
lossy / lossless :: discards detail / keeps everything
JPEG / PNG :: photos / screenshots and logos
codec / container :: how video is compressed / the file wrapping it
magic number :: the identifying first bytes of a file
SHA-256 :: a file fingerprint (hash)
`Get-FileHash` / `sha256sum` :: compute a hash

@@drill
What is the ASCII code of capital `A`?
= 65
? Lowercase `a` is 97: 32 more.

@@drill
How many bytes per pixel does uncompressed 24-bit RGB colour use?
= 3
? One byte each for red, green and blue.

@@exercise Encoding sizes and mojibake
For each word, print its number of characters and its size in bytes in UTF-8 and UTF-16 (little-endian, without a BOM: `encode("utf-16-le")`). Then reproduce mojibake: encode `"café"` as UTF-8 and decode the bytes as `"cp1252"` (Windows Western), and print the garbled result.
@@lang python
@@starter
words = ["hello", "café", "東京", "😀"]
for w in words:
    pass  # print word, characters, UTF-8 bytes, UTF-16 bytes
@@expected
hello: 5 chars, UTF-8 5 bytes, UTF-16 10 bytes
café: 4 chars, UTF-8 5 bytes, UTF-16 8 bytes
東京: 2 chars, UTF-8 6 bytes, UTF-16 4 bytes
😀: 1 chars, UTF-8 4 bytes, UTF-16 4 bytes
Mojibake: cafÃ©
@@requires
.encode("utf-8")
.decode("cp1252")
@@hint
`len(w.encode("utf-8"))`. For mojibake: `"café".encode("utf-8").decode("cp1252")`.
@@solution
words = ["hello", "café", "東京", "😀"]
for w in words:
    print(f"{w}: {len(w)} chars, UTF-8 {len(w.encode('utf-8'))} bytes, UTF-16 {len(w.encode('utf-16-le'))} bytes")

garbled = "café".encode("utf-8").decode("cp1252")
print("Mojibake:", garbled)

@@exercise Identify files by their magic numbers
Write `identify(data)` that recognises a file from its first bytes: `b"%PDF"` → `"PDF document"`, `b"PK"` → `"ZIP archive (or .docx/.xlsx)"`, `b"MZ"` → `"Windows program"`, `b"\x89PNG"` → `"PNG image"`, `b"\xff\xd8\xff"` → `"JPEG image"`, otherwise `"unknown"`. Check the sample "attachments" and warn (print `WARNING`) when the detected type doesn't match the extension, e.g. a Windows program named `.pdf`.
@@lang python
@@starter
attachments = {
    "invoice.pdf": b"%PDF-1.7 ...",
    "report.docx": b"PK\x03\x04 ...",
    "invoice2.pdf": b"MZ\x90\x00 ...",
    "photo.jpg": b"\xff\xd8\xff\xe0 ...",
    "logo.png": b"\x89PNG\r\n ...",
}
EXPECTED = {".pdf": "PDF document", ".docx": "ZIP archive (or .docx/.xlsx)", ".jpg": "JPEG image", ".png": "PNG image"}

def identify(data):
    raise NotImplementedError
@@expected
invoice.pdf: PDF document
report.docx: ZIP archive (or .docx/.xlsx)
invoice2.pdf: Windows program  WARNING: extension doesn't match content
photo.jpg: JPEG image
logo.png: PNG image
@@requires
def identify
startswith(
@@hint
Loop over a list of `(prefix, name)` pairs and return the first where `data.startswith(prefix)`. Get the extension with `name[name.rfind("."):]`.
@@solution
attachments = {
    "invoice.pdf": b"%PDF-1.7 ...",
    "report.docx": b"PK\x03\x04 ...",
    "invoice2.pdf": b"MZ\x90\x00 ...",
    "photo.jpg": b"\xff\xd8\xff\xe0 ...",
    "logo.png": b"\x89PNG\r\n ...",
}
EXPECTED = {".pdf": "PDF document", ".docx": "ZIP archive (or .docx/.xlsx)", ".jpg": "JPEG image", ".png": "PNG image"}
SIGNATURES = [
    (b"%PDF", "PDF document"),
    (b"PK", "ZIP archive (or .docx/.xlsx)"),
    (b"MZ", "Windows program"),
    (b"\x89PNG", "PNG image"),
    (b"\xff\xd8\xff", "JPEG image"),
]

def identify(data):
    for prefix, kind in SIGNATURES:
        if data.startswith(prefix):
            return kind
    return "unknown"

for name, data in attachments.items():
    kind = identify(data)
    ext = name[name.rfind("."):]
    flag = "" if EXPECTED.get(ext) == kind else "  WARNING: extension doesn't match content"
    print(f"{name}: {kind}{flag}")
