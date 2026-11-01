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
