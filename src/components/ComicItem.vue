<template>
  <n-card :bordered="false" content-style="padding:0 10px 20px 10px">
    <template #cover>
      <!--      <img src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg" alt=""  @click="testRead" >-->
      <img :src="src" alt="" @click="testRead">
    </template>
    <n-thing>

      <template #header-extra>
        {{ props.comic.count }}P
      </template>

      <template #header>
        <n-ellipsis style="max-width: 140px">
          <template #default>
          <span @click="copyTitle">
          {{ props.comic.title }}
          </span>
          </template>
        </n-ellipsis>
      </template>

      <template #description>
        <n-space justify="space-between">
          <n-button circle @click.stop="openSourceFolder" type="primary" size="small">
            <n-icon>
              <folder-open-outline/>
            </n-icon>
          </n-button>
          <n-button circle @click.stop="deleteComic" type="error" size="small">
            <n-icon>
              <trash-outline/>
            </n-icon>
          </n-button>
        </n-space>
      </template>
    </n-thing>
  </n-card>
</template>

<script setup lang="ts">
import {NCard, NThing, NButton, NSpace, NEllipsis, NIcon, useMessage} from 'naive-ui'
import {FolderOpenOutline, TrashOutline} from '@vicons/ionicons5'
import {convertFileSrc, invoke} from "@tauri-apps/api/tauri"

const message = useMessage()
const props = defineProps(['comic'])
const src = convertFileSrc(props.comic.cover)
const emit = defineEmits(['delete'])

async function deleteComic() {
  // todo 根据config中是否删除源文件 提供不同的文本提示
  await invoke('delete_comic', {id: props.comic.id})
      .then(() => {
        message.success('已删除：' + props.comic.title)
      })
      .catch((err) => {
        message.error('删除错误：' + err as string)
      })
  emit('delete')
}

function testRead() {
  // todo 根据是否以第三方打开 进行invoke或者router.push
  message.info('正在用第三方查看器打开：' + props.comic.title)
  invoke('open_with_third_party', {folder: props.comic.path})
      .catch((err) => {
        message.error('打开失败：' + err as string)
      })
}

function openSourceFolder() {
  message.info('正在打开：' + props.comic.title)
  invoke('open_source_folder', {folder: props.comic.path})
      .then(()=>{
        message.success('已打开')
      })
      .catch((err) => {
        message.error('打开失败：' + err as string)
      })
}

function copyTitle() {
  const clipboard = navigator.clipboard
  clipboard.writeText(props.comic.title)
  message.info('已复制')
}
</script>

<style scoped>

</style>
