<template>
  <n-layout vertical size="large" @click="settingActive = false">
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
          <n-list-item v-for="library in libraryList" :key="library.id"
                       @dblclick.stop="toBookcase(library.id,library.root)">
            <div style="display: flex;justify-content: space-between;">
              <div>
                {{ library.root }}
              </div>
              <div>
                {{ library.count + '本' }}
              </div>
            </div>
            <template #prefix>
              <div style="display: flex;align-items: center;">
                <n-icon>
                  <library-outline/>
                </n-icon>
              </div>
            </template>
            <template #suffix>
              <div style="display: flex" :id="'library_item_'+library.id">
                <n-space justify="center" style="padding-right: 8px; ">
                  <n-button secondary round type="primary" size="small"
                            @click.stop="openSetting(library)">
                    <template #icon>
                      <n-icon>
                        <settings-outline/>
                      </n-icon>
                    </template>
                    设置
                  </n-button>
                </n-space>
                <n-space justify="center" style="padding-right: 8px; ">
                  <n-button round type="info" size="small" @click.stop="listItemRead(library)">
                    <template #icon>
                      <n-icon>
                        <eye-outline/>
                      </n-icon>
                    </template>
                    阅读
                  </n-button>
                </n-space>
                <n-space justify="center" style="padding-right: 8px; ">
                  <n-button tertiary round type="primary" size="small"
                            @click.stop="toBookcase(library.id,library.root)">
                    <template #icon>
                      <n-icon>
                        <folder-open-outline/>
                      </n-icon>
                    </template>
                    列表
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

  <n-drawer
      v-model:show="settingActive"
      :width="250"
      :trap-focus="false"
      :block-scroll="false"
      :to="drawerId"
  >
    <div
        style="display: flex;justify-content: space-between;align-items: center;width: 100%;height: 100%"
        @click.stop="settingActive = false"
    >
      <n-space justify="center" style="padding-left: 8px">
        <n-switch @click.stop :value="settingLibrary.random_mode === 1" @update:value="changeRandomMode">
          <template #checked>
            随机阅读
          </template>
          <template #unchecked>
            顺序阅读
          </template>
        </n-switch>
      </n-space>

      <n-space justify="center" style="padding-top: 4px;padding-right: 8px">
        <n-button tertiary round type="error" size="tiny"
                  @click.stop="removeLibraryInSetting">
          移除
          <template #icon>
            <n-icon>
              <remove-sharp/>
            </n-icon>
          </template>
        </n-button>
      </n-space>
    </div>
  </n-drawer>

</template>
<script setup lang="ts">
import Header from "@/components/Header.vue";
import {invoke} from "@tauri-apps/api/tauri";
import {nextTick, onMounted, reactive, ref} from "vue";
import {
  NButton,
  NDrawer,
  NIcon,
  NLayout,
  NLayoutContent,
  NLayoutFooter,
  NLayoutHeader,
  NList,
  NListItem,
  NPagination,
  NScrollbar,
  NSpace,
  NSwitch,
  useMessage
} from "naive-ui"
import {
  Add,
  EyeOutline,
  FolderOpenOutline,
  Library,
  LibraryOutline,
  RemoveSharp,
  SettingsOutline
} from "@vicons/ionicons5";
import {useRouter} from "vue-router";
import {open} from "@tauri-apps/api/dialog";
import {useConfigStore} from "@/store/config";

const router = useRouter()
const message = useMessage()
const config = useConfigStore()


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
  count: number,
  random_mode: number
}

interface LibraryPage {
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
  }) as LibraryPage
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

async function toBookcase(libraryId: number, libraryRoot: string) {
  if (settingActive.value === true) {
    return
  }
  await reloadLibrary(libraryId, libraryRoot)
  await router.push({path: '/ComicBookcase', query: {libraryId: libraryId}})
}

async function removeLibrary(libraryId: number, libraryRoot: string) {
  await invoke('delete_library', {id: libraryId})
  await queryLibrary()
  message.error('已经删除 ' + libraryRoot)
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

async function reloadLibrary(libraryId: number, libraryRoot: string) {
  message.info('重新加载 ' + libraryRoot)
  await invoke("reload_library", {libraryId: libraryId})
  await queryLibrary()
  message.info('加载完成 ' + libraryRoot)
}

function read(comicId: number, libraryId: number) {
  router.push({path: '/ComicReader', query: {id: comicId, libraryId: libraryId}})
}

async function listItemRead(item: Library) {
  let libraryId = item.id
  let page = 1
  if (item.random_mode === 0) {
    page = 1
  }
  if (item.random_mode === 1) {
    page = Math.floor(Math.random() * (item.count - 0)) + 0
  }
  let res = await invoke('query_comic', {
    search: "",
    libraryId: libraryId,
    page: page,
    pageSize: 1
  }) as ComicList
  let comicId = res.list[0].id
  read(comicId, libraryId)
}


const drawerId = ref("")
const settingActive = ref(false)
let settingLibrary = reactive(<Library>{})

async function openSetting(library: Library) {
  settingLibrary = library
  if (settingActive.value) {
    settingActive.value = false
    await nextTick()
  }
  drawerId.value = '#library_item_' + library.id
  settingActive.value = true
}

async function removeLibraryInSetting() {
  let {root, id} = settingLibrary
  await invoke('delete_library', {id: id})
  await queryLibrary()
  message.error('已经删除 ' + root)
}

async function changeRandomMode() {
  let {id, random_mode} = settingLibrary
  random_mode = (random_mode + 1) % 2
  await invoke('set_library_random', {id: id, mode: random_mode})
  await queryLibrary()
  settingLibrary.random_mode = random_mode
}

onMounted(async () => {
  await queryLibrary()
})


</script>

<style scoped>
</style>
