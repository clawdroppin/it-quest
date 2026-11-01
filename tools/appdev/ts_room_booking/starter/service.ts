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
