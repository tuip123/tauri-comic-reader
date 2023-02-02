<template>
  <n-card hoverable style="height: 450px;width: 300px" @click="testRead" ref="root_card">
    <template #cover>
      <img src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg" alt="">
    </template>
    <n-thing content-indented>
      <template #header>
        {{ props.comic.id }}
      </template>
      <template #description>
        <n-ellipsis style="max-width: 240px">
          {{ props.comic.title }}
        </n-ellipsis>
      </template>
      <template #action>
        <n-space>
          <n-button size="small" @click.stop="testDelete">
            1 块钱
          </n-button>
        </n-space>
      </template>
    </n-thing>
  </n-card>
</template>

<script setup lang="ts">
import {NCard,NThing,NButton,NSpace,NEllipsis} from 'naive-ui'
import {convertFileSrc} from "@tauri-apps/api/tauri"

const props = defineProps(['comic'])
const src = convertFileSrc(props.comic.cover)

function testDelete() {
  console.log('testDelete')
}function testRead() {
  console.log('testRead')
}
</script>

<style scoped>

</style>
