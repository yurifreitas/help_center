import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [diskUsage, setDiskUsage] = useState("");

  async function getDiskUsage() {
    try {
      console.log('invoke');
      const usage = await invoke('check_disk_usage');
      setDiskUsage(usage);
      console.log('Disk usage:', usage);
    } catch (error) {
      console.error("Failed to get disk usage:", error);
      setDiskUsage("Error fetching disk usage");
    }
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>
      <button onClick={getDiskUsage}>Check Disk Usage</button>
      <p>Disk Usage: {diskUsage}</p>
    </div>
  );
}

export default App;
