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
