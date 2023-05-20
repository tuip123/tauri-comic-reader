<script setup lang="ts">
import {darkTheme, dateZhCN, NConfigProvider, NMessageProvider, NScrollbar, zhCN} from 'naive-ui'
import {invoke} from "@tauri-apps/api/tauri";
import {appWindow} from '@tauri-apps/api/window';

let timechecker: number = 0

window.addEventListener('mousedown', function (event) {
  if (event.button === 1) {
    timechecker = new Date().getTime()
  }
})

window.addEventListener('mouseup', async function (event) {
  if (event.button === 4 || event.button === 3) {
    event.preventDefault()
  }
  if (event.button === 1) {
    if (((new Date().getTime() - timechecker) < 250) && await invoke('minimize_window'))
      await appWindow.minimize();
  }
})
</script>

<template>
  <n-scrollbar x-scrollable>
    <n-config-provider :locale="zhCN" :date-locale="dateZhCN" :theme="darkTheme" style="min-width: 400px">
      <n-message-provider>
        <RouterView ref="root"/>
      </n-message-provider>
    </n-config-provider>
  </n-scrollbar>
</template>

<style scoped>
</style>
