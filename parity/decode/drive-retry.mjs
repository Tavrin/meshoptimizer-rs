// Headless Chromium, no window, no audio. Pins the browser to CPU 2.
import { spawn } from "node:child_process";
import { writeFileSync, readFileSync } from "node:fs";
import { setTimeout as delay } from "node:timers/promises";

const root = "<scratch>/meshopt-decode/browser";
const port = 9349;
const httpPort = 8789;
const profile = root + "/chrome-profile";
const bytesOnly = process.argv.includes("--bytes-only");

const http = spawn("python3", ["-m", "http.server", String(httpPort), "--bind", "127.0.0.1"], {
    cwd: root,
    stdio: "ignore",
});
const chrome = spawn("taskset", ["-c", "2", "google-chrome",
    "--headless=new",
    "--disable-gpu",
    "--mute-audio",
    "--no-first-run",
    "--no-default-browser-check",
    "--disable-background-timer-throttling",
    "--disable-renderer-backgrounding",
    "--user-data-dir=" + profile,
    "--remote-debugging-port=" + port,
    "about:blank",
], { stdio: "ignore" });

function stop() {
    chrome.kill("SIGTERM");
    http.kill("SIGTERM");
}

async function getJson(url, method = "GET") {
    const res = await fetch(url, { method });
    if (!res.ok) throw new Error(url + " " + method + " " + res.status);
    return res.json();
}

try {
    let version = null;
    for (let i = 0; i < 50; i++) {
        try {
            version = await getJson("http://127.0.0.1:" + port + "/json/version");
            break;
        } catch {
            await delay(200);
        }
    }
    if (!version) throw new Error("chrome debugger did not open");
    const url = "http://127.0.0.1:" + httpPort + "/page.html";
    const tab = await getJson("http://127.0.0.1:" + port + "/json/new?" + url, "PUT");
    const ws = new WebSocket(tab.webSocketDebuggerUrl);
    await new Promise((resolve, reject) => {
        ws.addEventListener("open", resolve);
        ws.addEventListener("error", reject);
    });
    let next = 1;
    const pending = new Map();
    ws.addEventListener("message", (event) => {
        const msg = JSON.parse(event.data);
        if (msg.id && pending.has(msg.id)) {
            const slot = pending.get(msg.id);
            pending.delete(msg.id);
            if (msg.error) slot.reject(new Error(JSON.stringify(msg.error)));
            else slot.resolve(msg.result);
        }
    });
    function send(method, params) {
        const id = next++;
        ws.send(JSON.stringify({ id, method, params }));
        return new Promise((resolve, reject) => pending.set(id, { resolve, reject }));
    }
    await send("Runtime.enable", {});
    // The initial tab URL may still be navigating when CDP attaches. Wait
    // for its stable document before awaiting the measurement promise.
    let ready = false;
    for (let i = 0; i < 100; i++) {
        try {
            const state = await send("Runtime.evaluate", {
                expression: "document.readyState === 'complete' && typeof window.__done !== 'undefined'",
                returnByValue: true,
            });
            if (state.result?.value === true) { ready = true; break; }
        } catch (error) {
            if (!String(error).includes("Execution context was destroyed")) throw error;
        }
        await delay(100);
    }
    if (!ready) throw new Error("page document did not settle");
    await send("Runtime.evaluate", {
        expression: "new Promise((resolve) => { const tick = () => window.__done ? resolve(true) : setTimeout(tick, 50); tick(); })",
        awaitPromise: true,
    });
    const evaluated = await send("Runtime.evaluate", {
        expression: "window.__done",
        awaitPromise: true,
        returnByValue: true,
    });
    if (evaluated.exceptionDetails) {
        throw new Error(JSON.stringify(evaluated.exceptionDetails));
    }
    const result = evaluated.result.value;
    result.browser = version.Browser;
    result.user_agent_protocol = version["User-Agent"];
    let affinity = "";
    try {
        affinity = readFileSync("/proc/" + chrome.pid + "/status", "utf8")
            .split("\n")
            .filter((line) => line.startsWith("Cpus_allowed_list"))
            .join(" ");
    } catch {
        affinity = "unread";
    }
    result.cpus_allowed = affinity;
    const out = bytesOnly ? root + "/data/chromium-bytes.json" : root + "/data/chromium-timing.json";
    writeFileSync(out, JSON.stringify(result));
    const fails = result.rows.filter((row) => row.excluded || Object.values(row.bytes).some((v) => v !== "match"));
    console.log(JSON.stringify({
        out,
        browser: version.Browser,
        streams: result.rows.length,
        excluded: result.rows.filter((row) => row.excluded).length,
        not_match: fails.length,
        stop_timing: result.stop_timing,
        timer_resolution_ms: result.timer_resolution_ms,
        cpus_allowed: affinity,
    }));
    ws.close();
} finally {
    stop();
}
