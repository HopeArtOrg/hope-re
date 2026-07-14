import type { Locale, Messages } from "$lib/i18n";

import { LazyStore } from "@tauri-apps/plugin-store";

import { dictionaries } from "$lib/i18n";

const SUPPORTED_LOCALES: Locale[] = ["en", "vi", "ja", "zh"];

const store = new LazyStore("settings.json");

function detectLocale(): Locale {
  if (typeof navigator === "undefined")
    return "en";

  const candidate = navigator.language.slice(0, 2).toLowerCase() as Locale;
  return SUPPORTED_LOCALES.includes(candidate) ? candidate : "en";
}

let locale = $state<Locale>(detectLocale());

function applyLocale(value: Locale) {
  if (typeof document !== "undefined") {
    document.documentElement.lang = value;
  }
}

function resolveMessage(dictionary: Messages, key: string): unknown {
  return key.split(".").reduce<unknown>((node, part) => {
    if (node && typeof node === "object") {
      return (node as Record<string, unknown>)[part];
    }
    return undefined;
  }, dictionary);
}

export function t(key: string, params?: Record<string, string | number>): string {
  let message = resolveMessage(dictionaries[locale], key);

  if (typeof message !== "string") {
    message = resolveMessage(dictionaries.en, key);
  }

  if (typeof message !== "string") {
    return key;
  }

  if (!params) {
    return message;
  }

  return message.replace(/\{(\w+)\}/g, (match, name: string) => {
    const value = params[name];
    return value === undefined ? match : String(value);
  });
}

export function useI18n() {
  async function initLocale() {
    try {
      const savedLocale = await store.get<Locale>("locale");
      if (savedLocale && SUPPORTED_LOCALES.includes(savedLocale)) {
        locale = savedLocale;
      }
      applyLocale(locale);
    }
    catch (error) {
      console.error("Failed to load locale:", error);
      applyLocale(locale);
    }
  }

  function getLocale(): Locale {
    return locale;
  }

  async function setLocale(newLocale: Locale) {
    try {
      locale = newLocale;
      applyLocale(newLocale);
      await store.set("locale", newLocale);
    }
    catch (error) {
      console.error("Failed to save locale:", error);
    }
  }

  return {
    initLocale,
    getLocale,
    setLocale,
  };
}
