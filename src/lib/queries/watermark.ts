import { createMutation } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

export type EmbedWatermarkInput = {
  imageBase64: string;
  watermark: string;
  seed?: number;
};

export type ExtractWatermarkInput = {
  imageBase64: string;
  watermarkLen: number;
  seed?: number;
};

export async function embedWatermark(input: EmbedWatermarkInput): Promise<string> {
  return await invoke<string>("embed_watermark", {
    imageBase64: input.imageBase64,
    watermark: input.watermark,
    seed: input.seed ?? null,
  });
}

export async function extractWatermark(input: ExtractWatermarkInput): Promise<string> {
  return await invoke<string>("extract_watermark", {
    imageBase64: input.imageBase64,
    watermarkLen: input.watermarkLen,
    seed: input.seed ?? null,
  });
}

export function useEmbedWatermark() {
  return createMutation(() => ({
    mutationFn: async (input: EmbedWatermarkInput) =>
      await embedWatermark(input),
  }));
}

export function useExtractWatermark() {
  return createMutation(() => ({
    mutationFn: async (input: ExtractWatermarkInput) =>
      await extractWatermark(input),
  }));
}
