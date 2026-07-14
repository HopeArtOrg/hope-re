import type { ProtectionMenuProps } from "./components";
import type { Icon as IconType } from "@lucide/svelte";

import {
  BrushIcon,
  Building2Icon,
  CarIcon,
  CatIcon,
  ClockIcon,
  DogIcon,
  DropletsIcon,
  GaugeIcon,
  LandmarkIcon,
  LayoutGridIcon,
  PaletteIcon,
  PencilIcon,
  ShapesIcon,
  TargetIcon,
  UtensilsIcon,
  ZapIcon,
} from "@lucide/svelte";

export const MODEL_ALGO_SUFFIX = "_algorithm.onnx";

export const algorithms: {
  value: Exclude<ProtectionMenuProps["algorithm"], undefined>;
  icon: typeof IconType;
  colour: string;
  bgColour: string;
}[] = [
  {
    value: "noise",
    icon: ZapIcon,
    colour: "text-indigo-500/80 dark:text-indigo-400/80",
    bgColour: "bg-indigo-500/5",
  },
  {
    value: "glaze",
    icon: PaletteIcon,
    colour: "text-violet-500/80 dark:text-violet-400/80",
    bgColour: "bg-violet-500/5",
  },
  {
    value: "nightshade",
    icon: TargetIcon,
    colour: "text-rose-500/80 dark:text-rose-400/80",
    bgColour: "bg-rose-500/5",
  },
];

export const glazeStyles: {
  value: Exclude<ProtectionMenuProps["glazeStyle"], undefined>;
  icon: typeof IconType;
}[] = [
  { value: "abstract", icon: ShapesIcon },
  { value: "impressionist", icon: BrushIcon },
  { value: "cubist", icon: LayoutGridIcon },
  { value: "sketch", icon: PencilIcon },
  { value: "watercolor", icon: DropletsIcon },
];

export const nightshadeTargets: {
  value: Exclude<ProtectionMenuProps["nightshadeTarget"], undefined>;
  icon: typeof IconType;
}[] = [
  { value: "dog", icon: DogIcon },
  { value: "cat", icon: CatIcon },
  { value: "car", icon: CarIcon },
  { value: "landscape", icon: LandmarkIcon },
  { value: "person", icon: LandmarkIcon },
  { value: "building", icon: Building2Icon },
  { value: "food", icon: UtensilsIcon },
  { value: "abstract", icon: ShapesIcon },
];

export const qualityPresets: {
  value: number;
  key: string;
  icon: typeof IconType;
  colour: string;
}[] = [
  {
    value: 0,
    key: "faster",
    icon: ZapIcon,
    colour: "text-emerald-500/70 dark:text-emerald-400/70",
  },
  {
    value: 25,
    key: "fast",
    icon: ZapIcon,
    colour: "text-teal-500/70 dark:text-teal-400/70",
  },
  {
    value: 50,
    key: "default",
    icon: GaugeIcon,
    colour: "text-sky-500/70 dark:text-sky-400/70",
  },
  {
    value: 75,
    key: "slower",
    icon: ClockIcon,
    colour: "text-amber-500/70 dark:text-amber-400/70",
  },
  {
    value: 100,
    key: "slowest",
    icon: ClockIcon,
    colour: "text-rose-500/70 dark:text-rose-400/70",
  },
];
