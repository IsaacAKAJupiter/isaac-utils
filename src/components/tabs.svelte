<script lang="ts">
    import type { Tab } from '../types/tabs';

    export let items: Tab[] = [];
    export let activeTabValue = 1;
</script>

<ul class="flex flex-wrap pl-0 mb-0 list-none">
    {#each items as item}
        <li>
            <button
                class="border border-transparent rounded-t block py-2 px-4 cursor-pointer hover:border-white {activeTabValue ===
                item.value
                    ? 'text-black bg-white border-white'
                    : ''}"
                on:click={() => (activeTabValue = item.value)}
            >
                {item.label}
            </button>
        </li>
    {/each}
</ul>

{#each items as item}
    {#if activeTabValue == item.value}
        <div class="border-t border-white">
            <svelte:component this={item.component} />
        </div>
    {/if}
{/each}
