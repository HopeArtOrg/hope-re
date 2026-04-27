import type { ProtectionMenuProps } from "./components";
import type { Icon as IconType } from "@lucide/svelte";

import { ClockIcon, GaugeIcon, ZapIcon } from "@lucide/svelte";

export const algorithms: {
  value: Exclude<ProtectionMenuProps["algorithm"], undefined>;
  label: string;
  description: string;
  colour: string;
  bgColour: string;
}[] = [
  {
    value: "noise",
    label: "Noise",
    description: "Add adversarial noise",
    colour: "text-indigo-600 dark:text-indigo-400",
    bgColour: "bg-indigo-500/10",
  },
  {
    value: "glaze",
    label: "Glaze",
    description: "Style transfer protection",
    colour: "text-violet-600 dark:text-violet-400",
    bgColour: "bg-violet-500/10",
  },
  {
    value: "nightshade",
    label: "Nightshade",
    description: "Data poisoning protection",
    colour: "text-rose-600 dark:text-rose-400",
    bgColour: "bg-rose-500/10",
  },
];

export const glazeStyles: {
  value: Exclude<ProtectionMenuProps["glazeStyle"], undefined>;
  label: string;
  description: string;
}[] = [
  { value: "abstract", label: "Abstract", description: "Chaotic brushstrokes" },
  { value: "impressionist", label: "Impressionist", description: "Gentle light" },
  { value: "cubist", label: "Cubist", description: "Fragmented forms" },
  { value: "sketch", label: "Sketch", description: "Loose pencil lines" },
  { value: "watercolor", label: "Watercolor", description: "Flowing colors" },
];

export const nightshadeTargets: {
  value: Exclude<ProtectionMenuProps["nightshadeTarget"], undefined>;
  label: string;
  description: string;
}[] = [
  { value: "dog", label: "Dog", description: "Tricks AI into seeing a dog" },
  { value: "cat", label: "Cat", description: "Tricks AI into seeing a cat" },
  { value: "car", label: "Car", description: "Tricks AI into seeing a car" },
  { value: "landscape", label: "Landscape", description: "Tricks AI into seeing a landscape" },
  { value: "person", label: "Person", description: "Tricks AI into seeing a person" },
  { value: "building", label: "Building", description: "Tricks AI into seeing a building" },
  { value: "food", label: "Food", description: "Tricks AI into seeing food" },
  { value: "abstract", label: "Abstract", description: "Tricks AI into seeing abstract art" },
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
    colour: "text-emerald-600 dark:text-emerald-400",
  },
  {
    value: 25,
    label: "Fast",
    time: "~10-25 mins",
    icon: ZapIcon,
    colour: "text-teal-600 dark:text-teal-400",
  },
  {
    value: 50,
    label: "DEFAULT",
    time: "~20-50 mins",
    icon: GaugeIcon,
    colour: "text-sky-600 dark:text-sky-400",
  },
  {
    value: 75,
    label: "Slower",
    time: "~40-90 mins",
    icon: ClockIcon,
    colour: "text-amber-600 dark:text-amber-400",
  },
  {
    value: 100,
    label: "Slowest",
    time: "~60-150 mins",
    icon: ClockIcon,
    colour: "text-rose-600 dark:text-rose-400",
  },
];
