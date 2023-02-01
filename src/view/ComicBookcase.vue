<template>
  <n-layout vertical size="large">
    <n-layout-header style="height: 64px;padding: 12px">
      <Header/>
    </n-layout-header>
    <n-layout-content>
      {{libraryId}}
    </n-layout-content>
    <n-layout-footer
        style="height: 64px;padding: 24px">
      <n-space justify="center" size="large">
        <n-pagination
            v-model:page="pagination.current"
            v-model:page-size="pagination.size"
            :item-count="pagination.total"
            :page-sizes="[10, 20]"
            :page-slot="5"
            show-size-picker
            :on-update:page="pageChange"
            :on-update:page-size="sizeChange"
        />
      </n-space>
    </n-layout-footer>
  </n-layout>
</template>
<!--todo 展示、搜索、删除漫画-->
<script setup lang="ts">
import { NLayout, NLayoutHeader,NLayoutContent,NLayoutFooter,NSpace,NPagination} from "naive-ui"
import Header from "@/components/Header.vue";
import {useRoute,useRouter} from "vue-router";
import {ref} from "vue";
interface Pagination {
  current: number,
  size: number,
  total: number,
}
const pagination = ref<Pagination>({
  current: 1,
  size: 10,
  total: 1
})
const router = useRouter()
const route = useRoute()
const libraryId = route.query.libraryId

console.log(libraryId)

async function queryComic(){

}

async function pageChange(num: number) {
  pagination.value.current = num
  await queryComic()
}

async function sizeChange(num: number) {
  pagination.value.size = num
  await queryComic()
}

</script>

<style scoped>

</style>
