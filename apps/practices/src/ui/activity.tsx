import { SkipLink, Surface } from "@libre-ai/ui";
import type { ActivityOutcome } from "../domain/activity-outcome";

function stateLabel(state: ActivityOutcome["state"]): string {
  switch (state) {
    case "in-progress":
      return "En cours";
    case "completed":
      return "Terminée";
    case "stopped":
      return "Arrêtée";
  }
}

export function Activity({
  outcome,
  onComplete,
  onStop,
}: {
  readonly outcome: ActivityOutcome;
  readonly onComplete?: () => void;
  readonly onStop?: () => void;
}) {
  return (
    <>
      <SkipLink targetId="activity" />
      <header className="lai-header lai-page">
        <h1>Libre AI — Pratiques</h1>
        <p>
          Complétez l'activité sur votre appareil. Rien n'est transmis : vos réponses restent en
          stockage local.
        </p>
      </header>
      <main id="activity" className="lai-main lai-page lai-stack" tabIndex={-1}>
        <Surface>
          <div className="lai-stack">
            <div>
              <p data-testid="activity-id">{outcome.activity.activityId}</p>
              <p data-testid="activity-version">{outcome.activity.activityVersion}</p>
              <p data-testid="state">{stateLabel(outcome.state)}</p>
            </div>
            <div className="lai-enhanced-only lai-cluster">
              <button
                type="button"
                aria-pressed={outcome.state === "completed"}
                onClick={onComplete}
              >
                Terminer
              </button>
              <button type="button" aria-pressed={outcome.state === "stopped"} onClick={onStop}>
                Arrêter
              </button>
            </div>
          </div>
        </Surface>
      </main>
    </>
  );
}
