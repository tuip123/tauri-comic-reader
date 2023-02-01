<template>
  <n-space justify="space-between" size="large">
    <div style="width: 10vh">
      <div v-if="props.noBack===undefined">
        <n-space justify="end">
          <n-button strong secondary circle @click="back">
            <template #icon>
              <n-icon>
                <chevron-back/>
              </n-icon>
            </template>
          </n-button>
        </n-space>

      </div>
    </div>
    <div v-if="props.noSearch===undefined">
      <n-space justify="space-around" size="large">
        <n-input
            round
            clearable
            v-model:value="searchWord"
            :on-input="query"
            placeholder="搜索"
        >
          <template #prefix>
            <n-icon @click="toConfig">
              <search-outline/>
            </n-icon>
          </template>
        </n-input>
      </n-space>
    </div>
    <div style="width: 10vh">
      <div v-if="props.noConfig===undefined">
        <n-button strong secondary circle @click="toConfig">
          <template #icon>
            <n-icon>
              <options-outline/>
            </n-icon>
          </template>
        </n-button>
      </div>
    </div>
  </n-space>

</template>

<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
import {ref} from "vue";
import {NSpace, NButton, NIcon, NInput} from "naive-ui"
import {SearchOutline, OptionsOutline,ChevronBack} from "@vicons/ionicons5";

const router = useRouter()
const route = useRoute()
const path = route.path
const searchWord = ref("")
const props = defineProps(['noSearch','noBack','noConfig'])
console.log(props)
const emit = defineEmits(['query'])

function query() {
  emit('query', searchWord.value)
}

function back(){
  router.go(-1)
}

function toConfig(){
  router.push('/Config')
}

</script>

<style scoped>

</style>
