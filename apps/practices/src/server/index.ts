import { parseServerAddress } from "@libre-ai/web-platform";
import { createPracticesHandler } from "./handler";

const { hostname, port } = parseServerAddress(Bun.env);
const fetch = createPracticesHandler();

const server = Bun.serve({
  fetch,
  hostname,
  port,
});

console.log(`Libre AI Practices listening on ${server.url.origin}`);
