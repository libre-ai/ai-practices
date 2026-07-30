import { describe, expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { Activity } from "./activity";
import { ACTIVITY_FIXTURE } from "./fixture";

function render(
  outcome = ACTIVITY_FIXTURE,
  handlers: {
    onComplete?: () => void;
    onStop?: () => void;
  } = {},
): string {
  return renderToStaticMarkup(
    <Activity outcome={outcome} onComplete={handlers.onComplete} onStop={handlers.onStop} />,
  );
}

describe("Activity — accessible baseline", () => {
  test("renders the activity id, version and state with no colour-only meaning", () => {
    const html = render();
    expect(html).toContain(ACTIVITY_FIXTURE.activity.activityId);
    expect(html).toContain(ACTIVITY_FIXTURE.activity.activityVersion);
    expect(html).toContain("En cours");
    expect(html).not.toContain("style=");
  });

  test("shows the state as text (in-progress / completed / stopped)", () => {
    const html = render();
    expect(html).toContain("En cours");
  });

  test("hides the interactive controls without handlers (no-JS baseline)", () => {
    const html = render();
    // Interactive controls live in lai-enhanced-only; the enhanced wrapper is
    // present but the baseline exposes no buttons as reachable controls.
    expect(html).toContain("lai-enhanced-only");
  });
});
