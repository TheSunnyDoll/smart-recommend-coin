<template>
    <div class="flex w-full">
        <aside
            class="w-72 text-white flex flex-col from-sky-400 to-blue-500 bg-gradient-to-r"
        >
            <div class="flex-shrink-0 flex items-center justify-center h-16">
                <!-- <img
                    src="https://www.pngwing.com/en/search?q=sample+Logo"
                    alt="Logo"
                    class="w-12 h-12"
                /> -->
                <span class="ml-2 text-lg font-bold">Smart Recommend Coin</span>
            </div>

            <nav
                class="flex-grow flex flex-col bg-transparent overflow-y-hidden h-full"
            >
                <router-link to="/" class="nav-link cursor-pointer"
                    >Trending coin</router-link
                >
                <router-link
                    to="/smart_address"
                    class="nav-link cursor-pointer text-xs"
                    >Smart Address(comming soon)</router-link
                >
                <router-link
                    to="/solana"
                    class="nav-link cursor-pointer text-xs"
                    >Solana</router-link
                >
            </nav>
        </aside>

        <div
            class="flex min-h-screen w-full justify-center bg-gradient-to-l from-sky-500 to-blue-400"
        >
            <router-view />
        </div>
    </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
const router = useRouter();
const current_path = router.currentRoute.value.path;
onMounted(() => {
    console.log(router.currentRoute.value.fullPath);
});
</script>

<style scoped>
.nav-link {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    text-decoration: none;
    color: white;

    &:hover {
        background-color: blue;
    }
}
</style>
