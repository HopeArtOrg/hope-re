import type { Update } from "@tauri-apps/plugin-updater";

import { createQuery } from "@tanstack/svelte-query";
import { check } from "@tauri-apps/plugin-updater";

let currentUpdate: Update | null = null;

export async function fetchUpdate(): Promise<Update | null> {
  if (currentUpdate) {
    await currentUpdate.close();
    currentUpdate = null;
  }

  const update = await check();
  currentUpdate = update;
  return update;
}

export function useCheckForUpdate() {
  return createQuery(() => ({
    queryKey: ["updater-check"],
    queryFn: fetchUpdate,
    staleTime: 5 * 60 * 1000,
    gcTime: 10 * 60 * 1000,
    retry: 1,
    refetchOnWindowFocus: false,
  }));
}
