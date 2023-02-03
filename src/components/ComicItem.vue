<template>
  <n-card ref="root_card" :bordered="false" >
    <template #cover>
      <img src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg" alt=""  @click="testRead" >
<!--      <img :src="src" alt="">-->
    </template>
    <n-thing content-indented>
      <template #header>
        {{ props.comic.id }}
      </template>
      <template #description >
        <n-ellipsis style="max-width: 140px">
          {{ props.comic.title }}
        </n-ellipsis>
      </template>
      <template #action>
        <n-space>
          <n-button size="small" @click.stop="testDelete">
            删除
          </n-button>
        </n-space>
      </template>
    </n-thing>
  </n-card>
</template>

<script setup lang="ts">
import {NCard,NThing,NButton,NSpace,NEllipsis} from 'naive-ui'
import {convertFileSrc,invoke} from "@tauri-apps/api/tauri"

const props = defineProps(['comic'])
const src = convertFileSrc(props.comic.cover)
const emit = defineEmits(['delete'])
function testDelete() {
  invoke('delete_comic',{id:props.comic.id}).then((res)=>{
    console.log(res)
    emit('delete')
  }).catch((err)=>{
    console.error(err)
  })
}
function testRead() {
  invoke('open_with_third_party',{folder:props.comic.path}).then((res)=>{
    console.log(res)
  }).catch((err)=>{
    console.error(err)
  })
}
</script>

<style scoped>

</style>
