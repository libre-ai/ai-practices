import { describe, expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { ActivityApp } from "./activity-app";

describe("ActivityApp — SSR baseline", () => {
  test("renders the activity with no store (server render)", () => {
    const html = renderToStaticMarkup(<ActivityApp />);
    expect(html).toContain("urn:libre-ai:activity:learning-foundations");
    expect(html).toContain("En cours");
    expect(html).not.toContain("corrupt-notice");
    expect(html).not.toContain("style=");
  });
});
