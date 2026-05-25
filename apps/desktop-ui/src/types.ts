import type { LucideIcon } from "lucide-react";

export type ScreenId =
  | "dashboard"
  | "timeline"
  | "processes"
  | "network"
  | "files"
  | "startup"
  | "settings"
  | "about";

export type NavigationItem = {
  id: ScreenId;
  label: string;
  icon: LucideIcon;
};
