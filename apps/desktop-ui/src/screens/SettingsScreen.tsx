import type { Dictionary, Locale } from "../i18n";

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
        <section className="settings-panel">
          <h2>{t.settings.language}</h2>
          <select value={locale} onChange={(event) => setLocale(event.target.value as Locale)}>
            <option value="en">English</option>
            <option value="ru">Русский</option>
          </select>
        </section>

        <section className="settings-panel">
          <h2>{t.settings.privacy}</h2>
          <p>{t.settings.privacyDescription}</p>
        </section>

        <section className="settings-panel">
          <h2>{t.settings.sponsor}</h2>
          <p>{t.settings.sponsorDescription}</p>
        </section>

        <section className="settings-panel">
          <h2>{t.settings.releaseChannel}</h2>
          <p>{t.settings.releaseDescription}</p>
        </section>

        <section className="settings-panel">
          <h2>{t.settings.demoData}</h2>
          <p>{t.settings.demoDataDescription}</p>
          <button className="primary-button" type="button" onClick={onSeedDemoEvents}>
            {t.settings.seedDemoEvents}
          </button>
          {seedStatus && <p className="settings-status">{seedStatus}</p>}
        </section>
      </div>
    </section>
  );
}
