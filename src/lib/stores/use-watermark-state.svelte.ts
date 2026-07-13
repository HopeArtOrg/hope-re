let lastWatermarkLength = $state<number>(17);
let scannableImage = $state<string | null>(null);
let activeSubTab = $state<string>("embed");
let lastSeed = $state<string>("");
let useSeed = $state<boolean>(false);

export function useWatermarkState() {
  return {
    get lastWatermarkLength() {
      return lastWatermarkLength;
    },
    set lastWatermarkLength(value: number) {
      lastWatermarkLength = value;
    },
    get scannableImage() {
      return scannableImage;
    },
    set scannableImage(value: string | null) {
      scannableImage = value;
    },
    get activeSubTab() {
      return activeSubTab;
    },
    set activeSubTab(value: string) {
      activeSubTab = value;
    },
    get lastSeed() {
      return lastSeed;
    },
    set lastSeed(value: string) {
      lastSeed = value;
    },
    get useSeed() {
      return useSeed;
    },
    set useSeed(value: boolean) {
      useSeed = value;
    },
  };
}
