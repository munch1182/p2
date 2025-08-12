<script setup lang="ts">
// https://www.shadcn-vue.com/docs/components/command.html
import {
  Command as CommandVue,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { Window } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";
import { Command } from "./command";

async function drag() {
  await Window.getCurrent().startDragging();
}

let items = ref();

onMounted(async () => {
  items.value = await Command.getDefaultsCommand();
});
</script>

<template>
  <main @mousedown.prevent="drag">
    <CommandVue class="h-[320px]">
      <CommandInput :class="'text-2xl h-[56px]'" />
      <CommandList class="h-[262px]">
        <CommandEmpty>Nothing found.</CommandEmpty>
        <CommandGroup>
          <CommandItem
            v-for="item in items"
            key="{{ item.id }}"
            value="{{ item.value }}"
            class="data-[highlighted]:bg-black/20"
          >
            {{ item.name }}
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </CommandVue>
  </main>
</template>

<style scoped></style>
