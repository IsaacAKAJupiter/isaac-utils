<script lang="ts">
    import Icon from './icon.svelte';

    const icons: { text: string; page: string; icon: any }[] = [
        {
            text: 'unix',
            page: 'unix',
            icon: '',
        },
        {
            text: 'uuid',
            page: 'uuid',
            icon: '',
        },
        {
            text: 'p2p',
            page: 'p2p',
            icon: '',
        },
        {
            text: 'barcode',
            page: 'barcode',
            icon: '',
        },
        {
            text: 'base64',
            page: 'base64',
            icon: '',
        },
    ];
    let { onPageChange }: { onPageChange: (p: string) => void } = $props();
</script>

<!-- Fake sidenav to push contents beside. -->
<div class="w-16"></div>

<!-- Actual fixed sidenav to not have it scroll with the page. -->
<div
    class="fixed h-screen bg-[#231a18] border-r border-white border-opacity-50 py-2 space-y-4 w-16"
>
    <div class="h-full flex flex-col">
        <div
            class="border-b border-white border-opacity-50 pb-2 px-2 w-full flex flex-col items-center justify-center"
        >
            <img src="/favicon.png" alt="Icon" />
        </div>

        <div class="flex-1 overflow-auto scrollbar-thin">
            {#each icons as icon}
                <button
                    class="border-b border-white border-opacity-50 px-2 py-2 flex flex-col items-center justify-center w-full"
                    onclick={() => onPageChange(icon.page)}
                >
                    {#if icon.icon}
                        <Icon
                            name={icon.icon}
                            class="text-4xl rounded-full border border-white mb-1"
                        />
                    {/if}
                    <p class="text-sm">{icon.text}</p>
                </button>
            {/each}
        </div>

        <div
            class="border-t border-white border-opacity-50 pt-2 px-2 w-full flex flex-col items-center justify-center"
        >
            <button onclick={() => onPageChange('settings')}>
                <Icon name="settings" class="text-2xl" />
            </button>
        </div>
    </div>
</div>
