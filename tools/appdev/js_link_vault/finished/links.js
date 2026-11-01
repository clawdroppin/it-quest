// links.js: the business rules for saved links. No HTTP, no files: it gets a store and a clock.

class AppError extends Error {
  constructor(status, message) {
    super(message);
    this.status = status;
  }
}

// Tracking parameters that make the same page look like a different URL.
const TRACKING = /^(utm_\w+|fbclid|gclid|mc_eid)$/i;

/**
 * The canonical form of a URL, so duplicates can be found:
 * - add "https://" when there's no scheme; only http and https are allowed (AppError 400 "url must be http or https"),
 * - lower-case the host, drop "www.", drop the #fragment and tracking parameters (TRACKING),
 * - remove a trailing "/" from the path (but keep "https://example.com/" as is).
 * Anything new URL() can't parse is AppError 400 "url is not valid".
 */
function normalizeUrl(input) {
  let text = String(input ?? "").trim();
  if (!/^[a-z][a-z0-9+.-]*:/i.test(text)) text = "https://" + text;
  let url;
  try {
    url = new URL(text);
  } catch {
    throw new AppError(400, "url is not valid");
  }
  if (url.protocol !== "http:" && url.protocol !== "https:") throw new AppError(400, "url must be http or https");
  if (!url.hostname.includes(".") && url.hostname !== "localhost") throw new AppError(400, "url is not valid");
  url.hostname = url.hostname.toLowerCase().replace(/^www\./, "");
  url.hash = "";
  for (const key of [...url.searchParams.keys()]) {
    if (TRACKING.test(key)) url.searchParams.delete(key);
  }
  if (url.pathname.length > 1 && url.pathname.endsWith("/")) url.pathname = url.pathname.replace(/\/+$/, "");
  return url.toString();
}

function cleanTags(tags) {
  if (tags === undefined) return [];
  if (!Array.isArray(tags)) throw new AppError(400, "tags must be a list");
  return [...new Set(tags.map((t) => String(t).trim().toLowerCase()).filter(Boolean))].sort();
}

function createLinkService(store, clock = () => new Date()) {
  function find(id) {
    const link = store.data.links.find((l) => l.id === Number(id));
    if (!link) throw new AppError(404, `no link #${id}`);
    return link;
  }

  return {
    add({ url, title, tags } = {}) {
      const normalized = normalizeUrl(url);
      if (store.data.links.some((l) => l.url === normalized)) throw new AppError(409, "already saved");
      const cleanTitle = String(title ?? "").trim() || new URL(normalized).hostname;
      if (cleanTitle.length > 200) throw new AppError(400, "title is too long");
      const link = {
        id: store.data.nextId++,
        url: normalized,
        title: cleanTitle,
        tags: cleanTags(tags),
        created: clock().toISOString(),
        visits: 0,
      };
      store.data.links.push(link);
      store.save();
      return link;
    },

    get: find,

    /**
     * Search the saved links.
     * - q: every word must appear in the title or the url (case-insensitive),
     * - tag: only links with this tag,
     * - sort: "new" (newest first, the default), "popular" (most visits first, then newest)
     *   or "title" (A to Z, case-insensitive). Anything else is AppError 400 "unknown sort".
     */
    list({ q, tag, sort = "new" } = {}) {
      const sorters = {
        new: (a, b) => b.id - a.id,
        popular: (a, b) => b.visits - a.visits || b.id - a.id,
        title: (a, b) => a.title.localeCompare(b.title, "en", { sensitivity: "base" }),
      };
      if (!sorters[sort]) throw new AppError(400, "unknown sort");
      const words = String(q ?? "").toLowerCase().split(/\s+/).filter(Boolean);
      const wanted = tag ? String(tag).toLowerCase() : null;
      return store.data.links
        .filter((l) => !wanted || l.tags.includes(wanted))
        .filter((l) => words.every((w) => `${l.title} ${l.url}`.toLowerCase().includes(w)))
        .sort(sorters[sort]);
    },

    update(id, { title, tags } = {}) {
      const link = find(id);
      if (title !== undefined) {
        const t = String(title).trim();
        if (!t) throw new AppError(400, "title is required");
        link.title = t;
      }
      if (tags !== undefined) link.tags = cleanTags(tags);
      store.save();
      return link;
    },

    remove(id) {
      const link = find(id);
      store.data.links = store.data.links.filter((l) => l !== link);
      store.save();
    },

    visit(id) {
      const link = find(id);
      link.visits++;
      store.save();
      return link;
    },

    tags() {
      const counts = {};
      for (const l of store.data.links) for (const t of l.tags) counts[t] = (counts[t] ?? 0) + 1;
      return Object.entries(counts)
        .map(([name, count]) => ({ name, count }))
        .sort((a, b) => b.count - a.count || a.name.localeCompare(b.name));
    },
  };
}

module.exports = { AppError, normalizeUrl, cleanTags, createLinkService };
