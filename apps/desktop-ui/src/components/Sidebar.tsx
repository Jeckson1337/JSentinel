import type { NavigationItem, ScreenId } from "../types";

type SidebarProps = {
  appName: string;
  items: NavigationItem[];
  activeScreen: ScreenId;
  onNavigate: (screen: ScreenId) => void;
};

export function Sidebar({ appName, items, activeScreen, onNavigate }: SidebarProps) {
  return (
    <aside className="sidebar">
      <div className="brand-block">
        <div className="brand-mark">JS</div>
        <div>
          <div className="brand-name">{appName}</div>
          <div className="brand-subtitle">Local control center</div>
        </div>
      </div>
      <nav className="nav-list" aria-label="Primary navigation">
        {items.map((item) => {
          const Icon = item.icon;
          const isActive = item.id === activeScreen;

          return (
            <button
              className={isActive ? "nav-item nav-item-active" : "nav-item"}
              key={item.id}
              type="button"
              onClick={() => onNavigate(item.id)}
              title={item.label}
            >
              <Icon size={18} strokeWidth={2} />
              <span>{item.label}</span>
            </button>
          );
        })}
      </nav>
    </aside>
  );
}
