import { getCurrentWindow } from "@tauri-apps/api/window";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { api } from "./api";
import { setCourse } from "./course";
import { loadProgress, saveNow } from "./progress";
import "./styles.css";

async function start() {
  const root = createRoot(document.getElementById("root")!);
  try {
    const [data] = await Promise.all([api.course(), loadProgress()]);
    setCourse(data);
    root.render(<App />);
  } catch (e) {
    root.render(<pre style={{ padding: 20, color: "#f88" }}>IT Quest couldn't start: {String(e)}</pre>);
  }
  // Save before the window closes: the debounced save may still be pending.
  getCurrentWindow()
    .onCloseRequested(() => saveNow())
    .catch(() => window.addEventListener("beforeunload", () => void saveNow()));
}

start();
