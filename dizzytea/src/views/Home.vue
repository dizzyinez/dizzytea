<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import Entity from './Entity.vue'

async function run_script() {
        disp.value = await invoke("ecs_run_script", {
                path: "Core", code: `
using flecs.meta

Concept {}
Quote {}
Thought {}
Media {}

Project {}
Area {}
Resource {}
Archived {}

Created {}
Expires {}

struct Note {
        text = string
}

struct HyperLink {
        link = string
}

Description {}

FeelsLike {}
In {}
Contains {}
        `})
}

const disp = ref("");
async function get_entire_world() {
        disp.value = JSON.parse(await invoke("get_entire_world"));
        console.log(disp.value);
}

async function new_entity() {
        disp.value = await invoke("ecs_entity", { path: "fuck you" });
        console.log(disp.value);
}

async function get() {
        disp.value = JSON.parse(await invoke("ecs_get", {path: "fuck you", component: ""}))
}

function get_today_path() : string {
        let today = new Date();
        return today.getFullYear() + "::" + (today.getMonth()+1) + "::" + today.getDate();
}

function get_yesterday_path() : string {
        let day = new Date();
        day.setDate(day.getDate() - 1);
        return day.getFullYear() + "::" + (day.getMonth()+1) + "::" + day.getDate();
}

async function today() {
        let today_path = get_today_path();
        await invoke("ecs_entity", { path: today_path });
        await invoke("")
}

</script>

<template>
        <button @click="get_entire_world"> Print World </button>
        <button @click="new_entity"> new </button>
        <button @click="run_script"> script </button>
        <button @click="today"> today </button>
        <div>
                <Entity :path="get_today_path()"/>
        </div>
</template>
