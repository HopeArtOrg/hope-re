---
name: tanstack-query
description: Create TanStack Svelte Query hooks following Hope:RE patterns with createQuery and createMutation
---

## TanStack Query Pattern

When creating query hooks in Hope:RE, follow these conventions:

### Query Hook (reading data)
```typescript
import type { MyDataType } from "./types";

import { createQuery } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

export function useMyData() {
  return createQuery(() => ({
    queryKey: ["my-data"],
    queryFn: async () => await invoke<MyDataType>("get_my_data"),
    staleTime: 30 * 1000,
    gcTime: 60 * 1000,
    retry: 1,
  }));
}
```

### Mutation Hook (writing data)
```typescript
import type { MyInput, MyResult } from "./types";

import { createMutation } from "@tanstack/svelte-query";
import { invoke } from "@tauri-apps/api/core";

export function useMyAction() {
  return createMutation(() => ({
    mutationFn: async (input: MyInput) =>
      await invoke<MyResult>("perform_action", { ...input }),
  }));
}
```

### Using in Stores (composable pattern)
```typescript
import { useMyData, useMyAction } from "$lib/queries";
import { toast } from "svelte-sonner";

export function useMyFeature() {
  const query = useMyData();
  const mutation = useMyAction();
  const isLoading = $derived(query.isPending || mutation.isPending);
  const data = $derived(query.data ?? null);

  async function handleAction(input: MyInput) {
    try {
      await mutation.mutateAsync(input);
      toast.success("Action completed");
    }
    catch (error) {
      toast.error("Action failed");
      console.error("Action error:", error);
    }
  }

  return {
    get data() { return data; },
    get isLoading() { return isLoading; },
    handleAction,
  };
}
```

### Rules
- Organize query files by domain in `src/lib/queries/` (e.g., `protection.ts`, `models.ts`)
- Re-export all hooks through `src/lib/queries/index.ts`
- Use `invoke<T>()` from `@tauri-apps/api/core` to call Rust backend
- Return types are defined with `type` (never `interface`)
- Stores compose queries/mutations and expose reactive state via getters
- Error handling: `try/catch` with `toast.error()` + `console.error()`
- No comments in code -- self-documenting through clear naming
- No emojis anywhere in the codebase
