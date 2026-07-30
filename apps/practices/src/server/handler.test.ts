import { describe, expect, test } from "bun:test";
import { createPracticesHandler } from "./handler";

const handler = createPracticesHandler(undefined, () => "req_0000000000000000");

describe("practices handler", () => {
  test("serves the SSR activity at /", async () => {
    const response = await handler(new Request("https://practices.test/"));
    expect(response.status).toBe(200);
    expect(response.headers.get("content-type")).toContain("text/html");
    const html = await response.text();
    expect(html).toContain("Libre AI — Pratiques");
    expect(html).toContain("urn:libre-ai:activity:learning-foundations");
  });

  test("reports health as JSON", async () => {
    const response = await handler(new Request("https://practices.test/api/health"));
    expect(response.status).toBe(200);
    expect(await response.json()).toEqual({
      service: "libre-ai-practices",
      status: "ok",
      version: "v1",
    });
  });

  test("an unknown route is not found", async () => {
    const response = await handler(new Request("https://practices.test/nope"));
    expect(response.status).toBe(404);
  });
});
