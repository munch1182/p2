<script setup lang="ts">
import { Command as CommandVue, CommandInput } from "@/components/ui/command";
import { Window } from "@tauri-apps/api/window";

async function drag() {
  await Window.getCurrent().startDragging();
}
</script>

<template>
  <main @mousedown.prevent="drag">
    <CommandVue>
      <CommandInput :class="'text-2xl'" />
    </CommandVue>
  </main>
</template>

<style scoped></style>
