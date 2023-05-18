<template>
  <n-layout vertical size="large" @mousedown.stop="mouseEvent">
    <n-layout-header style="height: 64px;padding: 12px">
      <Header @query="setSearchWord"/>
    </n-layout-header>
    <n-layout-content style="height: calc(100vh - 128px)">
      <n-scrollbar ref="scrollbar">
        <n-grid cols="2 s:2 m:3 l:6 xl:6 2xl:8" responsive="screen">
          <n-grid-item v-for="comic in comicList" :key="comic.id" style="padding: 6px">
            <ComicItem :comic="comic" @delete="queryComic"/>
          </n-grid-item>
        </n-grid>
      </n-scrollbar>
    </n-layout-content>
    <n-layout-footer
        style="height: 64px;padding: 24px">
      <n-space justify="center" size="large">
        <n-pagination
            v-model:page="pagination.current"
            v-model:page-size="pagination.size"
            :item-count="pagination.total"
            :page-sizes="[6, 12, 18, 24]"
            :page-slot="5"
            show-size-picker
            :on-update:page="pageChange"
            :on-update:page-size="sizeChange"
        />
      </n-space>
    </n-layout-footer>
  </n-layout>
</template>
<script setup lang="ts">
import {
  NGrid,
  NGridItem,
  NLayout,
  NLayoutContent,
  NLayoutFooter,
  NLayoutHeader,
  NPagination,
  NScrollbar,
  NSpace
} from "naive-ui"
import Header from "@/components/Header.vue";
import ComicItem from "@/components/ComicItem.vue";
import {useRoute} from "vue-router";
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

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

const pagination = ref<Pagination>({
  current: 1,
  size: 12,
  total: 1
})
const comicList = ref<Comic[]>([])
const scrollbar = ref()
const route = useRoute()
const libraryId = route.query.libraryId

const searchWord = ref("")

function setSearchWord(word: string) {
  searchWord.value = word
  queryComic()
}

async function queryComic() {
  let res = await invoke('query_comic', {
    search: searchWord.value,
    page: pagination.value.current,
    pageSize: pagination.value.size,
    libraryId: Number(libraryId)
  }) as ComicList
  pagination.value = res.pagination
  comicList.value = res.list
  scrollbar.value.scrollTo({top: 0})
}

async function pageChange(num: number) {
  pagination.value.current = num
  await queryComic()
}

async function sizeChange(num: number) {
  pagination.value.size = num
  await queryComic()
}

function mouseEvent(event: any) {
  console.log(event.button)
}

onMounted(async () => {
  await queryComic()
})
</script>

<style scoped>

</style>
