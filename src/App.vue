<script setup lang="ts">
import {NConfigProvider, NMessageProvider,zhCN, dateZhCN,darkTheme,NScrollbar} from 'naive-ui'
</script>

<template>
<!--  todo style中高度通过动态获取-->
  <n-scrollbar style="max-height: 600px">
    <n-config-provider :locale="zhCN" :date-locale="dateZhCN" :theme="darkTheme">
      <n-message-provider>
        <RouterView ref="root"/>
      </n-message-provider>
    </n-config-provider>
  </n-scrollbar>
</template>

<style scoped>
</style>
