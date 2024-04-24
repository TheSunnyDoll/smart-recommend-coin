<template>
    <div class="w-full max-w-full px-2 py-2 sm:px-0">
        <ul>
            <li
                v-for="(items, idx) in Object.values(categories)"
                :key="idx"
                :class="[
                    'rounded-xl bg-white p-3',
                    'ring-white/60 ring-offset-2 ring-offset-blue-400 focus:outline-none focus:ring-2',
                ]"
            >
                <ul>
                    <li
                        v-for="item in items"
                        :key="item.id"
                        class="relative rounded-md p-3 hover:bg-gray-100"
                    >
                        <div class="flex">
                            <img
                                :src="item.token_icon"
                                alt="Token Icon"
                                class="w-6 h-6 mr-2"
                            />
                            <h3 class="text-sm font-medium leading-5">
                                {{ item.token_symbol }}
                                <span class="text-gray-500"
                                    >({{ item.token_name }})</span
                                >
                            </h3>
                        </div>

                        <ul
                            class="mt-3 flex flex-wrap space-x-1 text-xs font-normal leading-4 text-gray-500"
                        >
                            <!-- <li>{{ item.created }}</li>
                                <li>&middot;</li> -->
                            <li>{{ item.token_address }}</li>
                            <li>&middot;</li>
                            <li>Holder: {{ item.holders }}</li>
                        </ul>
                    </li>
                </ul>
            </li>
        </ul>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { fetchWithBaseUrl } from "../api.ts";
import { TabGroup, TabList, Tab, TabPanels, TabPanel } from "@headlessui/vue";

const token_recommend = ref([]);
const categories = ref({
    Trending: token_recommend,
    // SmartAddress: [],
});
onMounted(async () => {
    try {
        // const response = await fetch("/api/tokens/recommend");
        const response = await fetchWithBaseUrl("/api/tokens/recommend");
        if (!response.ok) {
            throw new Error("Failed to fetch token recommend");
        }
        const data = await response.json();
        token_recommend.value = data;
    } catch (error) {
        console.log(error);
    }
});
</script>
