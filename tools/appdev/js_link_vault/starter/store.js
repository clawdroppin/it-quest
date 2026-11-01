// store.js: keeps the data in one JSON file, written safely.
const fs = require("node:fs");

const EMPTY = () => ({ nextId: 1, links: [] });

// file = null keeps everything in memory (for tests).
function createStore(file = null) {
  let data = EMPTY();
  if (file && fs.existsSync(file)) data = { ...EMPTY(), ...JSON.parse(fs.readFileSync(file, "utf8")) };

  return {
    get data() {
      return data;
    },
    set data(value) {
      data = value;
    },
    save() {
      if (!file) return;
      // Write a temporary file, then rename it over the real one. A rename is atomic, so a crash
      // in the middle leaves either the old file or the new one, never half a file.
      const tmp = `${file}.tmp`;
      fs.writeFileSync(tmp, JSON.stringify(data, null, 2));
      fs.renameSync(tmp, file);
    },
  };
}

module.exports = { createStore };
