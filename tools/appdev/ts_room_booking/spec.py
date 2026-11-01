OUT = "content/typescript/u2_01_room_booking.md"
BASE = "tools/appdev/ts_room_booking"
FILES = ["domain.ts", "service.ts", "adapters.ts", "main.ts", "booking.test.ts", "package.json", "tsconfig.json"]

PARTS = [
    ("text", r'''
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
'''),
    ("project", "room_booking", "Meeting Room Booking",
     "Write the four functions that throw `TODO` until `node --test` passes, then run `node main.ts`.",
     BASE + "/starter", FILES),
    ("text", r'''
@@text
## Stretch goals

- **A web API**: `POST /holds`, `POST /bookings/:id/confirm`, `GET /rooms/:id/agenda?day=…` with `node:http` (like the Link Vault project in the JavaScript track). Validate the JSON body at the edge and turn it into a `BookingRequest`.
- **Recurring meetings**: "every Monday 09:00 for 8 weeks", all-or-nothing: if one week clashes, hold nothing and say which week.
- **A real database**: a `SqliteRepo` with `node:sqlite` implementing `BookingRepo`. The tests for the service should still pass with `MemoryRepo`. Add a test suite that runs the same checks against both repos ("contract tests").
- **Events instead of direct notifications**: the service publishes `BookingConfirmed` / `HoldExpired` events; separate subscribers send email, update a dashboard and write an audit log.
- **No double booking under load**: two people hold the same room at the same moment. In memory that can't happen (JavaScript is single-threaded), but with a database and several servers it can. How would you prevent it? (Hint: a unique constraint or an exclusion constraint, and a transaction.)
'''),
    ("project", "room_booking_finished", "Meeting Room Booking: the finished version",
     "A complete solution to compare with yours.",
     BASE + "/finished", FILES),
    ("text", r'''
@@text
🎉 Discriminated unions for state, `Result` values for expected failures, pure domain functions, and ports with swappable adapters: this is how large TypeScript codebases (and Java, C# and Go ones) stay changeable for years.
'''),
]
