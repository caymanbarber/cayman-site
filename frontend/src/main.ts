import "./styles/main.css";
import { checkHealth } from "./api";

const statusEl = document.getElementById("status");

async function init() {
  const healthy = await checkHealth();

  if (statusEl) {
    statusEl.textContent = healthy
      ? "// status: compiled ✓"
      : "// status: backend offline";
  }
}

init();
