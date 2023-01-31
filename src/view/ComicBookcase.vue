<template>
comic bookcase
  {{id}}
</template>

<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
const route = useRoute()
const router = useRouter()
const id = route.query.id
</script>

<style scoped>

</style>
