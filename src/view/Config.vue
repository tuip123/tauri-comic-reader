<template>
  <n-layout vertical size="large">
    <n-layout-header style="height: 64px;padding: 12px">
      <Header noSearch noConfig/>
    </n-layout-header>
  </n-layout>
</template>

<script setup lang="ts">
import { NLayout, NLayoutHeader} from "naive-ui"
import Header from "@/components/Header.vue";

</script>

<style scoped>

</style>
