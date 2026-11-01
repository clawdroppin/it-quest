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
