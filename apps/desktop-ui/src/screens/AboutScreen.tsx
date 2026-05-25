import type { Dictionary } from "../i18n";
import { SectionCard, StatusBadge } from "../components/ui";

export function AboutScreen({ t }: { t: Dictionary }) {
  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.about}</p>
        <h1>JSentinel</h1>
        <p>{t.about.description}</p>
      </div>

      <div className="about-grid">
        <SectionCard title={t.about.whatItIsTitle} description={t.about.whatItIs}>
          <div className="badge-list">
            <StatusBadge label={t.common.noTelemetry} tone="success" />
            <StatusBadge label={t.common.notAntivirus} tone="warning" />
            <StatusBadge label={t.common.githubReleases} tone="info" />
          </div>
        </SectionCard>

        <SectionCard title={t.about.statusTitle} description={t.about.statusDescription}>
          <ul className="plain-list">
            <li>{t.about.foundation}</li>
            <li>{t.about.eventModel}</li>
            <li>{t.about.mockData}</li>
            <li>{t.about.noBackend}</li>
          </ul>
        </SectionCard>

        <SectionCard title={t.about.platformsTitle} description={t.about.platformsDescription}>
          <div className="badge-list">
            <StatusBadge label="Windows 10/11" tone="success" />
            <StatusBadge label="Linux planned/beta" tone="neutral" />
          </div>
        </SectionCard>

        <SectionCard title={t.about.linksTitle} description={t.about.linksDescription}>
          <p className="code-line">https://github.com/Jeckson1337/JSentinel</p>
          <p className="code-line">docs/</p>
          <p className="code-line">GitHub Releases</p>
        </SectionCard>
      </div>
    </section>
  );
}
