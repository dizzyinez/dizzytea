<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref, watchEffect } from 'vue';

const props = defineProps({
        path: String
})

const entity = ref({});
const comp = ref("");

async function update_entity() {
        entity.value = JSON.parse(await invoke("ecs_entity", { path: props.path }))
}

async function add_comp() {
        console.log("fuck you")
        await invoke("ecs_add", { path: props.path, component: comp.value })
        entity.value = JSON.parse( await invoke("ecs_entity", { path: props.path }))
}

watchEffect(async () => {
        update_entity()
})

</script>

<template>
        <h1> {{ entity?.name }} </h1>
        <input v-model="comp" /> <button @click="add_comp"> submit </button>
        <p> {{ entity }} </p>
</template>
