<template>
  <n-button @click="back">back</n-button>

</template>

<script setup lang="ts">
import {NButton} from 'naive-ui'
import {useRoute, useRouter} from "vue-router";
import {invoke} from "@tauri-apps/api/tauri";
import {onMounted,ref} from "vue";

const router = useRouter()
const route = useRoute()

const comicList = ref<string[]>([])
function back() {
  router.go(-1)
}

onMounted(async () => {
  console.log(route.query.id)
  try {
    let res = <any>await invoke('read_comic', {id: Number(route.query.id)})
    console.log(res)
    comicList.value = res.page as string[]
    console.log(comicList.value)
  } catch (e) {
    console.error(e)
  }
})
</script>

<style scoped>

</style>
