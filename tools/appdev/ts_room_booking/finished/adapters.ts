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
