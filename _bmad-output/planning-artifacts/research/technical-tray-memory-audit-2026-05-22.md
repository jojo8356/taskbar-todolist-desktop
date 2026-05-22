---
stepsCompleted: [audit, local-benchmark, recommendation]
inputDocuments:
  - src-tauri/tauri.conf.json
  - src-tauri/src/app/windows.rs
  - src-tauri/src/app/tray.rs
  - src-tauri/Cargo.toml
workflowType: research
lastStep: recommendation
research_type: technical
research_topic: tray-memory-audit
research_goals: identify why release RSS exceeds 150 MB, benchmark practical alternatives, recommend the next implementation path
user_name: Johan
date: 2026-05-22
web_research_enabled: true
source_verification: true
---

# Research Report: Tray Memory Audit

## Executive Summary

The 150 MB RSS failure is not caused by the new direct StatusNotifierItem tray implementation. It is caused primarily by creating a Tauri/WebKitGTK webview at process startup, even though the window is configured as hidden.

The best next move is to keep Tauri and the current TypeScript UI, but switch the tray panel to lazy creation:

- remove the startup window from `tauri.conf.json`
- create the `WebviewWindow` only when the user activates the tray
- hide instead of destroy after first use

This preserves the current architecture and reduces idle RSS from about 158 MB to about 72 MB on this machine. The recommendation has been implemented in the project and validated on the release binary.

## Measurement Method

Benchmarks were run locally on 2026-05-22 using release binaries. Each process was measured at 5, 30, 60, 120, 240, and 300 seconds.

Metrics:

- RSS from `/proc/<pid>/status`
- PSS and Private_Dirty from `/proc/<pid>/smaps_rollup`
- CPU from `ps -p <pid> -o pcpu=`

Why PSS matters: RSS includes shared GTK/WebKit pages and can overstate the memory uniquely attributable to this app. The BMAD acceptance criterion currently uses RSS, so RSS remains the pass/fail metric, but PSS better explains ownership.

## Benchmarks

| Option | Description | 300s RSS | 300s PSS | 300s Private Dirty | 300s CPU | Passes 150 MB RSS? |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| Current Tauri hidden window | Current app, hidden WebView created at startup | 157,912 KiB | 65,943 KiB | 34,696 KiB | 0.2% | No |
| Lazy Tauri window | Same app shape, no startup WebView, create panel on tray activation | 72,360 KiB | 39,687 KiB | 27,776 KiB | 0.0% | Yes |
| Native GTK minimal | Minimal GTK hidden window plus SNI registration | 24,032 KiB | 10,119 KiB | 6,512 KiB | 0.0% | Yes |
| Native SNI only | Minimal D-Bus StatusNotifierItem, no GUI | 4,120 KiB | 2,313 KiB | 2,300 KiB | 0.0% | Yes |
| Project after lazy-window implementation | Actual project after applying the recommendation | 72,056 KiB | 39,250 KiB | 18,588 KiB | 0.0% | Yes |

Raw benchmark samples:

```csv
label,second,rss_kib,pss_kib,private_dirty_kib,cpu_percent
current-hidden,5,158892,66923,35676,4.9
current-hidden,30,158904,66924,35688,0.8
current-hidden,60,158876,66894,35660,0.4
current-hidden,120,158888,66909,35672,0.2
current-hidden,240,157892,65912,34676,0.2
current-hidden,300,157912,65943,34696,0.2
lazy-no-startup-webview,5,72244,39621,27716,1.5
lazy-no-startup-webview,30,72264,39641,27736,0.3
lazy-no-startup-webview,60,72272,39651,27744,0.2
lazy-no-startup-webview,120,72276,39644,27748,0.1
lazy-no-startup-webview,240,72352,39676,27768,0.0
lazy-no-startup-webview,300,72360,39687,27776,0.0
native-gtk-hidden,5,23976,10117,6512,0.3
native-gtk-hidden,30,23976,10114,6512,0.0
native-gtk-hidden,60,23976,10106,6512,0.0
native-gtk-hidden,120,23976,10115,6512,0.0
native-gtk-hidden,240,24032,10117,6512,0.0
native-gtk-hidden,300,24032,10119,6512,0.0
native-sni-only,5,4120,2313,2300,0.0
native-sni-only,30,4120,2313,2300,0.0
native-sni-only,60,4120,2313,2300,0.0
native-sni-only,120,4120,2313,2300,0.0
native-sni-only,240,4120,2314,2300,0.0
native-sni-only,300,4120,2313,2300,0.0
project-lazy-window,5,71964,39176,27264,1.3
project-lazy-window,30,71964,39190,18520,0.2
project-lazy-window,60,71964,39179,18520,0.1
project-lazy-window,120,71976,39208,18532,0.0
project-lazy-window,240,72028,39228,18560,0.0
project-lazy-window,300,72056,39250,18588,0.0
```

## Root Cause

`src-tauri/tauri.conf.json` defines a hidden window:

```json
"windows": [
  {
    "title": "Taskbar Todolist",
    "width": 340,
    "height": 420,
    "resizable": false,
    "visible": false
  }
]
```

Hidden does not mean uncreated. Tauri creates the webview process/runtime at application startup, then `prepare_tray_panel` hides it again. On Linux this pulls in the WebKitGTK stack.

Local dynamic linkage confirms the release binary uses:

- `libwebkit2gtk-4.1.so.0`
- `libjavascriptcoregtk-4.1.so.0`
- `libgtk-3.so.0`
- `libgdk-3.so.0`
- `libsoup-3.0.so.0`
- `libdbus-1.so.3`

The frontend bundle is tiny, around 8 KB gzipped JS/CSS scale, so task rendering or CSS is not the cause of a 150 MB RSS boundary failure.

## Options

### Option A: Keep Current Startup WebView

Keep the implementation as-is and change the acceptance metric from RSS to PSS.

Pros:

- No code change.
- PSS is a better ownership metric for shared GTK/WebKit pages.
- Current PSS is around 66 MB and CPU is acceptable.

Cons:

- It does not satisfy the existing BMAD RSS criterion.
- Task managers and simple process checks will still show around 158 MB RSS.
- Does not improve actual idle footprint perception.

Verdict: acceptable only if the product requirement is changed from RSS to PSS.

### Option B: Lazy-Create the Tauri WebView

Remove the startup window from config and create the panel window only on tray activation.

Pros:

- Best cost/benefit.
- Keeps current Tauri, TypeScript, CSS, commands, and task store.
- Passes the strict RSS criterion by a wide margin: around 72 MB.
- No deprecated libayatana path.
- Small implementation scope.

Cons:

- First tray click pays window creation latency.
- After the first panel is opened, memory will likely move closer to the current WebView-loaded profile unless we destroy the window on hide.
- Requires careful event wiring because `show_tray_panel` must create the window if missing.

Verdict: recommended.

### Option C: Lazy-Create and Destroy WebView on Hide

Create the WebView when opened, destroy it when hidden/closed, keep data in SQLite/Rust.

Pros:

- Idle returns near lazy startup footprint after each close.
- Still keeps Tauri/TypeScript.

Cons:

- More UX risk: reopen latency every time.
- More state lifecycle complexity.
- Need to guarantee unsaved frontend state never matters.

Verdict: useful later if memory after first open becomes a hard requirement.

### Option D: Native GTK UI

Rewrite the compact panel with GTK widgets and keep SNI tray registration.

Pros:

- Minimal benchmark was around 24 MB RSS.
- No WebKitGTK webview cost.
- Very low CPU.

Cons:

- Real rewrite of UI layer.
- Current TypeScript UI work becomes throwaway.
- More Rust GTK code and state binding.
- More design effort for polished task rows, controls, focus behavior, and future full UI.

Verdict: best raw memory, poor short-term ROI.

### Option E: Native SNI Daemon Only

Keep only the tray process and move UI elsewhere.

Pros:

- Around 4 MB RSS in the minimal benchmark.

Cons:

- Not functionally equivalent to the app.
- No task panel.
- Would require a second UI process or a full architecture change.

Verdict: only useful as a lower bound, not a product solution.

## Recommendation

Implement Option B: lazy-create the Tauri webview.

This was selected because it simultaneously:

- fixes the BMAD RSS acceptance criterion
- preserves the existing app architecture
- avoids a native UI rewrite
- keeps the modern direct StatusNotifierItem tray path
- has a measured local result below 150 MB RSS

Implementation status: complete. The project release binary now measures 72,056 KiB RSS and 0.0% CPU at the 5-minute idle sample.

Then update BMAD AC 5 to be explicit:

- idle before first panel open: under 150 MB RSS and under 2% CPU
- after first panel open and hide: either document the WebView-loaded RSS separately, or add a later story for destroy-on-hide if persistent idle-after-use below 150 MB is required

## Source Notes

- Tauri v2 tray docs still require the `tray-icon` feature for the built-in tray API; this project intentionally bypasses that path to avoid the deprecated libayatana warning.
- The StatusNotifierItem spec defines D-Bus registration with a StatusNotifierWatcher and properties such as `Category`, `Status`, `WindowId`, and `IconName`; the direct `zbus` implementation follows that model.
- WebKitGTK documents WebView process models, which aligns with the observed memory behavior: creating a WebView brings in WebKit auxiliary/runtime components even if the app window starts hidden.
