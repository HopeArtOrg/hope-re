import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { toast } from "svelte-sonner";

import { t } from "$lib/stores/use-i18n.svelte";

const ISO_REPLACE_REGEX = /[:.]/g;
const DATA_URL_MIME_REGEX = /^data:(image\/[a-z+]+);base64,/i;

function extensionForDataUrl(dataUrl: string): string {
  const mime = dataUrl.match(DATA_URL_MIME_REGEX)?.[1]?.toLowerCase();
  if (mime === "image/jpeg")
    return "jpg";
  if (mime === "image/webp")
    return "webp";
  return "png";
}

export function useImage() {
  let originalImage = $state<string | null>(null);
  let fullscreenOpen = $state<boolean>(false);
  let uploadToken = 0;

  async function handleUpload(files: File[]) {
    const file = files[0];
    if (!file)
      return;

    const token = ++uploadToken;
    const reader = new FileReader();

    reader.onload = (e) => {
      if (token !== uploadToken)
        return;
      originalImage = e.target?.result as string;
      toast.success(t("image.loaded", { name: file.name }));
    };

    reader.onerror = () => {
      if (token !== uploadToken)
        return;
      toast.error(t("image.failedLoad"));
    };

    reader.readAsDataURL(file);
  }

  async function handleDownload(renderedImage: string | null, algorithm: string) {
    if (!renderedImage)
      return;

    try {
      const extension = extensionForDataUrl(renderedImage);
      const timestamp = new Date().toISOString().replace(ISO_REPLACE_REGEX, "-").slice(0, -5);
      const defaultPath = `protected-${algorithm}-${timestamp}.${extension}`;

      const filePath = await save({
        filters: [{
          name: t("image.filterName"),
          extensions: [extension],
        }],
        defaultPath,
      });

      if (!filePath)
        return;

      const response = await fetch(renderedImage);
      const arrayBuffer = await response.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);

      await writeFile(filePath, bytes);
      toast.success(t("image.saved"));
    }
    catch (error) {
      toast.error(t("image.failedSave"));
      console.error("Save error:", error);
    }
  }

  function handleFullscreen() {
    fullscreenOpen = true;
  }

  function clear() {
    uploadToken++;
    originalImage = null;
    fullscreenOpen = false;
  }

  return {
    get originalImage() {
      return originalImage;
    },
    set originalImage(value: string | null) {
      originalImage = value;
    },
    get fullscreenOpen() {
      return fullscreenOpen;
    },
    set fullscreenOpen(value: boolean) {
      fullscreenOpen = value;
    },
    get hasImage() {
      return !!originalImage;
    },
    handleUpload,
    handleDownload,
    handleFullscreen,
    clear,
  };
}
