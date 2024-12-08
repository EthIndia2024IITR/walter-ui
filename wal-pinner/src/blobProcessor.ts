import axios from "axios";
import fs from "fs";
import { Config } from "./config";

const outputFilePath = "/tmp/for-pinning";
const PUBLISHER = "https://publisher.walrus-testnet.walrus.space/"
const AGGREGATOR = "https://aggregator.walrus-testnet.walrus.space/"

export async function processFiles(config: Config): Promise<void> {
  console.log("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");
  console.log("Processing files...");
  console.log("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");
  while (true) {
    try {
      for (const [fileName, fileConfig] of Object.entries(config.files)) {
        const triggerAmount = config.renew_epoch_threshold;
        const blobId = fileConfig.blobs;

        // Process each blob
        await processBlob(fileName, blobId, triggerAmount);
      }
    } catch (err) {
      console.error("Error processing files:", err);
    }

    // Wait for 1 hour before the next iteration
    await new Promise((resolve) => setTimeout(resolve, 3600000));
  }
}

async function processBlob(fileName: string, blobId: string, triggerAmount: number): Promise<void> {
  try {
    // Simulate a request to check the epoch
    const epochResponse = await axios.get(`${AGGREGATOR}/v1/${blobId}`, { responseType: "arraybuffer" });

    const filePath = `/tmp/${fileName}-${blobId}`;
    fs.writeFileSync(filePath, epochResponse.data);

    const fileStream = fs.createReadStream("/tmp/" + fileName + "-" + blobId); // Make the PUT request

    const response = await axios.put(`${PUBLISHER}/v1/store?epochs=5`, fileStream, {
      headers: {
        "Content-Type": "application/octet-stream", // Set appropriate content type
      },
    });

    console.log("Response data in just reading:", response.data);

    if (response.data.alreadyCertified.endEpoch - 51 > triggerAmount) {
      console.log(`Epoch is less than renew_epoch_threshold. Downloading blob into RAM...`);

      const response = await axios.put(`${PUBLISHER}/v1/store?epochs=20`, fileStream, {
        headers: {
          "Content-Type": "application/octet-stream", // Set appropriate content type
        },
      });
      const blobData = response.data;
      console.log("Response data:", blobData);

      console.log(`Blob ID: ${blobId} downloaded into RAM.`);
    }
  } catch (err) {
    console.error(`Failed to process blob ${blobId}:`, err);
  }
}
