<script setup lang="ts">
import {darkTheme, dateZhCN, NConfigProvider, NMessageProvider, NScrollbar, zhCN} from 'naive-ui'
import {invoke} from "@tauri-apps/api/tauri";

async function minimizeWindow(event: any) {
  if (event.button == 1) {
    await invoke('minimize_window')
  }
}

</script>

<template>
  <n-scrollbar x-scrollable @mousedown="minimizeWindow">
    <n-config-provider :locale="zhCN" :date-locale="dateZhCN" :theme="darkTheme" style="min-width: 400px">
      <n-message-provider>
        <RouterView ref="root"/>
      </n-message-provider>
    </n-config-provider>
  </n-scrollbar>
</template>

<style scoped>
</style>
