const invoke = window.__TAURI__.core.invoke;
let config;
let timer;

const esc = (value) => String(value ?? "").replace(
  /[&<>"']/g,
  (character) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[character],
);

function formatPercent(value) {
  return value == null ? "—" : `${Math.round(value)}%`;
}

function resetCountdown(date) {
  if (!date) return "Reset unavailable";
  const milliseconds = new Date(date) - Date.now();
  if (milliseconds <= 0) return "Reset time passed";
  const minutes = Math.floor(milliseconds / 60000);
  const days = Math.floor(minutes / 1440);
  const hours = Math.floor((minutes % 1440) / 60);
  const remainder = minutes % 60;
  return days > 0 ? `Resets in ${days}d ${hours}h` : `Resets in ${hours}h ${remainder}m`;
}

function resetDate(date) {
  if (!date) return "";
  return new Date(date).toLocaleString([], {
    weekday: "short", month: "short", day: "numeric", hour: "2-digit", minute: "2-digit",
  });
}

function renderWindow(window) {
  const remaining = Math.max(0, Math.min(100, window.remaining_percent ?? 0));
  return `<div class="usage-window">
    <div class="window-head"><strong>${esc(window.display_name)}</strong><span class="window-state ${esc(window.state)}">${esc(window.state)}</span></div>
    <div class="percent-row"><span><b>${esc(formatPercent(window.used_percent))}</b> used</span><span><b>${esc(formatPercent(window.remaining_percent))}</b> left</span></div>
    <div class="meter" role="meter" aria-valuemin="0" aria-valuemax="100" aria-valuenow="${remaining}" aria-label="${esc(window.display_name)} remaining"><span style="width:${remaining}%"></span></div>
    <div class="reset-row"><span>${esc(resetCountdown(window.resets_at))}</span><span>${esc(resetDate(window.resets_at))}</span></div>
  </div>`;
}

function renderProvider(provider) {
  const windows = provider.windows.map(renderWindow).join("");
  const unavailable = `<div class="unavailable"><strong>Usage unavailable</strong><p>${esc(provider.message || "No supported local usage source detected.")}</p></div>`;
  return `<section class="provider">
    <div class="provider-title"><h2>${esc(provider.provider.toUpperCase())}</h2><span class="status ${esc(provider.status)}">${esc(provider.status)}</span></div>
    ${windows || unavailable}
  </section>`;
}

async function refreshUsage() {
  const button = document.getElementById("refresh");
  button.disabled = true;
  button.classList.add("spinning");
  try {
    const snapshots = await invoke("refresh_usage");
    document.getElementById("providers").innerHTML = snapshots.length
      ? snapshots.map(renderProvider).join("")
      : '<div class="loading">All providers are disabled.</div>';
    document.getElementById("freshness").textContent = `Updated ${new Date().toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}`;
  } catch (error) {
    document.getElementById("providers").innerHTML = `<div class="loading">${esc(error)}</div>`;
  } finally {
    button.disabled = false;
    button.classList.remove("spinning");
  }
}

window.refreshUsage = refreshUsage;
function applyTheme(theme) { document.documentElement.dataset.theme = theme === "system" ? "" : theme; }
function schedule() { clearInterval(timer); timer = setInterval(refreshUsage, Math.max(1, config.refresh_minutes) * 60000); }

async function load() {
  config = await invoke("get_config");
  applyTheme(config.theme);
  for (const [id, key] of [["theme", "theme"], ["always", "always_on_top"], ["tray", "close_to_tray"], ["startup", "start_with_windows"], ["interval", "refresh_minutes"], ["codex", "codex_enabled"], ["claude", "claude_enabled"]]) {
    const element = document.getElementById(id);
    if (element.type === "checkbox") element.checked = config[key]; else element.value = String(config[key]);
  }
  schedule();
  await refreshUsage();
}

document.getElementById("refresh").onclick = refreshUsage;
document.getElementById("settingsButton").onclick = () => document.getElementById("settings").showModal();
document.getElementById("save").onclick = async (event) => {
  event.preventDefault();
  config = { ...config, theme: theme.value, always_on_top: always.checked, close_to_tray: tray.checked, start_with_windows: startup.checked, refresh_minutes: Number(interval.value), codex_enabled: codex.checked, claude_enabled: claude.checked };
  await invoke("save_config", { config });
  applyTheme(config.theme);
  schedule();
  settings.close();
  await refreshUsage();
};
load();
