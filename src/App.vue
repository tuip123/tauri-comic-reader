<script setup lang="ts">
import {darkTheme} from 'naive-ui'
import {NConfigProvider} from 'naive-ui'
import {zhCN, dateZhCN} from 'naive-ui'
</script>

<template>
  <n-config-provider :locale="zhCN" :date-locale="dateZhCN" :theme="darkTheme">
    <RouterView/>
  </n-config-provider>
</template>

<style scoped>
</style>
