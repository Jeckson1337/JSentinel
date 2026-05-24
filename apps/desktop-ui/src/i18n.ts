import en from "./locales/en.json";
import ru from "./locales/ru.json";

export type Locale = "en" | "ru";
export type Dictionary = typeof en;

const dictionaries = { en, ru };

export function getDictionary(locale: Locale) {
  return dictionaries[locale];
}
