<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';


const world = ref("");
async function get_entire_world() {
        world.value = await invoke("get_entire_world");
}

async function reload() {
        console.log("test");
        await invoke("reload_asset_metadata");
}

</script>

<template>
        <form @submit.prevent="get_entire_world">
                <button type="submit"> Print World </button>
        </form>
        <p> {{ world }} </p>
        <button @click="reload"> reload </button>
</template>
