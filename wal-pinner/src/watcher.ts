import chokidar from "chokidar";
import { Config, loadConfig } from "./config";
import path from "path";

export function watchConfig(onConfigChange: (newConfig: Config) => void): void {
  const configPath = path.resolve("/home/phoenix/.walter/config.json");

  chokidar.watch(configPath).on("change", () => {
    console.log("Config file changed. Reloading...");
    try {
      const newConfig = loadConfig();
      onConfigChange(newConfig);
      console.log("Config reloaded:", newConfig);
    } catch (err) {
      console.error("Failed to reload config:", err);
    }
  });
}
