import type { Dictionary, Locale } from "../i18n";
import { SectionCard, StatusBadge } from "../components/ui";

type SettingsScreenProps = {
  locale: Locale;
  setLocale: (locale: Locale) => void;
  t: Dictionary;
  onSeedDemoEvents: () => void;
  seedStatus: string | null;
};

export function SettingsScreen({
  locale,
  setLocale,
  t,
  onSeedDemoEvents,
  seedStatus,
}: SettingsScreenProps) {
  return (
    <section className="screen">
      <div className="screen-heading">
        <p className="eyebrow">{t.nav.settings}</p>
        <h1>{t.settings.title}</h1>
        <p>{t.settings.subtitle}</p>
      </div>

      <div className="settings-grid">
        <SectionCard title={t.settings.language} description={t.settings.languageDescription}>
          <select value={locale} onChange={(event) => setLocale(event.target.value as Locale)}>
            <option value="en">English</option>
            <option value="ru">Русский</option>
          </select>
        </SectionCard>

        <SectionCard title={t.settings.privacy} description={t.settings.privacyDescription}>
          <div className="badge-list">
            <StatusBadge label={t.settings.localOnly} tone="success" />
            <StatusBadge label={t.common.noTelemetry} tone="success" />
            <StatusBadge label={t.settings.noAccounts} tone="success" />
            <StatusBadge label={t.settings.noAdSdk} tone="success" />
          </div>
        </SectionCard>

        <SectionCard title={t.settings.sponsor} description={t.settings.sponsorDescription}>
          <p className="muted-line">{t.settings.sponsorDetails}</p>
        </SectionCard>

        <SectionCard title={t.settings.storage} description={t.settings.storageDescription}>
          <p className="code-line">.jsentinel-dev/jsentinel.sqlite3</p>
          <p className="muted-line">{t.settings.demoDataDescription}</p>
          <button className="primary-button" type="button" onClick={onSeedDemoEvents}>
            {t.settings.seedDemoEvents}
          </button>
          {seedStatus && <p className="settings-status">{seedStatus}</p>}
        </SectionCard>

        <SectionCard title={t.settings.releaseChannel} description={t.settings.releaseDescription}>
          <p className="muted-line">{t.settings.releaseDetails}</p>
        </SectionCard>
      </div>
    </section>
  );
}
