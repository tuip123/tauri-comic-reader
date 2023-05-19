<script setup lang="ts">
import {darkTheme, dateZhCN, NConfigProvider, NMessageProvider, NScrollbar, zhCN} from 'naive-ui'
import {invoke} from "@tauri-apps/api/tauri";
import {appWindow} from '@tauri-apps/api/window';

async function minimizeWindow(event: any) {
  if (event.button == 1) {
    let res = await invoke('minimize_window')
    if (res)
      await appWindow.minimize();
  }
}

window.addEventListener('mouseup', function (event) {
  if (event.button === 4 || event.button === 3) {
    event.preventDefault()
  }
})
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
