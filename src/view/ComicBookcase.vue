<template>
  <n-layout vertical size="large">
    <n-layout-header style="height: 64px;padding: 12px">
      <Header @query="setSearchWord"/>
    </n-layout-header>
    <n-layout-content style="height: calc(100vh - 128px)">
      <n-scrollbar ref="scrollbar">
        <n-grid cols="2 s:2 m:3 l:6 xl:6 2xl:8" responsive="screen">
          <n-grid-item v-for="comic in comicList" :key="comic.id" style="padding: 6px">
            <ComicItem :comic="comic" @delete="deleteItem"/>
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
import {onMounted, onUnmounted, ref} from "vue";
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

async function deleteItem() {
  pagination.value.total -= 1
  await queryComic()
}

async function queryComic() {
  if (pagination.value.current > (pagination.value.total / pagination.value.size)) {
    pagination.value.current = Math.ceil(pagination.value.total / pagination.value.size)
  }
  let res = await invoke('query_comic', {
    search: searchWord.value,
    page: pagination.value.current,
    pageSize: pagination.value.size,
    libraryId: Number(libraryId)
  }) as ComicList
  pagination.value = res.pagination
  comicList.value = res.list
}

async function pageChange(num: number) {
  pagination.value.current = num
  await queryComic()
  scrollbar.value.scrollTo({top: 0})
}

async function sizeChange(num: number) {
  pagination.value.size = num
  await queryComic()
}

onMounted(async () => {
  document.addEventListener('keyup', keyup)
  document.addEventListener('mouseup', mouseup)
  await queryComic()
})

onUnmounted(() => {
  document.removeEventListener('keyup', keyup)
  document.removeEventListener('mouseup', mouseup)
})

const keyboardHotkey = {
  'prev': 'ArrowLeft',
  'next': 'ArrowRight',
  'more': '+',
  'less': '-'
}
const mouseHotkey = {
  'prev': 4,
  'next': 3
}


function mouseup(event: MouseEvent) {
  if (event.button === mouseHotkey.prev) {
    if (pagination.value.current > 1) {
      pageChange(pagination.value.current - 1)
    }
  }
  if (event.button === mouseHotkey.next) {
    if (pagination.value.current < (pagination.value.total / pagination.value.size)) {
      pageChange(pagination.value.current + 1)
    }
  }
}

function keyup(event: KeyboardEvent) {
  if (event.key === keyboardHotkey.prev) {
    if (pagination.value.current > 1) {
      pageChange(pagination.value.current - 1)
    }
    event.preventDefault()
  } else if (event.key === keyboardHotkey.next) {
    if (pagination.value.current < (pagination.value.total / pagination.value.size)) {
      pageChange(pagination.value.current + 1)
    }
    event.preventDefault()
  } else if (event.key === keyboardHotkey.more) {
    sizeChange(pagination.value.size + 6)
  } else if (event.key === keyboardHotkey.less) {
    if (pagination.value.size > 6) {
      sizeChange(pagination.value.size - 6)
    }
  }
}

</script>

<style scoped>

</style>
