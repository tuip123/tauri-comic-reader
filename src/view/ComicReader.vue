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
            content-style="padding:24px"
            bordered
        >
          <div>
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
            <div style="height: 500vh">left</div>
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
          <div v-for="comic of comicList" style="margin: auto;" :style="'width:'+comicWidth+'%'" :key="'center_'+comic">
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
        content-style="padding:24px"
        bordered>
      <n-grid :cols="1">
        <n-gi v-for="(comic,i) of comicList" :key="'right_'+comic">
          <img style="width: 100%;" :src="comic" @click="turnPage(i)" alt=""/>
        </n-gi>
      </n-grid>
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
  NLayout,
  NLayoutContent,
  NLayoutSider,
  NGrid,
  NGi,
  useMessage
} from 'naive-ui'
import {ChevronBack} from "@vicons/ionicons5";
import {useRoute, useRouter} from "vue-router";
import {invoke, convertFileSrc} from "@tauri-apps/api/tauri";
import {onMounted, ref} from "vue";

const router = useRouter()
const route = useRoute()
const message = useMessage()

const comicList = ref<string[]>([])
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

async function initData(id:number) {
  try {
    let res = <any>await invoke('read_comic', {id})
    let fileList: string[] = res.page
    fileList.forEach((e) => {
      comicList.value.push(convertFileSrc(e))
    })
  } catch (e) {
    router.go(-1)
    message.error('文件无法打开')
  }
}

onMounted(async () => {
  await initData(Number(route.query.id))
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
