<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {open} from '@tauri-apps/api/dialog';

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", {name: name.value});
}

async function addLibrary() {
  const selected = await open({
    multiple: true,
    directory: true
  }) as string[];
  for (let string of selected) {
    let b = await invoke("add_library", {path: string})
    if (!b) console.log('已经添加过了')
    else console.log('添加成功')
  }
}

</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..."/>
    <button type="button" @click="greet()">Greet</button>
    <button type="button" @click="addLibrary()">addLibrary</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
