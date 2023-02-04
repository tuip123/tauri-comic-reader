<template>
  <n-layout vertical size="large">
    <n-layout-header style="height: 64px;padding: 12px">
      <Header noSearch noConfig/>
    </n-layout-header>
    <n-layout-content>
      <Test/>
    </n-layout-content>
  </n-layout>
</template>
<!--todo 展示，修改config-->
<script setup lang="ts">
import { NLayout, NLayoutHeader,NLayoutContent} from "naive-ui"
import Header from "@/components/Header.vue";
import Test from "@/components/Test.vue";

</script>

<style scoped>

</style>
