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
              {{ BASE_NAME }} | {{ CN_NAME }}
            </div>
          </template>
          <template #footer>
            <n-grid :cols="6" style="padding-bottom: 12px">
              <n-gi>
                <div style="text-align: center;height: 100%;display: flex;justify-content: center;align-items: center;">
                  <n-switch :value="config.third_party_open" @update:value="thirdPartyOpenSetting">
                    <template #checked>
                      第三方阅读
                    </template>
                    <template #unchecked>
                      应用内阅读
                    </template>
                  </n-switch>
                </div>
              </n-gi>
              <n-gi>
                <div style="text-align: center;height: 100%;display: flex;justify-content: center;align-items: center;">
                  <n-switch :value="config.delete_source_file" @update:value="deleteSourceFileSetting">
                    <template #checked>
                      删除源文件
                    </template>
                    <template #unchecked>
                      普通删除
                    </template>
                  </n-switch>
                </div>
              </n-gi>
            </n-grid>
            <n-grid :cols="6">
              <n-gi>
                <div style="text-align: center;height: 100%;display: flex;justify-content: center;align-items: center;">
                  第三方阅读器路径：
                </div>
              </n-gi>
              <n-gi span="5">
                <div>
                  <n-input ref="partyInput" :disabled="!config.third_party_open" type="text"
                           :placeholder="config.third_party_image_viewer"
                           @click="setPath"/>
                </div>
              </n-gi>
            </n-grid>
          </template>
          <template #header-extra>
            <div>
              版本：{{ config.version }}
            </div>
          </template>
        </n-thing>

      </n-card>
    </n-layout-content>
  </n-layout>
</template>
<!--todo 修改config-->
<script setup lang="ts">
import {NLayout, NLayoutHeader, NLayoutContent, NCard, NInput, NGrid, NGi, NSwitch, NSpace, NThing} from "naive-ui"
import Header from "@/components/Header.vue";
import Test from "@/components/Test.vue";
import {useConfigStore} from "../store/config";
import {ref} from 'vue'

const BASE_NAME = import.meta.env.VITE_APP_BASE_NAME
const CN_NAME = import.meta.env.VITE_APP_CN_NAME

const config = useConfigStore()

const partyInput = ref()

function setPath() {
  partyInput.value.blur()
//  单独设置path：点击 利用open打开文件 判断是否成功 保存路径 保存设置 重新获取设置
}

function thirdPartyOpenSetting() {
  config.third_party_open = !config.third_party_open
//  打开 第三方启动器：点击按钮，判断路径是否为空->利用open打开exe文件 else 直接设置third_party_open 保存设置 重新获取设置
//  open打开是否成功->保存路径，保存third_party_open，重新获取config else third_party_open不改变

//  关闭 直接保存设置 重新获取设置
}

function deleteSourceFileSetting() {
  config.delete_source_file = !config.delete_source_file
//  设置关闭 保存设置 重新获取设置
}
</script>

<style scoped>

</style>
