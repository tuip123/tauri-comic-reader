<template>
  <n-layout vertical size="large">
    <n-layout-header style="height: 64px;padding: 12px">
      <Header noSearch noConfig/>
    </n-layout-header>
    <n-layout-content style="height: calc(100vh - 64px);padding: 12px">
      <Test v-show="false"/>
      <n-card style="width: 80%;height: 100%;margin: auto;">
        <n-thing>
          <template #header>
            <div>
              {{ BASE_NAME }}
              <n-divider vertical/>
              {{ CN_NAME }}
            </div>
          </template>

          <template #header-extra>
            <div>
              <n-divider vertical/>
              版本：{{ config.version }}
            </div>
          </template>

          <template #footer>
            <n-divider/>
            <n-grid :cols="6" style="padding-bottom: 12px">
              <n-gi>
                <div style="text-align: center;height: 100%;display: flex;justify-content: center;align-items: center;">
                  <n-switch :value="config.minimize_window" @update:value="minimizeWindow">
                    <template #checked>
                      开启中键最小化
                    </template>
                    <template #unchecked>
                      停用中键最小化
                    </template>
                  </n-switch>
                </div>
              </n-gi>

              <n-gi>
                <div style="text-align: center;height: 100%;display: flex;justify-content: center;align-items: center;">
                  <n-switch :value="config.third_party_open" @update:value="thirdPartyOpenSetting">
                    <template #checked>
                      使用第三方阅读
                    </template>
                    <template #unchecked>
                      使用应用内阅读
                    </template>
                  </n-switch>
                </div>
              </n-gi>

              <n-gi>
                <div style="text-align: center;height: 100%;display: flex;justify-content: center;align-items: center;">
                  <n-switch :value="config.delete_source_file" @update:value="deleteSourceFileSetting"
                            :rail-style="railStyle">
                    <template #checked>
                      移入回收站
                    </template>
                    <template #unchecked>
                      列表删除
                    </template>
                  </n-switch>
                </div>
              </n-gi>

            </n-grid>
            <n-grid :cols="12">
              <n-gi span="2">
                <div style="text-align: center;height: 100%;display: flex;justify-content: center;align-items: center;">
                  第三方阅读器路径：
                </div>
              </n-gi>
              <n-gi span="9">
                <div>
                  <n-input ref="partyInput" :disabled="!config.third_party_open" type="text"
                           :placeholder="config.third_party_image_viewer"
                           @click="setPath"/>
                </div>
              </n-gi>
            </n-grid>
          </template>
        </n-thing>

      </n-card>
    </n-layout-content>
  </n-layout>
</template>
<script setup lang="ts">
import {
  NCard,
  NDivider,
  NGi,
  NGrid,
  NInput,
  NLayout,
  NLayoutContent,
  NLayoutHeader,
  NSwitch,
  NThing,
  useMessage
} from "naive-ui"
import Header from "@/components/Header.vue";
import Test from "@/components/Test.vue";
import {Config, getConfig, useConfigStore} from "@/store/config";
import {CSSProperties, ref} from 'vue'
import {invoke} from "@tauri-apps/api/tauri";
import {open} from "@tauri-apps/api/dialog";

const BASE_NAME = import.meta.env.VITE_APP_BASE_NAME
const CN_NAME = import.meta.env.VITE_APP_CN_NAME

const config = useConfigStore() as Config
const partyInput = ref()
const message = useMessage()
const railStyle = ({focused, checked}: { focused: boolean, checked: boolean }) => {
  const style: CSSProperties = {}
  if (checked) {
    style.background = '#d03050'
    if (focused) {
      style.boxShadow = '0 0 0 2px #d0305040'
    }
  }
  return style
}

async function setPath() {
  if (config.third_party_open) {
    partyInput.value.blur()
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: '可执行程序',
        extensions: ['exe']
      }, {
        name: '所有文件',
        extensions: ['*']
      }]
    }) as string | unknown;
    if (selected !== null) {
      invoke("update_config", {key: 'third_party_image_viewer', value: selected}).then(async (res) => {
        message.success(res as string)
        await getConfig()
      }).catch((err)=>{
        message.error(err)
      })
    }
  }
}

async function thirdPartyOpenSetting() {
  if (config.third_party_open) {
    await invoke("update_config", {key: 'third_party_image_viewer', value: "null"})
    await invoke("update_config", {key: 'third_party_open', value: "false"})
    message.info('已关闭第三方启动')
  } else {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: '可执行程序',
        extensions: ['exe']
      }, {
        name: '所有文件',
        extensions: ['*']
      }]
    }) as string | unknown;
    if (selected !== null) {
      await invoke("update_config", {key: 'third_party_image_viewer', value: selected})
      await invoke("update_config", {key: 'third_party_open', value: "true"})
      message.success('设置成功')
    }
    else {
      message.info('已取消')
    }
  }
  await getConfig()
}

async function deleteSourceFileSetting() {
  await invoke("update_config", {key: 'delete_source_file', value: config.delete_source_file ? 'false' : 'true'})
  await getConfig()
  if (config.delete_source_file) {
    message.error('请注意，现在删除书柜中的漫画会导致文件被删除！！！')
  } else {
    message.info('已关闭删除源文件功能')
  }
}

async function minimizeWindow() {
  await invoke("update_config", {key: 'minimize_window', value: config['minimize_window'] ? 'false' : 'true'})
  await getConfig()
  if (config.minimize_window) {
    message.info('已开启最小化窗口')
  } else {
    message.info('已关闭最小化窗口')
  }
}

</script>

<style scoped>

</style>
