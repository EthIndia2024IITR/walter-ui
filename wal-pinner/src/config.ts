import fs from "fs";
import path from "path";

// Define types for the configuration
export interface FileConfig {
  is_encrypted: boolean;
  blobs: string;
}

export interface Config {
  default_file_download_dir: string;
  default_epochs: number;
  default_shard_size: number, // 1MB
  renew_epoch_threshold: number,
  default_renewal_value: number,
  files: Record<string, FileConfig>;
}

// Load configuration from config.json
export function loadConfig(): Config {
  const configPath = path.resolve("~/.walter/config.json");
  console.log("Loading config from:", configPath);
  const configData = fs.readFileSync(configPath, "utf-8");
  return JSON.parse(configData) as Config;
}
