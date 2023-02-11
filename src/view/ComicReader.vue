<template>
  <!--顶级 右侧-->
  <n-layout has-sider sider-placement="right" style="height: 100vh;">
    <n-layout-content :native-scrollbar="false">
      <!--次级 左侧 中区-->
      <n-layout has-sider style="height: 100vh;">
        <!--左侧侧边栏-->
        <n-layout-sider
            collapse-mode="width"
            bordered
            :default-collapsed="true"
            :collapsed-width="34"
            show-trigger="arrow-circle"
            :on-update:collapsed="afterCollapsedLeave"
            :on-after-enter="afterCollapsedEnter"
        >
          <div>
            <div>
              <n-space justify="left">
                <n-button strong secondary circle @click="back">
                  <template #icon>
                    <n-icon>
                      <chevron-back/>
                    </n-icon>
                  </template>
                </n-button>
                <div v-if="headShow">
                  <n-input></n-input>
                </div>
                <n-button v-if="headShow" strong secondary circle @click="toConfig">
                  <template #icon>
                    <n-icon>
                      <options-outline/>
                    </n-icon>
                  </template>
                </n-button>
              </n-space>
            </div>
            <n-scrollbar>
              <div>left</div>
            </n-scrollbar>
          </div>
        </n-layout-sider>
        <!--中区图片区-->
        <n-layout-content :native-scrollbar="false">
          <n-grid :cols="2" x-gap="24" y-gap="24">
            <n-gi v-for="comic of comicList">
              <img style="width: 100%;" v-lazy="comic"/>
            </n-gi>
          </n-grid>
        </n-layout-content>
      </n-layout>
    </n-layout-content>
    <!--右侧侧边栏-->
    <n-layout-sider
        collapse-mode="transform"
        :default-collapsed="true"
        show-trigger="arrow-circle"
        content-style="padding: 24px;"
        bordered>
      <div>
        right
      </div>
    </n-layout-sider>

  </n-layout>
</template>

<script setup lang="ts">
import {
  NButton,
  NIcon,
  NScrollbar,
  NInput,
  NSpace,
  NLayout,
  NLayoutContent,
  NLayoutSider,
  NGrid,
  NGi,
  NImage,
  useMessage
} from 'naive-ui'
import {ChevronBack, OptionsOutline} from "@vicons/ionicons5";
import {useRoute, useRouter} from "vue-router";
import {invoke, convertFileSrc} from "@tauri-apps/api/tauri";
import {onMounted, ref} from "vue";

const router = useRouter()
const route = useRoute()
const message = useMessage()

const comicList = ref<string[]>([])

function back() {
  router.go(-1)
}

function toConfig() {
  router.push('/Config')
}

const headShow = ref(false)

function afterCollapsedLeave(collapsed: boolean) {
  if (collapsed) {
    headShow.value = !collapsed
  }
}

function afterCollapsedEnter() {
  headShow.value = true
}

onMounted(async () => {
  try {
    let res = <any>await invoke('read_comic', {id: Number(route.query.id)})
    let fileList: string[] = res.page
    fileList.forEach((e) => {
      comicList.value.push(convertFileSrc(e))
    })
  } catch (e) {
    router.go(-1)
    message.error('文件无法打开')
  }
})

</script>

<style scoped>

</style>
