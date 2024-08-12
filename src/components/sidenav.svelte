<script lang="ts">
    import Icon from './icon.svelte';
    import { createEventDispatcher } from 'svelte';

    const icons: { text: string; page: string; icon: any }[] = [
        {
            text: 'Unix',
            page: 'unix',
            icon: '123',
        },
    ];
    const dispatch = createEventDispatcher();
</script>

<!-- Fake sidenav to push contents beside. -->
<div class="w-16"></div>

<!-- Actual fixed sidenav to not have it scroll with the page. -->
<div
    class="fixed h-screen bg-[#231a18] border-r border-white border-opacity-50 py-2 space-y-4 w-16"
>
    <div
        class="border-b border-white border-opacity-50 pb-2 px-2 w-full flex flex-col items-center justify-center"
    >
        <img src="/favicon.png" alt="Icon" />
        <!-- <p class="text-sm text-center">Isaac Utils</p> -->
    </div>

    {#each icons as icon}
        <button
            class="border-b border-white border-opacity-50 px-2 pb-2 flex flex-col items-center justify-center w-full"
            on:click={() => dispatch('page', icon.page)}
        >
            <Icon
                name={icon.icon}
                class="text-4xl rounded-full border border-white mb-1"
            />
            <p class="text-sm">{icon.text}</p>
        </button>
    {/each}
</div>
