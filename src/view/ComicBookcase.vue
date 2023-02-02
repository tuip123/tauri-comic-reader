<template>
  <n-layout vertical size="large">
    <n-layout-header style="height: 64px;padding: 12px">
      <Header @query="setSearchWord"/>
    </n-layout-header>
    <n-layout-content style="height: calc(100vh - 128px);background-color: #2b2b2b">
      <n-scrollbar ref="scrollbar">
          <n-grid cols="2 s:3 m:4 l:6 xl:8 2xl:10" responsive="screen">
            <n-grid-item v-for="comic in comicList" :key="comic.id" style="padding: 12px">
              <ComicItem :comic="comic"/>
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
            :page-sizes="[9, 18, 27]"
            :page-slot="5"
            show-size-picker
            :on-update:page="pageChange"
            :on-update:page-size="sizeChange"
        />
      </n-space>
    </n-layout-footer>
  </n-layout>
</template>
<!--todo 展示、删除漫画-->
<script setup lang="ts">
import {
  NLayout,
  NLayoutHeader,
  NLayoutContent,
  NLayoutFooter,
  NSpace,
  NPagination,
  NGrid,
  NGridItem,
  NScrollbar
} from "naive-ui"
import Header from "@/components/Header.vue";
import ComicItem from "@/components/ComicItem.vue";
import {useRoute, useRouter} from "vue-router";
import {ref, onMounted} from "vue";
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
  size: 9,
  total: 1
})
const comicList = ref<Comic[]>([])
const scrollbar = ref()
const router = useRouter()
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
  scrollbar.value.scrollTo({top:0})
}

async function pageChange(num: number) {
  pagination.value.current = num
  await queryComic()
}

async function sizeChange(num: number) {
  pagination.value.size = num
  await queryComic()
}

onMounted(async () => {
  await queryComic()
})
</script>

<style scoped>

</style>
