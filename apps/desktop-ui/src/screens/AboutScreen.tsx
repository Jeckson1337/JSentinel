import type { Dictionary } from "../i18n";

export function AboutScreen({ t }: { t: Dictionary }) {
  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.about}</p>
        <h1>JSentinel</h1>
        <p>{t.about.description}</p>
      </div>
      <div className="about-strip">
        <span>{t.common.noTelemetry}</span>
        <span>{t.common.notAntivirus}</span>
        <span>{t.common.githubReleases}</span>
      </div>
    </section>
  );
}
