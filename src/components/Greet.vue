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

async function reloadLibrary() {
  await invoke("reload_library", {libraryId: Number(temp.value)})
}

async function addThirdPartyImageViewer() {
  const selected = await open({
    multiple: false,
    directory: false,
    filters: [{
      name: '可执行程序',
      extensions: ['exe']
    },{
      name: '所有文件',
      extensions: ['*']
    }]
  }) as string;
  await invoke("add_third_party_image_viewer", {path: selected})
}

async function openWithThirdParty(){
  await invoke('open_with_third_party',{folder:""})
}

async function queryLibrary(){
  let res = await invoke('query_library',{search:"",page:1,pageSize:10})
  console.log(res)
}


</script>

<template>
  <div class="card">
    <input v-model="temp">
    <br>
    <button type="button" @click="addLibrary()">addLibrary</button>
    <br>
    <button type="button" @click="reloadLibrary()">reloadLibrary</button>
    <br>
    <button type="button" @click="addThirdPartyImageViewer()">addThirdPartyImageViewer</button>
    <br>
    <button type="button" @click="openWithThirdParty()">openWithThirdParty</button>
    <br>
    <button type="button" @click="queryLibrary()">queryLibrary</button>
  </div>
</template>
