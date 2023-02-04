<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {open} from '@tauri-apps/api/dialog';
import {ref} from 'vue';
import {NButton,NInput} from 'naive-ui';


import {useRouter} from "vue-router";

const router = useRouter()

interface Pagination {
  current: number,
  size: number,
  total: number,
}

interface Comic {
  id: number,
  count: number,
  library_id: number,
  cover: string,
  path: string,
  title: string,
}

interface ComicList {
  list: Comic[],
  pagination: Pagination
}

interface Library {
  id: number,
  root: string,
}

interface LibraryList {
  list: Library[],
  pagination: Pagination
}

async function addLibrary() {
  const selected = await open({
    multiple: true,
    directory: true
  }) as string[];
  for (let string of selected) {
    try {
      await invoke("add_library", {path: string})
      console.log('添加成功')
    } catch (e) {
      console.log(e)
    }
  }
}

const temp = ref(1)

async function reloadLibrary() {
  try {
    await invoke("reload_library", {libraryId: Number(temp.value)})
  } catch (e) {
    console.error(e)
  }
}

async function addThirdPartyImageViewer() {
  const selected = await open({
    multiple: false,
    directory: false,
    filters: [{
      name: '可执行程序',
      extensions: ['exe']
    }, {
      name: '所有文件',
      extensions: ['*']
    }]
  }) as string;
  await invoke("update_config", {key: 'third_party_image_viewer', value: selected})
}

async function openWithThirdParty() {
  try {
    await invoke('open_with_third_party', {folder: "D:\\56"})
  } catch (e) {
    console.error(e)
  }
}


async function queryLibrary() {
  let res = await invoke('query_library', {search: "", page: Number(temp.value), pageSize: 10}) as LibraryList
  console.log(res)
}

async function queryComic() {
  let res = await invoke('query_comic', {search: "", libraryId: Number(temp.value), page: 1, pageSize: 10}) as ComicList
  console.log(res)
}

async function deleteComic() {
  await invoke('delete_comic', {id: Number(temp.value)})
}

async function readComic() {
  try {
    let res = await invoke('read_comic', {id: Number(temp.value)})
    console.log(res)
  } catch (e) {
    console.error(e)
  }
}

function routerFn() {
  router.push({
    path: '/ComicBookcase',
    query: {
      id: temp.value
    }
  })
}

const img = ref()
const emit = defineEmits(['close'])

function closePanel() {
  emit('close')
}

function setDeleteFileIsTrue(value: boolean) {
  invoke("update_config", {key: 'delete_source_file', value: value.toString()})
}
</script>

<template>
  <div class="card">
    <n-input v-model="temp" />
    <br>
    <n-button round @click="addLibrary()">addLibrary</n-button>
    <br>
    <n-button round @click="reloadLibrary()">reloadLibrary</n-button>
    <br>
    <n-button round @click="addThirdPartyImageViewer()">addThirdPartyImageViewer</n-button>
    <br>
    <n-button round @click="openWithThirdParty()">openWithThirdParty</n-button>
    <br>
    <n-button round @click="queryLibrary()">queryLibrary</n-button>
    <br>
    <n-button round @click="queryComic()">queryComic</n-button>
    <br>
    <n-button round @click="deleteComic()">deleteComic</n-button>
    <br>
    <n-button round type="error" @click="setDeleteFileIsTrue(true)">setDeleteFileIsTrue</n-button>
    <br>
    <n-button round type="error" @click="setDeleteFileIsTrue(false)">setDeleteFileIsFalse</n-button>
    <br>
    <n-button round @click="readComic()">readComic</n-button>
    <br>
    <n-button round @click="routerFn()">router</n-button>
  </div>
</template>
