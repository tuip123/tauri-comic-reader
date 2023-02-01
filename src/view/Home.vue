<!--todo 添加库 删除库-->
<template>
  <Test v-show="false"/>
  <n-layout vertical size="large">
    <n-layout-header style="height: 64px;padding: 12px">
      <Header @query="setSearchWord" noBack/>
    </n-layout-header>
    <n-layout-content :native-scrollbar="false" style="height: calc(100vh - 128px)">
      <n-list style="margin-top:12px " bordered hoverable clickable show-divider>
        <template #header>
          <n-space justify="center" size="large">
            <n-icon size="24">
              <Library/>
            </n-icon>
            漫画库：
          </n-space>
        </template>
        <template #footer>
          添加/编辑
        </template>
        <n-list-item v-for="library in libraryList" :key="library.id" @click="toBookcase(library.id)">
          {{ library.root }}
          <template #prefix>
            <n-icon>
              <library-outline/>
            </n-icon>
          </template>
          <template #suffix>
            <n-icon>
              <chevron-forward/>
            </n-icon>
          </template>
        </n-list-item>
      </n-list>
    </n-layout-content>
    <n-layout-footer
        style="height: 64px;padding: 24px">
      <n-space justify="center" size="large">
        <n-pagination
            v-model:page="pagination.current"
            v-model:page-size="pagination.size"
            :item-count="pagination.total"
            :page-sizes="[10, 20]"
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
import Test from "@/components/Test.vue";
import Header from "@/components/Header.vue";
import {invoke} from "@tauri-apps/api/tauri";
import {ref, onMounted} from "vue";
import {
  NPagination,
  NLayout,
  NLayoutHeader,
  NLayoutFooter,
  NLayoutContent,
  NSpace,
  NList,
  NListItem,
  NIcon
} from "naive-ui"
import {Library, LibraryOutline, ChevronForward} from "@vicons/ionicons5";
import {useRouter} from "vue-router";

const router = useRouter()

interface Pagination {
  current: number,
  size: number,
  total: number,
}

interface Library {
  id: number,
  root: string,
}

interface LibraryList {
  list: Library[],
  pagination: Pagination
}

const libraryList = ref<Library[]>([])
const pagination = ref<Pagination>({
  current: 1,
  size: 10,
  total: 1
})
const searchWord = ref("")

function setSearchWord(word: string) {
  searchWord.value = word
  queryLibrary()
}

async function queryLibrary() {
  let res = await invoke('query_library', {
    search: searchWord.value,
    page: pagination.value.current,
    pageSize: pagination.value.size
  }) as LibraryList
  pagination.value = res.pagination
  libraryList.value = res.list
}

async function pageChange(num: number) {
  pagination.value.current = num
  await queryLibrary()
}

async function sizeChange(num: number) {
  pagination.value.size = num
  await queryLibrary()
}

function toBookcase(libraryId: number) {
  router.push({path: '/ComicBookcase', query: {libraryId: libraryId}})
}

onMounted(() => {
  queryLibrary()
})
</script>

<style scoped>
</style>
