<template>
    <div
        class="flex min-h-screen justify-center bg-gradient-to-l from-sky-500 to-blue-400"
    >
        <Nav />
        <!-- <Index /> -->
    </div>
</template>
<script setup>
import { onMounted, ref } from "vue";
import Index from "./components/Index.vue";
import Nav from "./components/Nav.vue";

const categories = ref({
    Trending: [],
    SmartAddress: [],
});
</script>

<style scoped>
.logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
}
.logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
    filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
