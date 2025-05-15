<script lang="ts">
    import type { Tab } from '../types/tabs';

    interface Props {
        items?: Tab[];
        activeTabValue?: number;
    }

    let { items = [], activeTabValue = $bindable(1) }: Props = $props();
</script>

<ul class="flex flex-wrap pl-0 mb-0 list-none">
    {#each items as item}
        <li>
            <button
                class="border border-transparent rounded-t block py-2 px-4 cursor-pointer hover:border-white {activeTabValue ===
                item.value
                    ? 'text-black bg-white border-white'
                    : ''}"
                onclick={() => (activeTabValue = item.value)}
            >
                {item.label}
            </button>
        </li>
    {/each}
</ul>

{#each items as item}
    {#if activeTabValue == item.value}
        <div class="border-t border-white">
            <item.component />
        </div>
    {/if}
{/each}
