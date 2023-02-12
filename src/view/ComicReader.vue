<template>
  <!--顶级 右侧-->
  <n-layout has-sider sider-placement="right" style="height: 100vh;">
    <n-layout-content :native-scrollbar="false">
      <!--次级 左侧 中区-->
      <n-layout has-sider style="height: 100vh;">
        <!--左侧侧边栏-->
        <n-layout-sider
            collapse-mode="width"
            :default-collapsed="false"
            :native-scrollbar="false"
            show-trigger="arrow-circle"
            :width="240"
            :collapsed-width="0"
            bordered
        >
          <div>
            <div style="padding: 24px 24px 0;">
              <n-form
                  label-placement="left"
                  label-width="auto"
                  size="small"
              >
                <n-form-item label="画面比例：">
                  <n-input-number button-placement="right" :max="100" :min="30" :step="10"
                                  v-model:value="comicWidth">
                    <template #suffix>
                      %
                    </template>
                  </n-input-number>
                </n-form-item>
              </n-form>
            </div>
            <div>
              <n-config-provider v-for="(comic,index) of libraryComics" :theme="comic.id === comicId?null:darkTheme">
                <n-card content-style="padding:4px" @click="changeComic(comic.id)" >
                  <n-space justify="space-between" :wrap="false">
                    <n-ellipsis line-clamp="2" style="height: 40px">
                      {{ comic.title }}
                    </n-ellipsis>
                    <n-button size="tiny" quaternary circle type="error" @click.stop="deleteComic(index)">
                      <template #icon>
                        <n-icon>
                          <trash-sharp/>
                        </n-icon>
                      </template>
                    </n-button>
                  </n-space>
                </n-card>
              </n-config-provider>
            </div>
          </div>
        </n-layout-sider>
        <!--中区图片区-->
        <n-layout-content :native-scrollbar="false" ref="center">
          <div class="top-hover-area">
            <n-button text class="back-button" @click="back">
              <template #icon>
                <n-icon size="30">
                  <ChevronBack/>
                </n-icon>
              </template>
            </n-button>
          </div>
          <div v-for="comic of comicPage" style="margin: auto;" :style="'width:'+comicWidth+'%'">
            <img style="width: 100%;" :src="comic" ref="comic" alt=""/>
          </div>
        </n-layout-content>
      </n-layout>
    </n-layout-content>
    <!--右侧侧边栏-->
    <n-layout-sider
        collapse-mode="width"
        :default-collapsed="false"
        :native-scrollbar="false"
        show-trigger="arrow-circle"
        :width="240"
        :collapsed-width="120"
        bordered>
      <n-scrollbar>
        <div style="padding:0 12px">
          <div v-for="(comic,i) of comicPage" :key="'right_'+comic">
            <img style="width: 100%;" :src="comic" @click="turnPage(i)" alt=""/>
          </div>
        </div>
      </n-scrollbar>
    </n-layout-sider>

  </n-layout>
</template>

<script setup lang="ts">
import {
  NButton,
  NIcon,
  NInputNumber,
  NForm,
  NFormItem,
  NScrollbar,
  NLayout,
  NLayoutContent,
  NLayoutSider,
  NEllipsis,
  NCard,
  NSpace,
  NConfigProvider,
  darkTheme,
  useMessage
} from 'naive-ui'
import {ChevronBack, TrashSharp} from "@vicons/ionicons5";
import {useRoute, useRouter} from "vue-router";
import {invoke, convertFileSrc} from "@tauri-apps/api/tauri";
import {onMounted, ref} from "vue";
import {useConfigStore} from "@/store/config";
const config = useConfigStore()

const router = useRouter()
const route = useRoute()
const message = useMessage()


interface Comic {
  id: number,
  count: number,
  library_id: number,
  cover: string,
  path: string,
  title: string,
}

const libraryComics = ref<Comic[]>([])
const comicPage = ref<string[]>([])
const comicId = ref(0)
const comic = ref()
const center = ref()
const comicWidth = ref(40)

function back() {
  router.go(-1)
}

function turnPage(i: number) {
  let scrollTop = comic.value[i].offsetTop
  center.value.scrollTo({top: scrollTop, behavior: 'smooth'})
}

async function changeComic(id: number) {
  comicId.value = id
  await initData()
}

async function deleteComic(index:number){
  let flag = false
  if (libraryComics.value[index].id === comicId.value){
    flag = true
  }
  await invoke('delete_comic', {id:libraryComics.value[index].id})
      .then(() => {
        message.success('已删除：' + libraryComics.value[index].title)
        if (config.delete_source_file){
          message.error('文件已被删除')
        }
      })
      .catch((err) => {
        message.error('删除错误：' + err as string)
      })

  libraryComics.value.splice(index,1)
  if (flag){
    comicPage.value = []
    if (libraryComics.value.length>0){
      let targetIndex = index>=libraryComics.value.length?index-1:index
      comicId.value = libraryComics.value[targetIndex].id
      await initData()
    }
  }
}

async function initData() {
  comicPage.value = []
  try {
    let res = <string[]>await invoke('read_comic', {id:comicId.value})
    res.forEach((e) => {
      comicPage.value.push(convertFileSrc(e))
    })
  } catch (e) {
    router.go(-1)
    message.error('文件无法打开')
  }
}

async function initLibrary(libraryId: number) {
  libraryComics.value = <Comic[]>await invoke('query_comic_name', {libraryId})
}

onMounted(async () => {
  comicId.value = Number(route.query.id)
  await initData()
  await initLibrary(Number(route.query.libraryId))

})


</script>

<style scoped>
.top-hover-area {
  height: 60px;
  width: 60px;
  position: fixed;
  top: 0;
  z-index: 100;
  display: flex;
  align-items: center;
}

.back-button {
  margin-left: 20px;
  transition: visibility 0s, opacity 0.1s linear;
  -webkit-transition: visibility 0s, opacity 0.1s linear; /* Safari */
}
</style>
