<script setup lang="ts">
import {darkTheme} from 'naive-ui'
import {NConfigProvider, NMessageProvider} from 'naive-ui'
import {zhCN, dateZhCN} from 'naive-ui'
</script>

<template>
  <n-config-provider :locale="zhCN" :date-locale="dateZhCN" :theme="darkTheme">
    <n-message-provider>
      <RouterView/>
    </n-message-provider>
  </n-config-provider>
</template>

<style scoped>
</style>
