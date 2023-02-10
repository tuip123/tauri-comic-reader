<template>
  <n-layout has-sider>
    <n-layout-sider
        collapse-mode="width"
        bordered
        :collapsed-width="120"
        :width="240"
        show-trigger="bar"
    >
      <div style="height: 10vh">
        <n-space justify="center">
          <n-button strong secondary circle @click="back">
            <template #icon>
              <n-icon>
                <chevron-back/>
              </n-icon>
            </template>
          </n-button>
          <div>
            <n-input></n-input>
          </div>
          <n-button strong secondary circle @click="toConfig">
            <template #icon>
              <n-icon>
                <options-outline/>
              </n-icon>
            </template>
          </n-button>
        </n-space>
      </div>
      <n-scrollbar style="max-height: 90vh">
        <div style="height: 200vh">left</div>
      </n-scrollbar>
    </n-layout-sider>
    <n-layout-content>
      <n-layout has-sider sider-placement="right">
        <n-layout-content style="height: calc(100vh);" :native-scrollbar="false" bordered>
          <div style="height: 300vh">
            mid
          </div>
        </n-layout-content>
        <n-layout-sider
            style="height: calc(100vh);"
            collapse-mode="width"
            :native-scrollbar="false"
            bordered
            :collapsed-width="120"
            :width="240"
            show-trigger="bar"
            content-style="padding: 24px;"
        >
          <div style="height: 300vh">
            right
          </div>
        </n-layout-sider>
      </n-layout>
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import {NButton, NIcon, NScrollbar, NInput, NSpace, NLayout, NLayoutContent, NLayoutSider, useMessage} from 'naive-ui'
import {ChevronBack, OptionsOutline} from "@vicons/ionicons5";
import {useRoute, useRouter} from "vue-router";
import {invoke} from "@tauri-apps/api/tauri";
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

onMounted(async () => {
  try {
    let res = <any>await invoke('read_comic', {id: Number(route.query.id)})
    comicList.value = res.page as string[]
    message.success(comicList.value[0] + '  ' + comicList.value.length)
  } catch (e) {
    router.go(-1)
    message.error('文件无法打开')
  }
})
</script>

<style scoped>

</style>
