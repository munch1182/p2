<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { ref } from "vue";

const file = ref<string | undefined>();

async function change(e: any) {
  const val = e.target.checked;
  await getCurrentWindow().setAlwaysOnTop(val);
}
function changeFile(e: any) {
  recivice(e.target?.files);
  // 修复同一文件无法二次上传的问题
  e.target.value = null;
}
function dropFile(e: DragEvent) {
  recivice(e.dataTransfer?.files);
}

function recivice(files: FileList | undefined) {
  if (files) {
    file.value = files[0].name;
  }
}
</script>

<template>
  <main
    class="flex flex-1 flex-col items-center"
    @dragenter.prevent
    @dragover.prevent
    @drop.prevent="dropFile"
  >
    <label
      v-if="!file"
      id="dropzone-file-l"
      for="dropzone-file"
      @change="changeFile"
      class="mt-[20vh] flex cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 bg-gray-50 px-5 hover:bg-gray-100 dark:border-gray-600 dark:bg-gray-700 dark:hover:border-gray-500 dark:hover:bg-gray-600"
    >
      <div class="flex flex-col items-center justify-center pt-5 pb-6">
        <svg
          class="mb-4 h-8 w-8 text-gray-500 dark:text-gray-400"
          aria-hidden="true"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 20 16"
        >
          <path
            stroke="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"
          />
        </svg>
        <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
          <span class="font-semibold">Click to upload</span> or drag and drop
        </p>
      </div>
      <input id="dropzone-file" type="file" class="hidden" />
    </label>

    <label
      v-if="file"
      for="first_name"
      class="mt-[20vh] flex items-center text-sm font-medium text-gray-900 dark:text-white"
    >
      <a
        @click="file = undefined"
        href="#"
        class="ms-0 me-3 flex h-8 items-center justify-center rounded-s-lg border-e-0 border-gray-300 bg-white px-3 leading-tight text-gray-500 hover:bg-gray-100 hover:text-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
      >
        <svg
          class="h-2.5 w-2.5 rtl:rotate-180"
          aria-hidden="true"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 6 10"
        >
          <path
            stroke="currentColor"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M5 1 1 5l4 4"
          />
        </svg>
      </a>
      <input
        type="text"
        id="first_name"
        class="rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
        placeholder="ic_"
        required
      />
      <button
        type="button"
        class="ml-3 rounded-lg bg-blue-700 px-5 py-2.5 text-sm font-medium text-white hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 focus:outline-none dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
      >
        重命名
      </button>
    </label>

    <div class="mt-[10vh] hidden items-center sm:inline">
      <input
        type="checkbox"
        @change="change"
        class="h-4 w-4 rounded-sm border-gray-300 bg-gray-100 text-blue-600 focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-blue-600"
      />
      <label
        for="checked-checkbox"
        class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
        >置顶窗口</label
      >
    </div>
  </main>
</template>

<style scoped></style>
