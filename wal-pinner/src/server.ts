import express, { Request, Response } from "express";

export function startServer(): void {
  const app = express();

  app.get("/health", (req: Request, res: Response) => {
    res.send("Server is running");
  });

  const PORT = 1337;
  app.listen(PORT, () => {
    console.log(`Server is running on http://127.0.0.1:${PORT}`);
  });
}
