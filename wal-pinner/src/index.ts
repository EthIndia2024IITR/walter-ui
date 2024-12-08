import { loadConfig, Config } from "./config";
import { watchConfig } from "./watcher";
import { processFiles } from "./blobProcessor";
import { startServer } from "./server";
import dotenv from "dotenv";

// Load environment variables
dotenv.config();

let config: Config = loadConfig();

// Watch the config file for changes
watchConfig((newConfig: Config) => {
  config = newConfig;
  processFiles(config);
});

// Start processing files
processFiles(config);

// Start the HTTP server
startServer();
