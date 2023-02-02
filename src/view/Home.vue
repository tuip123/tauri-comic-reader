<template>
  <Test v-show="false"/>
  <n-layout vertical size="large">
    <n-layout-header style="height: 64px;padding: 12px">
      <Header @query="setSearchWord" noBack/>
    </n-layout-header>
    <n-layout-content :native-scrollbar="false" style="height: calc(100vh - 128px)">
      <n-scrollbar>
        <n-list style="margin-top:12px " bordered hoverable clickable show-divider>
          <template #header>
            <n-space justify="space-between" size="large">
              <div style="height: 24px; line-height: 24px;font-size: 20px;display: flex">
                <div style="width: 24px">
                  <n-icon size="24">
                    <Library/>
                  </n-icon>
                </div>
                <div style="width: 128px">
                  漫画库：
                </div>
              </div>
              <div>
                <n-button size="tiny" type="primary" circle @click="addLibrary">
                  <template #icon>
                    <n-icon>
                      <add/>
                    </n-icon>
                  </template>
                </n-button>
              </div>
            </n-space>
          </template>
          <n-list-item v-for="library in libraryList" :key="library.id" @click="toBookcase(library.id)">
            {{ library.root }}
            <template #prefix>
              <div style="display: flex">
                <n-icon>
                  <library-outline/>
                </n-icon>
              </div>
            </template>
            <template #suffix>
              <div style="display: flex" >
                <n-space justify="center" style="padding-right: 8px; ">
                  <n-button tertiary round type="primary" size="small" @click.stop="reloadLibrary(library.id)">
                    <template #icon>
                      <n-icon>
                        <reload-sharp/>
                      </n-icon>
                    </template>
                    刷新
                  </n-button>
                </n-space>
                <n-space justify="center">
                  <n-button tertiary round type="error" size="small" @click.stop="removeLibrary(library.id)">
                    移除
                    <template #icon>
                      <n-icon>
                        <remove-sharp/>
                      </n-icon>
                    </template>
                  </n-button>
                </n-space>
              </div>
            </template>
          </n-list-item>
        </n-list>
      </n-scrollbar>
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
  NIcon,
  NButton,
  NScrollbar,
  useMessage
} from "naive-ui"
import {Library, LibraryOutline, Add, RemoveSharp, ReloadSharp} from "@vicons/ionicons5";
import {useRouter} from "vue-router";
import {open} from "@tauri-apps/api/dialog";

const router = useRouter()
const message = useMessage()

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

async function removeLibrary(libraryId: number) {
  await invoke('delete_library', {id: libraryId})
  await queryLibrary()
}

async function addLibrary() {
  const selected = await open({
    multiple: true,
    directory: true
  }) as string[];
  for (let string of selected) {
    try {
      await invoke("add_library", {path: string})
      message.success('添加成功')
      await queryLibrary()
    } catch (err) {
      message.error(err as string)
    }
  }
}

async function reloadLibrary(libraryId: number) {
  console.log('123')
  await invoke("reload_library", {libraryId: libraryId})
  await queryLibrary()
}

onMounted(() => {
  queryLibrary()
})
</script>

<style scoped>
</style>
