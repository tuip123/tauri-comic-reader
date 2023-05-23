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
            <!--            比例区-->
            <div style="padding: 24px 24px 0;">
              <n-form
                  label-placement="left"
                  label-width="auto"
                  size="small"
              >
                <n-form-item label="画面比例：">
                  <n-input-number button-placement="right" :max="100" :min="30" :step="10"
                                  v-model:value="comicWidth" :on-update:value="changeWidth">
                    <template #suffix>
                      %
                    </template>
                  </n-input-number>
                </n-form-item>
              </n-form>
            </div>
            <!--            页面类型选择区-->
            <div>
              <n-space justify="space-around">
                <n-radio-group v-model:value="readType" size="small" :on-update:value="changeReadType">
                  <n-radio-button
                      v-for="type in types"
                      :key="type.value"
                      :value="type.value"
                      :label="type.label"
                  />
                </n-radio-group>
              </n-space>
            </div>
            <!--            书架-->
            <div>
              <n-config-provider v-for="(comic,index) of libraryComics" :theme="comic.id === comicId?null:darkTheme">
                <n-card content-style="padding:4px" @click="changeComic(comic.id)">
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
        <n-layout-content>

          <div class="top-hover-area">
            <n-button text class="back-button" @click="back">
              <template #icon>
                <n-icon size="30">
                  <ChevronBack/>
                </n-icon>
              </template>
            </n-button>
          </div>

          <n-scrollbar v-if="readType === 0" ref="type0" :on-scroll="type0scroll">
            <div v-for="comic of comicPage" style="margin: auto;" :style="'width:'+comicWidth+'%'">
              <img style="width: 100%;" :src="comic" ref="comic" alt=""/>
            </div>
          </n-scrollbar>

          <n-scrollbar v-if="readType === 1">
            <div style="margin: auto;" :style="'width:'+comicWidth+'%'">
              <img style="width: 100%;" :src="comicPage[handlePageId]" alt=""/>
            </div>
          </n-scrollbar>

          <n-scrollbar v-if="readType === 2">
            <div style="margin: auto;" :style="'width:'+comicWidth+'%'">
              <n-space justify="space-around" :wrap="false" v-if="handlePageId%2===0">
                <img style="width: 100%;" :src="comicPage[handlePageId]" alt=""/>
                <img style="width: 100%;" :src="comicPage[handlePageId+1]" alt=""
                     v-if="handlePageId+1<comicPage.length"/>
              </n-space>
              <n-space justify="space-around" :wrap="false" v-else>
                <img style="width: 100%;" :src="comicPage[handlePageId-1]" alt=""/>
                <img style="width: 100%;" :src="comicPage[handlePageId]" alt=""/>
              </n-space>
            </div>
          </n-scrollbar>

          <n-scrollbar v-if="readType === 3">
            <div style="margin: auto;" :style="'width:'+comicWidth+'%'">
              <n-space justify="space-around" :wrap="false" v-if="handlePageId%2===0">
                <img style="width: 100%;" :src="comicPage[handlePageId+1]" alt=""
                     v-if="handlePageId+1<comicPage.length"/>
                <img style="width: 100%;" :src="comicPage[handlePageId]" alt=""/>
              </n-space>
              <n-space justify="space-around" :wrap="false" v-else>
                <img style="width: 100%;" :src="comicPage[handlePageId]" alt=""/>
                <img style="width: 100%;" :src="comicPage[handlePageId-1]" alt=""/>
              </n-space>
            </div>
          </n-scrollbar>


        </n-layout-content>
      </n-layout>
    </n-layout-content>
    <!--右侧侧边栏-->
    <n-layout-sider
        collapse-mode="width"
        :default-collapsed="false"
        show-trigger="arrow-circle"
        :width="240"
        :collapsed-width="120"
        bordered>
      <n-scrollbar ref="rightSider">
        <div style="padding:0 12px">
          <div v-for="(comic,i) of comicPage" :key="'right_'+comic">
            <img ref="rightSiderImage" style="width: 100%;" :src="comic" @click="turnPage(i)"
                 :style="i===handlePageId?'border-radius: 100px':''"/>
          </div>
        </div>
      </n-scrollbar>
    </n-layout-sider>

  </n-layout>
</template>

<script setup lang="ts">
import {
  darkTheme,
  NButton,
  NCard,
  NConfigProvider,
  NEllipsis,
  NForm,
  NFormItem,
  NIcon,
  NInputNumber,
  NLayout,
  NLayoutContent,
  NLayoutSider,
  NRadioButton,
  NRadioGroup,
  NScrollbar,
  NSpace,
  useMessage
} from 'naive-ui'
import {ChevronBack, TrashSharp} from "@vicons/ionicons5";
import {useRoute, useRouter} from "vue-router";
import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";
import {onMounted, onUnmounted, ref, watch} from "vue";
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
const type0 = ref()
const rightSider = ref()
const rightSiderImage = ref()
const stopRightScroll = ref(false)

const comicWidth = ref(40)
const readType = ref(0)

const types = ref([
  {value: 0, label: "滚动"},
  {value: 1, label: "单页"},
  {value: 2, label: "左右"},
  {value: 3, label: "右左"}
])

const handlePageId = ref(0)

watch(
    () => handlePageId.value,
    (id) => {
      if (!stopRightScroll.value) {

        let scrollTop = rightSiderImage.value[id].offsetTop
        rightSider.value.scrollTo({top: scrollTop, behavior: 'smooth'})
      }

    }
)
watch(
    () => readType.value,
    (type) => {
      if (type === 0) {
        setTimeout(test, 1)
      }
    }
)

function test() {
  let scrollTop = comic.value[handlePageId.value].offsetTop
  type0.value.scrollTo({top: scrollTop, behavior: 'smooth'})
}

function back() {
  router.go(-1)
}

function turnPage(i: number) {
  stopRightScroll.value = true

  if (readType.value === 0) {
    let scrollTop = comic.value[i].offsetTop
    type0.value.scrollTo({top: scrollTop, behavior: 'smooth'})
  } else {
    handlePageId.value = i
  }

  setTimeout(function () {
    stopRightScroll.value = false
  }, 1000)
}

function type0scroll(event: any) {
  let scrollTop = event.target.scrollTop
  for (let i = 0; i < comic.value.length; i++) {
    let c = comic.value[i]
    if ((c.offsetTop + (c.height * 0.8)) >= scrollTop) {
      handlePageId.value = i
      break
    }
  }
}

function prevPage() {
  if (handlePageId.value > 0)
    handlePageId.value -= 1
}

function nextPage() {
  if (handlePageId.value < comicPage.value.length - 1)
    handlePageId.value += 1
}

function widthLarger() {
  if (comicWidth.value < 100) {
    comicWidth.value += 10
  }
}

function widthSmaller() {
  if (comicWidth.value > 30) {
    comicWidth.value -= 10
  }
}

async function changeComic(id: number) {
  comicId.value = id
  handlePageId.value = 0
  await initData()
}

function changeWidth(width: number | null) {
  let w = width as number
  comicWidth.value = w
  invoke("update_config", {key: 'comic_width', value: w.toString()})
}


function changeReadType(type: number) {
  readType.value = type
  invoke("update_config", {key: 'read_type', value: type.toString()})

}

async function deleteComic(index: number) {
  let flag = false
  if (libraryComics.value[index].id === comicId.value) {
    flag = true
  }
  await invoke('delete_comic', {id: libraryComics.value[index].id})
      .then(() => {
        message.success('已删除：' + libraryComics.value[index].title)
        if (config.delete_source_file) {
          message.error('文件已被删除')
        }
      })
      .catch((err) => {
        message.error('删除错误：' + err as string)
      })

  libraryComics.value.splice(index, 1)
  if (flag) {
    comicPage.value = []
    if (libraryComics.value.length > 0) {
      let targetIndex = index >= libraryComics.value.length ? index - 1 : index
      comicId.value = libraryComics.value[targetIndex].id
      await initData()
    }
  }
}

async function initConfig() {
  comicWidth.value = Number(config.comic_width)
  readType.value = Number(config.read_type)
}


async function initData() {
  comicPage.value = []
  try {
    let res = <string[]>await invoke('read_comic', {id: comicId.value})
    res.forEach((e) => {
      if (e === "") {
        comicPage.value.push(e)
      } else {
        comicPage.value.push(convertFileSrc(e))
      }
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
  await initConfig()
  await initData()
  await initLibrary(Number(route.query.libraryId))
  document.addEventListener('keyup', keyup)
  document.addEventListener('mouseup', mouseup)
})

onUnmounted(() => {
  document.removeEventListener('keyup', keyup)
  document.removeEventListener('mouseup', mouseup)
})

const keyboardHotkey = {
  'prev': '[',
  'next': ']',
  'larger': '+',
  'smaller': '-'
}
const mouseHotkey = {
  'prev': 4,
  'next': 3
}


function keyup(event: KeyboardEvent) {
  if (event.key === keyboardHotkey.next) {
    nextPage()
  } else if (event.key === keyboardHotkey.prev) {
    prevPage()
  } else if (event.key === keyboardHotkey.larger) {
    widthLarger()
  } else if (event.key === keyboardHotkey.smaller) {
    widthSmaller()
  }
}

function mouseup(event: MouseEvent) {
  if (event.button === mouseHotkey.next) {
    nextPage()
  } else if (event.button === mouseHotkey.prev) {
    prevPage()
  }
}

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
