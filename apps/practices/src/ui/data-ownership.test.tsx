import { describe, expect, test } from "bun:test";
import { renderToStaticMarkup } from "react-dom/server";
import { exportOutcome } from "../domain/activity-outcome";
import { DataOwnership } from "./data-ownership";
import { ACTIVITY_FIXTURE } from "./fixture";

const noop = async (): Promise<void> => {};
const exportFixture = () => exportOutcome(ACTIVITY_FIXTURE);

describe("DataOwnership", () => {
  test("region is enhanced-only and lists export + delete in the idle state", () => {
    const html = renderToStaticMarkup(
      <DataOwnership exportData={exportFixture} onDeleteAll={noop} />,
    );
    expect(html).toContain("lai-enhanced-only");
    expect(html).toContain("Télécharger mon activité");
    expect(html).toContain("Supprimer mon activité");
    // The confirm controls appear only after the delete button is pressed (client
    // only): the SSR/idle markup must not contain them.
    expect(html).not.toContain("Confirmer la suppression");
    expect(html).not.toContain("delete-confirm");
    // Baseline discipline: no inline styles (matches the rest of the app).
    expect(html).not.toContain("style=");
  });

  test("export is always enabled (an outcome always exists)", () => {
    const html = renderToStaticMarkup(
      <DataOwnership exportData={exportFixture} onDeleteAll={noop} />,
    );
    expect(html).not.toContain("disabled");
  });
});
