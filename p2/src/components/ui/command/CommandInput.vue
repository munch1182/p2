<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { Search } from 'lucide-vue-next'
import { ListboxFilter, type ListboxFilterProps, useForwardProps } from 'reka-ui'
import { cn } from '@/lib/utils'
import { useCommand } from '.'

defineOptions({
  inheritAttrs: false,
})

const props = defineProps<ListboxFilterProps & {
  class?: HTMLAttributes['class']
}>()

const delegatedProps = reactiveOmit(props, 'class')

const forwardedProps = useForwardProps(delegatedProps)

const { filterState } = useCommand()
</script>

<template>
  <div data-slot="command-input-wrapper" class="flex h-search items-center gap-2 border-b px-3">
    <Search class="size-6 shrink-0 opacity-50" />
    <ListboxFilter v-bind="{ ...forwardedProps, ...$attrs }" v-model="filterState.search" data-slot="command-input"
      auto-focus
      :class="cn('placeholder:text-muted-foreground flex h-search w-full rounded-md bg-transparent py-3 text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50', props.class)" />
  </div>
</template>
