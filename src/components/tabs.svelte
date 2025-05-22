<script lang="ts">
    import type { Tab } from '../types/tabs';

    let {
        items = [],
        activeTabValue = $bindable(1),
        orientation = 'horizontal',
        ulClasses = '',
    }: {
        items?: Tab[];
        activeTabValue?: number;
        orientation?: 'vertical' | 'horizontal';
        ulClasses?: string;
    } = $props();
</script>

<ul
    class="{orientation == 'horizontal'
        ? 'flex flex-wrap pl-0 mb-0'
        : ''} list-none {ulClasses}"
>
    {#each items as item}
        <li class={item.liClasses}>
            <button
                class="border border-transparent block cursor-pointer hover:border-white {activeTabValue ===
                item.value
                    ? 'text-black bg-white border-white'
                    : ''} {orientation == 'horizontal'
                    ? 'rounded-t py-2 px-4'
                    : 'rounded-l py-4 px-2'} {item.buttonClasses}"
                onclick={() => (activeTabValue = item.value)}
            >
                {#if orientation == 'horizontal'}
                    {item.label}
                {:else}
                    <span class="[writing-mode:_sideways-lr]">
                        {item.label}
                    </span>
                {/if}
            </button>
        </li>
    {/each}
</ul>

{#each items as item}
    {#if activeTabValue == item.value}
        <div class={orientation == 'horizontal' ? 'border-t border-white' : ''}>
            {#if item.component}
                <item.component />
            {/if}
        </div>
    {/if}
{/each}
