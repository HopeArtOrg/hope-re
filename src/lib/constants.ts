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

export const algorithms: {
  value: Exclude<ProtectionMenuProps["algorithm"], undefined>;
  label: string;
  description: string;
  icon: typeof IconType;
  colour: string;
  bgColour: string;
}[] = [
  {
    value: "noise",
    label: "Noise",
    description: "Add adversarial noise",
    icon: ZapIcon,
    colour: "text-indigo-500/80 dark:text-indigo-400/80",
    bgColour: "bg-indigo-500/5",
  },
  {
    value: "glaze",
    label: "Glaze",
    description: "Style transfer protection",
    icon: PaletteIcon,
    colour: "text-violet-500/80 dark:text-violet-400/80",
    bgColour: "bg-violet-500/5",
  },
  {
    value: "nightshade",
    label: "Nightshade",
    description: "Data poisoning protection",
    icon: TargetIcon,
    colour: "text-rose-500/80 dark:text-rose-400/80",
    bgColour: "bg-rose-500/5",
  },
];

export const glazeStyles: {
  value: Exclude<ProtectionMenuProps["glazeStyle"], undefined>;
  label: string;
  description: string;
  icon: typeof IconType;
}[] = [
  { value: "abstract", label: "Abstract", description: "Chaotic brushstrokes", icon: ShapesIcon },
  { value: "impressionist", label: "Impressionist", description: "Gentle light", icon: BrushIcon },
  { value: "cubist", label: "Cubist", description: "Fragmented forms", icon: LayoutGridIcon },
  { value: "sketch", label: "Sketch", description: "Loose pencil lines", icon: PencilIcon },
  { value: "watercolor", label: "Watercolor", description: "Flowing colors", icon: DropletsIcon },
];

export const nightshadeTargets: {
  value: Exclude<ProtectionMenuProps["nightshadeTarget"], undefined>;
  label: string;
  description: string;
  icon: typeof IconType;
}[] = [
  { value: "dog", label: "Dog", description: "Tricks AI into seeing a dog", icon: DogIcon },
  { value: "cat", label: "Cat", description: "Tricks AI into seeing a cat", icon: CatIcon },
  { value: "car", label: "Car", description: "Tricks AI into seeing a car", icon: CarIcon },
  { value: "landscape", label: "Landscape", description: "Tricks AI into seeing a landscape", icon: LandmarkIcon },
  { value: "person", label: "Person", description: "Tricks AI into seeing a person", icon: LandmarkIcon }, // Generic fallback
  { value: "building", label: "Building", description: "Tricks AI into seeing a building", icon: Building2Icon },
  { value: "food", label: "Food", description: "Tricks AI into seeing food", icon: UtensilsIcon },
  { value: "abstract", label: "Abstract", description: "Tricks AI into seeing abstract art", icon: ShapesIcon },
];

export const qualityPresets: {
  value: number;
  label: string;
  time: string;
  icon: typeof IconType;
  colour: string;
}[] = [
  {
    value: 0,
    label: "Faster",
    time: "~2-5 mins",
    icon: ZapIcon,
    colour: "text-emerald-500/70 dark:text-emerald-400/70",
  },
  {
    value: 25,
    label: "Fast",
    time: "~10-25 mins",
    icon: ZapIcon,
    colour: "text-teal-500/70 dark:text-teal-400/70",
  },
  {
    value: 50,
    label: "DEFAULT",
    time: "~20-50 mins",
    icon: GaugeIcon,
    colour: "text-sky-500/70 dark:text-sky-400/70",
  },
  {
    value: 75,
    label: "Slower",
    time: "~40-90 mins",
    icon: ClockIcon,
    colour: "text-amber-500/70 dark:text-amber-400/70",
  },
  {
    value: 100,
    label: "Slowest",
    time: "~60-150 mins",
    icon: ClockIcon,
    colour: "text-rose-500/70 dark:text-rose-400/70",
  },
];
