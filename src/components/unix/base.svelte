<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import Convert from './tabs/convert.svelte';
    import Icon from '../icon.svelte';
    import { copy } from '../../util/tauri';
    import { isMinimized } from '../../stores/main-window';
    import type { Unsubscriber } from 'svelte/store';
    import Settings from './tabs/settings.svelte';
    import Tabs from '../tabs.svelte';

    let interval: number;
    let nowMS: number = Date.now();
    let nowS: number = Math.floor(Date.now() / 1000);
    let minimizedUnsubscriber: Unsubscriber;

    function initInterval() {
        interval = setInterval(() => {
            const n = Date.now();
            nowMS = n;
            nowS = Math.floor(n / 1000);
        }, 1);
    }

    onMount(() => {
        initInterval();

        // Subscribe to minimised to
        isMinimized.subscribe((value) => {
            // If minimized, clear interval.
            if (value) {
                clearInterval(interval);
                return;
            }

            // Else, re-init interval.
            initInterval();
        });
    });

    onDestroy(() => {
        clearInterval(interval);
        minimizedUnsubscriber?.();
    });
</script>

<main>
    <p class="mt-8 mb-2 text-center text-2xl">Unix Utilities</p>

    <div class="flex space-x-4 py-2 bg-accent items-center justify-center mb-4">
        <div class="flex flex-col justify-center">
            <div class="flex space-x-2 items-center justify-center">
                <p class="text-xs">seconds</p>
                <button
                    class="cursor-pointer"
                    on:click={() => copy(nowS, true)}
                >
                    <Icon name="copy"></Icon>
                </button>
            </div>
            <p>{nowS}</p>
        </div>
        <div class="flex flex-col items-center justify-center">
            <div class="flex space-x-2 items-center justify-center">
                <p class="text-xs">milliseconds</p>
                <button
                    class="cursor-pointer"
                    on:click={() => copy(nowMS, true)}
                >
                    <Icon name="copy"></Icon>
                </button>
            </div>
            <p>{nowMS}</p>
        </div>
    </div>

    <div class="px-4">
        <Tabs
            items={[
                {
                    label: 'Convert',
                    value: 1,
                    component: Convert,
                },
                {
                    label: 'Settings',
                    value: 2,
                    component: Settings,
                },
            ]}
        />
    </div>
</main>
