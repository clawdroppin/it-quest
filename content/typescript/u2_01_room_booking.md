@@lesson t9-room-booking Meeting Room Booking (application architecture)
@@text
Every company has this system, and most people hate theirs. You'll build the core of a good one: rooms, holds that expire, confirmations, cancellations, notifications, and a "find me a free room" search, using the architecture from the *Application Architecture* lessons.

```text
node main.ts        plays a morning at the office through the system
node --test         runs the tests
npx tsc             type-checks everything (after npm install)
```

## The design

```text
         main.ts  (or a web API, a Slack bot, a CLI...)
            │
            ▼
   service.ts: BookingService ──► ports: BookingRepo, Clock, Notifier
            │                          ▲
            ▼                          │ implemented by
   domain.ts: types + pure rules    adapters.ts: MemoryRepo, ConsoleNotifier, FakeNotifier
```

- **domain.ts** has no I/O and no time: types like `Booking` (a discriminated union of `held`, `confirmed` and `cancelled`) and pure functions like `overlaps` and `findFreeRooms`. Pure functions are trivial to test.
- **service.ts** holds the use cases. It reaches the outside world only through **ports** (interfaces). Expected failures come back as `Result` values, not exceptions.
- **adapters.ts** implements the ports. A PostgreSQL repo or a Slack notifier would be one more class here, and nothing else would change.

## Rules

- The office (in Tokyo, UTC+9) is open 08:00-20:00. Bookings are on 15-minute steps, at most 4 hours.
- Booking is two steps: **hold** (the room is reserved for 10 minutes) then **confirm**. Holds that aren't confirmed **expire** and give the room back. Ticket and hotel sites work the same way.
- Without a room id, the system picks the **smallest** free room that has the capacity and equipment.
- Only the organizer can confirm or cancel.

## Your tasks

1. `overlaps` in `domain.ts`: two slots overlap unless one ends before (or exactly when) the other starts.
2. `findFreeRooms` in `domain.ts`: filter by fit and availability, sort smallest first.
3. `hold` in `service.ts`: validation, the two ways of picking a room, and creating the held booking.
4. `expireHolds` in `service.ts`: cancel stale holds and notify their organizers.

The comments above each function give the exact behaviour and messages. `node --test` after each one.
@@project room_booking Meeting Room Booking
Write the four functions that throw `TODO` until `node --test` passes, then run `node main.ts`.
@@file domain.ts
// domain.ts: the types and pure rules of room booking. No I/O, no clock, no classes needed.

export type Equipment = "projector" | "video" | "whiteboard";

export type Room = { id: string; name: string; capacity: number; equipment: Equipment[] };

export type Slot = { start: Date; end: Date };

export type BookingRequest = Slot & {
  roomId?: string; // leave it out to get the smallest free room that fits
  organizer: string;
  title: string;
  attendees: number;
  needs: Equipment[];
};

// A booking is in exactly one state, and each state has its own fields. TypeScript makes it
// impossible to read `reason` from a confirmed booking, or to forget `expiresAt` on a hold.
type BookingBase = Slot & { id: number; roomId: string; organizer: string; title: string; attendees: number };
export type Booking =
  | (BookingBase & { status: "held"; expiresAt: Date })
  | (BookingBase & { status: "confirmed"; confirmedAt: Date })
  | (BookingBase & { status: "cancelled"; reason: string });

// Expected failures are values, not exceptions: callers can't forget to handle them.
export type Result<T> = { ok: true; value: T } | { ok: false; error: string };
export const ok = <T>(value: T): Result<T> => ({ ok: true, value });
export const fail = <T = never>(error: string): Result<T> => ({ ok: false, error });

export const RULES = {
  openMinute: 8 * 60, // 08:00 office time
  closeMinute: 20 * 60, // 20:00
  stepMinutes: 15,
  maxMinutes: 4 * 60,
  holdMinutes: 10,
  officeUtcOffsetMinutes: 9 * 60, // the office is in Japan (UTC+9, no daylight saving)
} as const;

const MINUTE = 60_000;

/** Minutes since midnight, and the calendar day, in the office's time zone. */
export function officeTime(d: Date): { day: string; minute: number } {
  const local = new Date(d.getTime() + RULES.officeUtcOffsetMinutes * MINUTE);
  return { day: local.toISOString().slice(0, 10), minute: local.getUTCHours() * 60 + local.getUTCMinutes() };
}

export function formatSlot(s: Slot): string {
  const start = officeTime(s.start);
  const end = officeTime(s.end);
  const hm = (m: number) => `${String(Math.floor(m / 60)).padStart(2, "0")}:${String(m % 60).padStart(2, "0")}`;
  return `${start.day} ${hm(start.minute)}-${hm(end.minute)}`;
}

/** Every problem with a request, in a fixed order. An empty list means it's valid. */
export function validateRequest(r: BookingRequest): string[] {
  const problems: string[] = [];
  if (!r.title.trim()) problems.push("title is required");
  if (!r.organizer.trim()) problems.push("organizer is required");
  if (!Number.isInteger(r.attendees) || r.attendees < 1) problems.push("attendees must be at least 1");
  const minutes = (r.end.getTime() - r.start.getTime()) / MINUTE;
  if (!(minutes > 0)) return [...problems, "end must be after start"];
  const start = officeTime(r.start);
  const end = officeTime(r.end);
  if (start.minute % RULES.stepMinutes || end.minute % RULES.stepMinutes || r.start.getUTCSeconds() || r.end.getUTCSeconds()) {
    problems.push("times must be on 15-minute steps");
  }
  if (minutes > RULES.maxMinutes) problems.push("bookings are at most 4 hours");
  const endMinute = end.day === start.day ? end.minute : 24 * 60;
  if (start.minute < RULES.openMinute || endMinute > RULES.closeMinute) problems.push("the office is open 08:00-20:00");
  return problems;
}

/**
 * Do two time slots overlap? End times are exclusive: a meeting 10:00-11:00 and one
 * 11:00-12:00 do NOT overlap (back-to-back meetings are fine).
 */
export function overlaps(a: Slot, b: Slot): boolean {
  throw new Error("TODO: not done yet");
}

/** A booking blocks its room if it's confirmed, or held and the hold hasn't expired yet. */
export function isActive(b: Booking, now: Date): boolean {
  return b.status === "confirmed" || (b.status === "held" && b.expiresAt.getTime() > now.getTime());
}

export function fits(room: Room, r: Pick<BookingRequest, "attendees" | "needs">): boolean {
  return room.capacity >= r.attendees && r.needs.every((n) => room.equipment.includes(n));
}

/**
 * The rooms that could take this request: big enough, with all the needed equipment, and
 * without an active booking that overlaps it. Smallest capacity first (don't waste the big
 * room on two people), then by name.
 */
export function findFreeRooms(rooms: Room[], bookings: Booking[], r: BookingRequest, now: Date): Room[] {
  throw new Error("TODO: not done yet");
}
@@file service.ts
// service.ts: the use cases. The service talks to the outside world only through "ports":
// small interfaces for storage, time and notifications. Adapters (memory.ts, a database, an
// email sender) plug into them. This is the "ports and adapters" (hexagonal) architecture.
import { fail, findFreeRooms, fits, formatSlot, isActive, ok, overlaps, RULES, validateRequest } from "./domain.ts";
import type { Booking, BookingRequest, Result, Room } from "./domain.ts";

export interface BookingRepo {
  nextId(): number;
  get(id: number): Booking | undefined;
  save(booking: Booking): void;
  all(): Booking[];
}

export type Clock = () => Date;

export interface Notifier {
  send(to: string, message: string): void;
}

const MINUTE = 60_000;

export class BookingService {
  private readonly rooms: Room[];
  private readonly repo: BookingRepo;
  private readonly clock: Clock;
  private readonly notifier: Notifier;

  constructor(rooms: Room[], repo: BookingRepo, clock: Clock, notifier: Notifier) {
    this.rooms = rooms;
    this.repo = repo;
    this.clock = clock;
    this.notifier = notifier;
  }

  /**
   * Hold a room for 10 minutes (RULES.holdMinutes) while the organizer confirms.
   * - invalid request: fail with the problems joined by "; "
   * - with roomId: fail "unknown room", "room doesn't fit the request" or "room is already booked"
   * - without roomId: take the first of findFreeRooms, or fail "no room is free"
   * The new booking gets id repo.nextId(), status "held" and expiresAt = now + 10 minutes.
   */
  hold(req: BookingRequest): Result<Booking> {
    throw new Error("TODO: not done yet");
  }

  confirm(id: number, by: string): Result<Booking> {
    const b = this.repo.get(id);
    if (!b) return fail(`no booking #${id}`);
    if (b.organizer !== by) return fail("only the organizer can do that");
    if (b.status !== "held") return fail(`booking is ${b.status}`);
    const now = this.clock();
    if (!isActive(b, now)) return fail("the hold has expired");
    const confirmed: Booking = { ...b, status: "confirmed", confirmedAt: now };
    this.repo.save(confirmed);
    this.notifier.send(b.organizer, `Confirmed: ${b.title}, ${this.roomName(b.roomId)}, ${formatSlot(b)}`);
    return ok(confirmed);
  }

  cancel(id: number, by: string, reason: string): Result<Booking> {
    const b = this.repo.get(id);
    if (!b) return fail(`no booking #${id}`);
    if (b.organizer !== by) return fail("only the organizer can do that");
    if (b.status === "cancelled") return fail("booking is cancelled");
    const cancelled: Booking = { ...b, status: "cancelled", reason: reason.trim() || "no reason given" };
    this.repo.save(cancelled);
    if (b.status === "confirmed") this.notifier.send(b.organizer, `Cancelled: ${b.title}, ${formatSlot(b)}`);
    return ok(cancelled);
  }

  /**
   * Cancel every hold whose expiresAt is now or earlier, with reason "hold expired", and send
   * the organizer "Your hold expired: <title>". Return how many were cancelled. A scheduler runs
   * this every minute; that's how un-confirmed holds give their rooms back.
   */
  expireHolds(): number {
    throw new Error("TODO: not done yet");
  }

  /** A room's confirmed meetings on one office day ("2026-11-04"), earliest first. */
  agenda(roomId: string, day: string): Booking[] {
    return this.repo
      .all()
      .filter((b) => b.roomId === roomId && b.status === "confirmed" && formatSlot(b).startsWith(day))
      .sort((a, b) => a.start.getTime() - b.start.getTime());
  }

  roomName(id: string): string {
    return this.rooms.find((r) => r.id === id)?.name ?? id;
  }
}
@@file adapters.ts
// adapters.ts: implementations of the ports. Swap them for a database, Slack or email later;
// the service doesn't change.
import type { Booking } from "./domain.ts";
import type { BookingRepo, Notifier } from "./service.ts";

export class MemoryRepo implements BookingRepo {
  private readonly bookings = new Map<number, Booking>();
  private last = 0;

  nextId(): number {
    return ++this.last;
  }

  get(id: number): Booking | undefined {
    return this.bookings.get(id);
  }

  save(booking: Booking): void {
    this.bookings.set(booking.id, booking);
  }

  all(): Booking[] {
    return [...this.bookings.values()];
  }
}

export class ConsoleNotifier implements Notifier {
  send(to: string, message: string): void {
    console.log(`  ✉ to ${to}: ${message}`);
  }
}

/** Records messages instead of sending them: for tests. */
export class FakeNotifier implements Notifier {
  readonly sent: { to: string; message: string }[] = [];

  send(to: string, message: string): void {
    this.sent.push({ to, message });
  }
}
@@file main.ts
// main.ts: a morning at the office, played through the service.   node main.ts
import { formatSlot } from "./domain.ts";
import type { BookingRequest, Room } from "./domain.ts";
import { ConsoleNotifier, MemoryRepo } from "./adapters.ts";
import { BookingService } from "./service.ts";

const rooms: Room[] = [
  { id: "fuji", name: "Fuji (12)", capacity: 12, equipment: ["projector", "video", "whiteboard"] },
  { id: "sakura", name: "Sakura (6)", capacity: 6, equipment: ["video", "whiteboard"] },
  { id: "momiji", name: "Momiji (4)", capacity: 4, equipment: ["whiteboard"] },
];

let now = new Date("2026-11-04T08:30:00+09:00");
const service = new BookingService(rooms, new MemoryRepo(), () => now, new ConsoleNotifier());

const at = (time: string) => new Date(`2026-11-04T${time}:00+09:00`);
const request = (r: Partial<BookingRequest> & Pick<BookingRequest, "start" | "end">): BookingRequest => ({
  organizer: "yuki",
  title: "Meeting",
  attendees: 3,
  needs: [],
  ...r,
});

function show(label: string, result: ReturnType<BookingService["hold"]>) {
  if (result.ok) {
    const b = result.value;
    console.log(`${label}: #${b.id} ${b.status} ${service.roomName(b.roomId)} ${formatSlot(b)}`);
  } else {
    console.log(`${label}: ✖ ${result.error}`);
  }
}

const standup = service.hold(request({ title: "Infra stand-up", start: at("09:00"), end: at("09:15") }));
show("hold stand-up", standup);
if (standup.ok) show("confirm stand-up", service.confirm(standup.value.id, "yuki"));

const review = service.hold(request({ organizer: "ken", title: "Firewall change review", attendees: 8, needs: ["projector"], start: at("10:00"), end: at("11:30") }));
show("hold review", review);
if (review.ok) show("confirm review", service.confirm(review.value.id, "ken"));

show("clash", service.hold(request({ roomId: "fuji", title: "Vendor call", start: at("11:00"), end: at("12:00") })));
show("bad request", service.hold(request({ title: " ", start: at("07:50"), end: at("13:00") })));

const forgotten = service.hold(request({ organizer: "aiko", title: "1:1", attendees: 2, start: at("14:00"), end: at("14:30") }));
show("hold 1:1", forgotten);
now = new Date(now.getTime() + 11 * 60_000);
console.log(`\n${now.toISOString()} expiring holds...`);
console.log(`expired: ${service.expireHolds()}`);

console.log("\nFuji today:");
for (const b of service.agenda("fuji", "2026-11-04")) console.log(`  ${formatSlot(b)} ${b.title} (${b.organizer})`);
@@file booking.test.ts
// Run: node --test
import { test } from "node:test";
import assert from "node:assert/strict";
import { findFreeRooms, officeTime, overlaps, validateRequest } from "./domain.ts";
import type { Booking, BookingRequest, Room } from "./domain.ts";
import { FakeNotifier, MemoryRepo } from "./adapters.ts";
import { BookingService } from "./service.ts";

const at = (time: string, day = "2026-11-04") => new Date(`${day}T${time}:00+09:00`);
const ROOMS: Room[] = [
  { id: "fuji", name: "Fuji", capacity: 12, equipment: ["projector", "video", "whiteboard"] },
  { id: "sakura", name: "Sakura", capacity: 6, equipment: ["video", "whiteboard"] },
  { id: "kaede", name: "Kaede", capacity: 6, equipment: ["whiteboard"] },
  { id: "momiji", name: "Momiji", capacity: 4, equipment: ["whiteboard"] },
];
const req = (start: string, end: string, extra: Partial<BookingRequest> = {}): BookingRequest => ({
  organizer: "yuki", title: "Sync", attendees: 3, needs: [], start: at(start), end: at(end), ...extra,
});
const booking = (roomId: string, start: string, end: string, status: "held" | "confirmed" | "cancelled", expires = "23:00"): Booking => {
  const base = { id: 1, roomId, organizer: "ken", title: "x", attendees: 2, start: at(start), end: at(end) };
  if (status === "held") return { ...base, status, expiresAt: at(expires) };
  if (status === "confirmed") return { ...base, status, confirmedAt: at("08:00") };
  return { ...base, status, reason: "test" };
};

test("officeTime uses Japan time", () => {
  assert.deepEqual(officeTime(new Date("2026-11-04T00:30:00Z")), { day: "2026-11-04", minute: 9 * 60 + 30 });
  assert.deepEqual(officeTime(new Date("2026-11-03T23:00:00Z")), { day: "2026-11-04", minute: 8 * 60 });
});

test("validateRequest lists every problem", () => {
  assert.deepEqual(validateRequest(req("09:00", "10:00")), []);
  assert.deepEqual(validateRequest(req("10:00", "09:00")), ["end must be after start"]);
  assert.deepEqual(validateRequest(req("09:10", "10:00")), ["times must be on 15-minute steps"]);
  assert.deepEqual(validateRequest(req("09:00", "13:15")), ["bookings are at most 4 hours"]);
  assert.deepEqual(validateRequest(req("07:45", "08:30")), ["the office is open 08:00-20:00"]);
  assert.deepEqual(validateRequest(req("19:00", "20:00")), []);
  assert.deepEqual(validateRequest(req("09:00", "09:30", { title: " ", attendees: 0 })), ["title is required", "attendees must be at least 1"]);
});

test("overlaps treats end times as exclusive", () => {
  const slot = (a: string, b: string) => ({ start: at(a), end: at(b) });
  assert.equal(overlaps(slot("10:00", "11:00"), slot("10:30", "11:30")), true);
  assert.equal(overlaps(slot("10:00", "11:00"), slot("09:00", "12:00")), true);
  assert.equal(overlaps(slot("10:00", "11:00"), slot("11:00", "12:00")), false);
  assert.equal(overlaps(slot("11:00", "12:00"), slot("10:00", "11:00")), false);
  assert.equal(overlaps(slot("10:00", "11:00"), slot("10:00", "11:00")), true);
});

test("findFreeRooms: fits, is free, smallest first", () => {
  const now = at("08:30");
  const names = (r: BookingRequest, bookings: Booking[] = []) => findFreeRooms(ROOMS, bookings, r, now).map((x) => x.name);
  assert.deepEqual(names(req("09:00", "10:00")), ["Momiji", "Kaede", "Sakura", "Fuji"]);
  assert.deepEqual(names(req("09:00", "10:00", { attendees: 5 })), ["Kaede", "Sakura", "Fuji"]);
  assert.deepEqual(names(req("09:00", "10:00", { needs: ["video"] })), ["Sakura", "Fuji"]);
  assert.deepEqual(names(req("09:00", "10:00", { needs: ["projector"], attendees: 20 })), []);
  const bookings = [
    booking("momiji", "09:30", "10:30", "confirmed"),
    booking("kaede", "09:00", "09:30", "held"),
    booking("sakura", "09:00", "10:00", "held", "08:20"), // expired hold: doesn't block
    booking("fuji", "09:00", "10:00", "cancelled"), // cancelled: doesn't block
  ];
  assert.deepEqual(names(req("09:00", "10:00"), bookings), ["Sakura", "Fuji"]);
});

function setup() {
  const clock = { now: at("08:30") };
  const notifier = new FakeNotifier();
  const service = new BookingService(ROOMS, new MemoryRepo(), () => clock.now, notifier);
  const minutes = (n: number) => (clock.now = new Date(clock.now.getTime() + n * 60_000));
  return { service, notifier, minutes };
}

test("hold picks the smallest free room, or the one asked for", () => {
  const { service } = setup();
  const a = service.hold(req("09:00", "10:00"));
  assert.ok(a.ok);
  assert.equal(a.value.roomId, "momiji");
  assert.equal(a.value.status, "held");
  assert.deepEqual(a.value.status === "held" && a.value.expiresAt, at("08:40"));
  const b = service.hold(req("09:00", "10:00"));
  assert.ok(b.ok && b.value.roomId === "kaede" && b.value.id === 2);
  assert.deepEqual(service.hold(req("09:30", "10:00", { roomId: "momiji" })), { ok: false, error: "room is already booked" });
  assert.deepEqual(service.hold(req("09:00", "10:00", { roomId: "nope" })), { ok: false, error: "unknown room" });
  assert.deepEqual(service.hold(req("09:00", "10:00", { roomId: "momiji", attendees: 9 })), { ok: false, error: "room doesn't fit the request" });
  assert.deepEqual(service.hold(req("09:05", "09:00")), { ok: false, error: "end must be after start" });
  assert.deepEqual(service.hold(req("09:00", "10:00", { attendees: 50 })), { ok: false, error: "no room is free" });
});

test("confirm, cancel and who may do it", () => {
  const { service, notifier } = setup();
  const held = service.hold(req("09:00", "10:00", { title: "Infra sync" }));
  assert.ok(held.ok);
  const id = held.value.id;
  assert.deepEqual(service.confirm(id, "ken"), { ok: false, error: "only the organizer can do that" });
  const confirmed = service.confirm(id, "yuki");
  assert.ok(confirmed.ok && confirmed.value.status === "confirmed");
  assert.deepEqual(notifier.sent, [{ to: "yuki", message: "Confirmed: Infra sync, Momiji, 2026-11-04 09:00-10:00" }]);
  assert.deepEqual(service.confirm(id, "yuki"), { ok: false, error: "booking is confirmed" });
  const cancelled = service.cancel(id, "yuki", "  ");
  assert.ok(cancelled.ok && cancelled.value.status === "cancelled" && cancelled.value.reason === "no reason given");
  assert.deepEqual(service.cancel(id, "yuki", "again"), { ok: false, error: "booking is cancelled" });
  assert.deepEqual(service.confirm(99, "yuki"), { ok: false, error: "no booking #99" });
});

test("holds expire and give the room back", () => {
  const { service, notifier, minutes } = setup();
  const first = service.hold(req("09:00", "10:00", { roomId: "momiji", title: "Forgotten" }));
  assert.ok(first.ok);
  minutes(9);
  assert.equal(service.expireHolds(), 0);
  assert.deepEqual(service.hold(req("09:00", "10:00", { roomId: "momiji" })), { ok: false, error: "room is already booked" });
  minutes(1);
  assert.deepEqual(service.confirm(first.value.id, "yuki"), { ok: false, error: "the hold has expired" });
  assert.equal(service.expireHolds(), 1);
  assert.deepEqual(notifier.sent, [{ to: "yuki", message: "Your hold expired: Forgotten" }]);
  assert.equal(service.expireHolds(), 0);
  const again = service.hold(req("09:00", "10:00", { roomId: "momiji" }));
  assert.ok(again.ok);
});

test("agenda shows a room's confirmed meetings for one day", () => {
  const { service } = setup();
  for (const [start, end, title] of [["14:00", "15:00", "B"], ["09:00", "09:30", "A"]]) {
    const h = service.hold(req(start, end, { roomId: "fuji", title }));
    assert.ok(h.ok);
    service.confirm(h.value.id, "yuki");
  }
  service.hold(req("16:00", "17:00", { roomId: "fuji", title: "only held" }));
  assert.deepEqual(service.agenda("fuji", "2026-11-04").map((b) => b.title), ["A", "B"]);
  assert.deepEqual(service.agenda("fuji", "2026-11-05"), []);
});
@@file package.json
{
  "name": "room-booking",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "start": "node main.ts",
    "test": "node --test",
    "check": "tsc"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "typescript": "^5.8.0"
  }
}
@@file tsconfig.json
{
  "compilerOptions": {
    "target": "es2023",
    "module": "nodenext",
    "strict": true,
    "noUncheckedIndexedAccess": false,
    "noEmit": true,
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "erasableSyntaxOnly": true,
    "skipLibCheck": true,
    "types": ["node"]
  },
  "include": ["*.ts"]
}
@@text
## Stretch goals

- **A web API**: `POST /holds`, `POST /bookings/:id/confirm`, `GET /rooms/:id/agenda?day=…` with `node:http` (like the Link Vault project in the JavaScript track). Validate the JSON body at the edge and turn it into a `BookingRequest`.
- **Recurring meetings**: "every Monday 09:00 for 8 weeks", all-or-nothing: if one week clashes, hold nothing and say which week.
- **A real database**: a `SqliteRepo` with `node:sqlite` implementing `BookingRepo`. The tests for the service should still pass with `MemoryRepo`. Add a test suite that runs the same checks against both repos ("contract tests").
- **Events instead of direct notifications**: the service publishes `BookingConfirmed` / `HoldExpired` events; separate subscribers send email, update a dashboard and write an audit log.
- **No double booking under load**: two people hold the same room at the same moment. In memory that can't happen (JavaScript is single-threaded), but with a database and several servers it can. How would you prevent it? (Hint: a unique constraint or an exclusion constraint, and a transaction.)
@@project room_booking_finished Meeting Room Booking: the finished version
A complete solution to compare with yours.
@@file domain.ts
// domain.ts: the types and pure rules of room booking. No I/O, no clock, no classes needed.

export type Equipment = "projector" | "video" | "whiteboard";

export type Room = { id: string; name: string; capacity: number; equipment: Equipment[] };

export type Slot = { start: Date; end: Date };

export type BookingRequest = Slot & {
  roomId?: string; // leave it out to get the smallest free room that fits
  organizer: string;
  title: string;
  attendees: number;
  needs: Equipment[];
};

// A booking is in exactly one state, and each state has its own fields. TypeScript makes it
// impossible to read `reason` from a confirmed booking, or to forget `expiresAt` on a hold.
type BookingBase = Slot & { id: number; roomId: string; organizer: string; title: string; attendees: number };
export type Booking =
  | (BookingBase & { status: "held"; expiresAt: Date })
  | (BookingBase & { status: "confirmed"; confirmedAt: Date })
  | (BookingBase & { status: "cancelled"; reason: string });

// Expected failures are values, not exceptions: callers can't forget to handle them.
export type Result<T> = { ok: true; value: T } | { ok: false; error: string };
export const ok = <T>(value: T): Result<T> => ({ ok: true, value });
export const fail = <T = never>(error: string): Result<T> => ({ ok: false, error });

export const RULES = {
  openMinute: 8 * 60, // 08:00 office time
  closeMinute: 20 * 60, // 20:00
  stepMinutes: 15,
  maxMinutes: 4 * 60,
  holdMinutes: 10,
  officeUtcOffsetMinutes: 9 * 60, // the office is in Japan (UTC+9, no daylight saving)
} as const;

const MINUTE = 60_000;

/** Minutes since midnight, and the calendar day, in the office's time zone. */
export function officeTime(d: Date): { day: string; minute: number } {
  const local = new Date(d.getTime() + RULES.officeUtcOffsetMinutes * MINUTE);
  return { day: local.toISOString().slice(0, 10), minute: local.getUTCHours() * 60 + local.getUTCMinutes() };
}

export function formatSlot(s: Slot): string {
  const start = officeTime(s.start);
  const end = officeTime(s.end);
  const hm = (m: number) => `${String(Math.floor(m / 60)).padStart(2, "0")}:${String(m % 60).padStart(2, "0")}`;
  return `${start.day} ${hm(start.minute)}-${hm(end.minute)}`;
}

/** Every problem with a request, in a fixed order. An empty list means it's valid. */
export function validateRequest(r: BookingRequest): string[] {
  const problems: string[] = [];
  if (!r.title.trim()) problems.push("title is required");
  if (!r.organizer.trim()) problems.push("organizer is required");
  if (!Number.isInteger(r.attendees) || r.attendees < 1) problems.push("attendees must be at least 1");
  const minutes = (r.end.getTime() - r.start.getTime()) / MINUTE;
  if (!(minutes > 0)) return [...problems, "end must be after start"];
  const start = officeTime(r.start);
  const end = officeTime(r.end);
  if (start.minute % RULES.stepMinutes || end.minute % RULES.stepMinutes || r.start.getUTCSeconds() || r.end.getUTCSeconds()) {
    problems.push("times must be on 15-minute steps");
  }
  if (minutes > RULES.maxMinutes) problems.push("bookings are at most 4 hours");
  const endMinute = end.day === start.day ? end.minute : 24 * 60;
  if (start.minute < RULES.openMinute || endMinute > RULES.closeMinute) problems.push("the office is open 08:00-20:00");
  return problems;
}

/**
 * Do two time slots overlap? End times are exclusive: a meeting 10:00-11:00 and one
 * 11:00-12:00 do NOT overlap (back-to-back meetings are fine).
 */
export function overlaps(a: Slot, b: Slot): boolean {
  return a.start.getTime() < b.end.getTime() && b.start.getTime() < a.end.getTime();
}

/** A booking blocks its room if it's confirmed, or held and the hold hasn't expired yet. */
export function isActive(b: Booking, now: Date): boolean {
  return b.status === "confirmed" || (b.status === "held" && b.expiresAt.getTime() > now.getTime());
}

export function fits(room: Room, r: Pick<BookingRequest, "attendees" | "needs">): boolean {
  return room.capacity >= r.attendees && r.needs.every((n) => room.equipment.includes(n));
}

/**
 * The rooms that could take this request: big enough, with all the needed equipment, and
 * without an active booking that overlaps it. Smallest capacity first (don't waste the big
 * room on two people), then by name.
 */
export function findFreeRooms(rooms: Room[], bookings: Booking[], r: BookingRequest, now: Date): Room[] {
  return rooms
    .filter((room) => fits(room, r))
    .filter((room) => !bookings.some((b) => b.roomId === room.id && isActive(b, now) && overlaps(b, r)))
    .sort((a, b) => a.capacity - b.capacity || a.name.localeCompare(b.name));
}
@@file service.ts
// service.ts: the use cases. The service talks to the outside world only through "ports":
// small interfaces for storage, time and notifications. Adapters (memory.ts, a database, an
// email sender) plug into them. This is the "ports and adapters" (hexagonal) architecture.
import { fail, findFreeRooms, fits, formatSlot, isActive, ok, overlaps, RULES, validateRequest } from "./domain.ts";
import type { Booking, BookingRequest, Result, Room } from "./domain.ts";

export interface BookingRepo {
  nextId(): number;
  get(id: number): Booking | undefined;
  save(booking: Booking): void;
  all(): Booking[];
}

export type Clock = () => Date;

export interface Notifier {
  send(to: string, message: string): void;
}

const MINUTE = 60_000;

export class BookingService {
  private readonly rooms: Room[];
  private readonly repo: BookingRepo;
  private readonly clock: Clock;
  private readonly notifier: Notifier;

  constructor(rooms: Room[], repo: BookingRepo, clock: Clock, notifier: Notifier) {
    this.rooms = rooms;
    this.repo = repo;
    this.clock = clock;
    this.notifier = notifier;
  }

  /**
   * Hold a room for 10 minutes (RULES.holdMinutes) while the organizer confirms.
   * - invalid request: fail with the problems joined by "; "
   * - with roomId: fail "unknown room", "room doesn't fit the request" or "room is already booked"
   * - without roomId: take the first of findFreeRooms, or fail "no room is free"
   * The new booking gets id repo.nextId(), status "held" and expiresAt = now + 10 minutes.
   */
  hold(req: BookingRequest): Result<Booking> {
    const problems = validateRequest(req);
    if (problems.length) return fail(problems.join("; "));
    const now = this.clock();
    let room: Room | undefined;
    if (req.roomId) {
      room = this.rooms.find((r) => r.id === req.roomId);
      if (!room) return fail("unknown room");
      if (!fits(room, req)) return fail("room doesn't fit the request");
      const taken = this.repo.all().some((b) => b.roomId === req.roomId && isActive(b, now) && overlaps(b, req));
      if (taken) return fail("room is already booked");
    } else {
      room = findFreeRooms(this.rooms, this.repo.all(), req, now)[0];
      if (!room) return fail("no room is free");
    }
    const booking: Booking = {
      id: this.repo.nextId(),
      roomId: room.id,
      organizer: req.organizer.trim(),
      title: req.title.trim(),
      attendees: req.attendees,
      start: req.start,
      end: req.end,
      status: "held",
      expiresAt: new Date(now.getTime() + RULES.holdMinutes * MINUTE),
    };
    this.repo.save(booking);
    return ok(booking);
  }

  confirm(id: number, by: string): Result<Booking> {
    const b = this.repo.get(id);
    if (!b) return fail(`no booking #${id}`);
    if (b.organizer !== by) return fail("only the organizer can do that");
    if (b.status !== "held") return fail(`booking is ${b.status}`);
    const now = this.clock();
    if (!isActive(b, now)) return fail("the hold has expired");
    const confirmed: Booking = { ...b, status: "confirmed", confirmedAt: now };
    this.repo.save(confirmed);
    this.notifier.send(b.organizer, `Confirmed: ${b.title}, ${this.roomName(b.roomId)}, ${formatSlot(b)}`);
    return ok(confirmed);
  }

  cancel(id: number, by: string, reason: string): Result<Booking> {
    const b = this.repo.get(id);
    if (!b) return fail(`no booking #${id}`);
    if (b.organizer !== by) return fail("only the organizer can do that");
    if (b.status === "cancelled") return fail("booking is cancelled");
    const cancelled: Booking = { ...b, status: "cancelled", reason: reason.trim() || "no reason given" };
    this.repo.save(cancelled);
    if (b.status === "confirmed") this.notifier.send(b.organizer, `Cancelled: ${b.title}, ${formatSlot(b)}`);
    return ok(cancelled);
  }

  /**
   * Cancel every hold whose expiresAt is now or earlier, with reason "hold expired", and send
   * the organizer "Your hold expired: <title>". Return how many were cancelled. A scheduler runs
   * this every minute; that's how un-confirmed holds give their rooms back.
   */
  expireHolds(): number {
    const now = this.clock();
    let count = 0;
    for (const b of this.repo.all()) {
      if (b.status === "held" && b.expiresAt.getTime() <= now.getTime()) {
        this.repo.save({ ...b, status: "cancelled", reason: "hold expired" });
        this.notifier.send(b.organizer, `Your hold expired: ${b.title}`);
        count++;
      }
    }
    return count;
  }

  /** A room's confirmed meetings on one office day ("2026-11-04"), earliest first. */
  agenda(roomId: string, day: string): Booking[] {
    return this.repo
      .all()
      .filter((b) => b.roomId === roomId && b.status === "confirmed" && formatSlot(b).startsWith(day))
      .sort((a, b) => a.start.getTime() - b.start.getTime());
  }

  roomName(id: string): string {
    return this.rooms.find((r) => r.id === id)?.name ?? id;
  }
}
@@file adapters.ts
// adapters.ts: implementations of the ports. Swap them for a database, Slack or email later;
// the service doesn't change.
import type { Booking } from "./domain.ts";
import type { BookingRepo, Notifier } from "./service.ts";

export class MemoryRepo implements BookingRepo {
  private readonly bookings = new Map<number, Booking>();
  private last = 0;

  nextId(): number {
    return ++this.last;
  }

  get(id: number): Booking | undefined {
    return this.bookings.get(id);
  }

  save(booking: Booking): void {
    this.bookings.set(booking.id, booking);
  }

  all(): Booking[] {
    return [...this.bookings.values()];
  }
}

export class ConsoleNotifier implements Notifier {
  send(to: string, message: string): void {
    console.log(`  ✉ to ${to}: ${message}`);
  }
}

/** Records messages instead of sending them: for tests. */
export class FakeNotifier implements Notifier {
  readonly sent: { to: string; message: string }[] = [];

  send(to: string, message: string): void {
    this.sent.push({ to, message });
  }
}
@@file main.ts
// main.ts: a morning at the office, played through the service.   node main.ts
import { formatSlot } from "./domain.ts";
import type { BookingRequest, Room } from "./domain.ts";
import { ConsoleNotifier, MemoryRepo } from "./adapters.ts";
import { BookingService } from "./service.ts";

const rooms: Room[] = [
  { id: "fuji", name: "Fuji (12)", capacity: 12, equipment: ["projector", "video", "whiteboard"] },
  { id: "sakura", name: "Sakura (6)", capacity: 6, equipment: ["video", "whiteboard"] },
  { id: "momiji", name: "Momiji (4)", capacity: 4, equipment: ["whiteboard"] },
];

let now = new Date("2026-11-04T08:30:00+09:00");
const service = new BookingService(rooms, new MemoryRepo(), () => now, new ConsoleNotifier());

const at = (time: string) => new Date(`2026-11-04T${time}:00+09:00`);
const request = (r: Partial<BookingRequest> & Pick<BookingRequest, "start" | "end">): BookingRequest => ({
  organizer: "yuki",
  title: "Meeting",
  attendees: 3,
  needs: [],
  ...r,
});

function show(label: string, result: ReturnType<BookingService["hold"]>) {
  if (result.ok) {
    const b = result.value;
    console.log(`${label}: #${b.id} ${b.status} ${service.roomName(b.roomId)} ${formatSlot(b)}`);
  } else {
    console.log(`${label}: ✖ ${result.error}`);
  }
}

const standup = service.hold(request({ title: "Infra stand-up", start: at("09:00"), end: at("09:15") }));
show("hold stand-up", standup);
if (standup.ok) show("confirm stand-up", service.confirm(standup.value.id, "yuki"));

const review = service.hold(request({ organizer: "ken", title: "Firewall change review", attendees: 8, needs: ["projector"], start: at("10:00"), end: at("11:30") }));
show("hold review", review);
if (review.ok) show("confirm review", service.confirm(review.value.id, "ken"));

show("clash", service.hold(request({ roomId: "fuji", title: "Vendor call", start: at("11:00"), end: at("12:00") })));
show("bad request", service.hold(request({ title: " ", start: at("07:50"), end: at("13:00") })));

const forgotten = service.hold(request({ organizer: "aiko", title: "1:1", attendees: 2, start: at("14:00"), end: at("14:30") }));
show("hold 1:1", forgotten);
now = new Date(now.getTime() + 11 * 60_000);
console.log(`\n${now.toISOString()} expiring holds...`);
console.log(`expired: ${service.expireHolds()}`);

console.log("\nFuji today:");
for (const b of service.agenda("fuji", "2026-11-04")) console.log(`  ${formatSlot(b)} ${b.title} (${b.organizer})`);
@@file booking.test.ts
// Run: node --test
import { test } from "node:test";
import assert from "node:assert/strict";
import { findFreeRooms, officeTime, overlaps, validateRequest } from "./domain.ts";
import type { Booking, BookingRequest, Room } from "./domain.ts";
import { FakeNotifier, MemoryRepo } from "./adapters.ts";
import { BookingService } from "./service.ts";

const at = (time: string, day = "2026-11-04") => new Date(`${day}T${time}:00+09:00`);
const ROOMS: Room[] = [
  { id: "fuji", name: "Fuji", capacity: 12, equipment: ["projector", "video", "whiteboard"] },
  { id: "sakura", name: "Sakura", capacity: 6, equipment: ["video", "whiteboard"] },
  { id: "kaede", name: "Kaede", capacity: 6, equipment: ["whiteboard"] },
  { id: "momiji", name: "Momiji", capacity: 4, equipment: ["whiteboard"] },
];
const req = (start: string, end: string, extra: Partial<BookingRequest> = {}): BookingRequest => ({
  organizer: "yuki", title: "Sync", attendees: 3, needs: [], start: at(start), end: at(end), ...extra,
});
const booking = (roomId: string, start: string, end: string, status: "held" | "confirmed" | "cancelled", expires = "23:00"): Booking => {
  const base = { id: 1, roomId, organizer: "ken", title: "x", attendees: 2, start: at(start), end: at(end) };
  if (status === "held") return { ...base, status, expiresAt: at(expires) };
  if (status === "confirmed") return { ...base, status, confirmedAt: at("08:00") };
  return { ...base, status, reason: "test" };
};

test("officeTime uses Japan time", () => {
  assert.deepEqual(officeTime(new Date("2026-11-04T00:30:00Z")), { day: "2026-11-04", minute: 9 * 60 + 30 });
  assert.deepEqual(officeTime(new Date("2026-11-03T23:00:00Z")), { day: "2026-11-04", minute: 8 * 60 });
});

test("validateRequest lists every problem", () => {
  assert.deepEqual(validateRequest(req("09:00", "10:00")), []);
  assert.deepEqual(validateRequest(req("10:00", "09:00")), ["end must be after start"]);
  assert.deepEqual(validateRequest(req("09:10", "10:00")), ["times must be on 15-minute steps"]);
  assert.deepEqual(validateRequest(req("09:00", "13:15")), ["bookings are at most 4 hours"]);
  assert.deepEqual(validateRequest(req("07:45", "08:30")), ["the office is open 08:00-20:00"]);
  assert.deepEqual(validateRequest(req("19:00", "20:00")), []);
  assert.deepEqual(validateRequest(req("09:00", "09:30", { title: " ", attendees: 0 })), ["title is required", "attendees must be at least 1"]);
});

test("overlaps treats end times as exclusive", () => {
  const slot = (a: string, b: string) => ({ start: at(a), end: at(b) });
  assert.equal(overlaps(slot("10:00", "11:00"), slot("10:30", "11:30")), true);
  assert.equal(overlaps(slot("10:00", "11:00"), slot("09:00", "12:00")), true);
  assert.equal(overlaps(slot("10:00", "11:00"), slot("11:00", "12:00")), false);
  assert.equal(overlaps(slot("11:00", "12:00"), slot("10:00", "11:00")), false);
  assert.equal(overlaps(slot("10:00", "11:00"), slot("10:00", "11:00")), true);
});

test("findFreeRooms: fits, is free, smallest first", () => {
  const now = at("08:30");
  const names = (r: BookingRequest, bookings: Booking[] = []) => findFreeRooms(ROOMS, bookings, r, now).map((x) => x.name);
  assert.deepEqual(names(req("09:00", "10:00")), ["Momiji", "Kaede", "Sakura", "Fuji"]);
  assert.deepEqual(names(req("09:00", "10:00", { attendees: 5 })), ["Kaede", "Sakura", "Fuji"]);
  assert.deepEqual(names(req("09:00", "10:00", { needs: ["video"] })), ["Sakura", "Fuji"]);
  assert.deepEqual(names(req("09:00", "10:00", { needs: ["projector"], attendees: 20 })), []);
  const bookings = [
    booking("momiji", "09:30", "10:30", "confirmed"),
    booking("kaede", "09:00", "09:30", "held"),
    booking("sakura", "09:00", "10:00", "held", "08:20"), // expired hold: doesn't block
    booking("fuji", "09:00", "10:00", "cancelled"), // cancelled: doesn't block
  ];
  assert.deepEqual(names(req("09:00", "10:00"), bookings), ["Sakura", "Fuji"]);
});

function setup() {
  const clock = { now: at("08:30") };
  const notifier = new FakeNotifier();
  const service = new BookingService(ROOMS, new MemoryRepo(), () => clock.now, notifier);
  const minutes = (n: number) => (clock.now = new Date(clock.now.getTime() + n * 60_000));
  return { service, notifier, minutes };
}

test("hold picks the smallest free room, or the one asked for", () => {
  const { service } = setup();
  const a = service.hold(req("09:00", "10:00"));
  assert.ok(a.ok);
  assert.equal(a.value.roomId, "momiji");
  assert.equal(a.value.status, "held");
  assert.deepEqual(a.value.status === "held" && a.value.expiresAt, at("08:40"));
  const b = service.hold(req("09:00", "10:00"));
  assert.ok(b.ok && b.value.roomId === "kaede" && b.value.id === 2);
  assert.deepEqual(service.hold(req("09:30", "10:00", { roomId: "momiji" })), { ok: false, error: "room is already booked" });
  assert.deepEqual(service.hold(req("09:00", "10:00", { roomId: "nope" })), { ok: false, error: "unknown room" });
  assert.deepEqual(service.hold(req("09:00", "10:00", { roomId: "momiji", attendees: 9 })), { ok: false, error: "room doesn't fit the request" });
  assert.deepEqual(service.hold(req("09:05", "09:00")), { ok: false, error: "end must be after start" });
  assert.deepEqual(service.hold(req("09:00", "10:00", { attendees: 50 })), { ok: false, error: "no room is free" });
});

test("confirm, cancel and who may do it", () => {
  const { service, notifier } = setup();
  const held = service.hold(req("09:00", "10:00", { title: "Infra sync" }));
  assert.ok(held.ok);
  const id = held.value.id;
  assert.deepEqual(service.confirm(id, "ken"), { ok: false, error: "only the organizer can do that" });
  const confirmed = service.confirm(id, "yuki");
  assert.ok(confirmed.ok && confirmed.value.status === "confirmed");
  assert.deepEqual(notifier.sent, [{ to: "yuki", message: "Confirmed: Infra sync, Momiji, 2026-11-04 09:00-10:00" }]);
  assert.deepEqual(service.confirm(id, "yuki"), { ok: false, error: "booking is confirmed" });
  const cancelled = service.cancel(id, "yuki", "  ");
  assert.ok(cancelled.ok && cancelled.value.status === "cancelled" && cancelled.value.reason === "no reason given");
  assert.deepEqual(service.cancel(id, "yuki", "again"), { ok: false, error: "booking is cancelled" });
  assert.deepEqual(service.confirm(99, "yuki"), { ok: false, error: "no booking #99" });
});

test("holds expire and give the room back", () => {
  const { service, notifier, minutes } = setup();
  const first = service.hold(req("09:00", "10:00", { roomId: "momiji", title: "Forgotten" }));
  assert.ok(first.ok);
  minutes(9);
  assert.equal(service.expireHolds(), 0);
  assert.deepEqual(service.hold(req("09:00", "10:00", { roomId: "momiji" })), { ok: false, error: "room is already booked" });
  minutes(1);
  assert.deepEqual(service.confirm(first.value.id, "yuki"), { ok: false, error: "the hold has expired" });
  assert.equal(service.expireHolds(), 1);
  assert.deepEqual(notifier.sent, [{ to: "yuki", message: "Your hold expired: Forgotten" }]);
  assert.equal(service.expireHolds(), 0);
  const again = service.hold(req("09:00", "10:00", { roomId: "momiji" }));
  assert.ok(again.ok);
});

test("agenda shows a room's confirmed meetings for one day", () => {
  const { service } = setup();
  for (const [start, end, title] of [["14:00", "15:00", "B"], ["09:00", "09:30", "A"]]) {
    const h = service.hold(req(start, end, { roomId: "fuji", title }));
    assert.ok(h.ok);
    service.confirm(h.value.id, "yuki");
  }
  service.hold(req("16:00", "17:00", { roomId: "fuji", title: "only held" }));
  assert.deepEqual(service.agenda("fuji", "2026-11-04").map((b) => b.title), ["A", "B"]);
  assert.deepEqual(service.agenda("fuji", "2026-11-05"), []);
});
@@file package.json
{
  "name": "room-booking",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "start": "node main.ts",
    "test": "node --test",
    "check": "tsc"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "typescript": "^5.8.0"
  }
}
@@file tsconfig.json
{
  "compilerOptions": {
    "target": "es2023",
    "module": "nodenext",
    "strict": true,
    "noUncheckedIndexedAccess": false,
    "noEmit": true,
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "erasableSyntaxOnly": true,
    "skipLibCheck": true,
    "types": ["node"]
  },
  "include": ["*.ts"]
}

@@drill
End times are exclusive. Do bookings 10:00-11:00 and 11:00-12:00 overlap? (yes or no)
= no
? Back-to-back meetings are fine: a.start < b.end && b.start < a.end is false.

@@text
🎉 Discriminated unions for state, `Result` values for expected failures, pure domain functions, and ports with swappable adapters: this is how large TypeScript codebases (and Java, C# and Go ones) stay changeable for years.
