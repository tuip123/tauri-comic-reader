<script setup lang="ts">
import {NConfigProvider, NMessageProvider, zhCN, dateZhCN, darkTheme, NScrollbar} from 'naive-ui'

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
