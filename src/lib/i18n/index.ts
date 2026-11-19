import type { Messages } from "./locales/en";

import { en } from "./locales/en";
import { es } from "./locales/es";
import { ja } from "./locales/ja";
import { vi } from "./locales/vi";
import { zh } from "./locales/zh";

export type { Messages };

export type Locale = "en" | "vi" | "ja" | "zh" | "es";

export const dictionaries: Record<Locale, Messages> = { en, vi, ja, zh, es };

export const locales: { value: Locale; label: string }[] = [
  { value: "en", label: "English" },
  { value: "vi", label: "Tiếng Việt" },
  { value: "ja", label: "日本語" },
  { value: "zh", label: "中文" },
  { value: "es", label: "Español" },
];
