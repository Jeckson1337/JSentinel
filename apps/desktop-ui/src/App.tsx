import { useMemo, useState } from "react";
import {
  Activity,
  Clock3,
  Cpu,
  FolderSearch,
  Globe2,
  Info,
  PlaySquare,
  Settings,
} from "lucide-react";
import { Sidebar } from "./components/Sidebar";
import { SponsorBanner } from "./components/SponsorBanner";
import { DashboardScreen } from "./screens/DashboardScreen";
import { PlaceholderScreen } from "./screens/PlaceholderScreen";
import { SettingsScreen } from "./screens/SettingsScreen";
import { AboutScreen } from "./screens/AboutScreen";
import { TimelineScreen } from "./screens/TimelineScreen";
import { seedDemoEvents } from "./events";
import { getDictionary, type Locale } from "./i18n";
import type { NavigationItem, ScreenId } from "./types";

type PlaceholderScreenId = "processes" | "network" | "files" | "startup";

function isPlaceholderScreen(screen: ScreenId): screen is PlaceholderScreenId {
  return ["processes", "network", "files", "startup"].includes(screen);
}

export function App() {
  const [locale, setLocale] = useState<Locale>("en");
  const [activeScreen, setActiveScreen] = useState<ScreenId>("dashboard");
  const [refreshToken, setRefreshToken] = useState(0);
  const [seedStatus, setSeedStatus] = useState<string | null>(null);
  const t = getDictionary(locale);

  const navigation = useMemo<NavigationItem[]>(
    () => [
      { id: "dashboard", label: t.nav.dashboard, icon: Activity },
      { id: "timeline", label: t.nav.timeline, icon: Clock3 },
      { id: "processes", label: t.nav.processes, icon: Cpu },
      { id: "network", label: t.nav.network, icon: Globe2 },
      { id: "files", label: t.nav.files, icon: FolderSearch },
      { id: "startup", label: t.nav.startup, icon: PlaySquare },
      { id: "settings", label: t.nav.settings, icon: Settings },
      { id: "about", label: t.nav.about, icon: Info },
    ],
    [t],
  );

  async function handleSeedDemoEvents() {
    const result = await seedDemoEvents();
    setRefreshToken((value) => value + 1);
    setSeedStatus(
      result.source === "tauri_sqlite"
        ? t.settings.seedDemoEventsDone.replace("{count}", String(result.data))
        : t.settings.seedDemoEventsFallback,
    );
  }

  return (
    <div className="app-shell">
      <Sidebar
        appName="JSentinel"
        items={navigation}
        activeScreen={activeScreen}
        onNavigate={setActiveScreen}
      />
      <main className="main-surface">
        <SponsorBanner text={t.sponsor.placeholder} note={t.sponsor.note} />
        {activeScreen === "dashboard" && <DashboardScreen t={t} refreshToken={refreshToken} />}
        {activeScreen === "timeline" && <TimelineScreen t={t} refreshToken={refreshToken} />}
        {activeScreen === "settings" && (
          <SettingsScreen
            locale={locale}
            setLocale={setLocale}
            t={t}
            onSeedDemoEvents={handleSeedDemoEvents}
            seedStatus={seedStatus}
          />
        )}
        {activeScreen === "about" && <AboutScreen t={t} />}
        {isPlaceholderScreen(activeScreen) && (
          <PlaceholderScreen
            title={navigation.find((item) => item.id === activeScreen)?.label ?? ""}
            description={t.placeholders[activeScreen]}
            badge={t.common.planned}
          />
        )}
      </main>
    </div>
  );
}
