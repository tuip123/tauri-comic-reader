<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {open} from '@tauri-apps/api/dialog';
import {ref} from 'vue'
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

const temp = ref(1)
async function reloadLibrary(){
  let b = await invoke("reload_library",{libraryId:Number(temp.value)})
  console.log(b)
}

</script>

<template>
  <div class="card">
    <input v-model="temp">
    <button type="button" @click="addLibrary()">addLibrary</button>
    <button type="button" @click="reloadLibrary()">reloadLibrary</button>
  </div>
</template>
