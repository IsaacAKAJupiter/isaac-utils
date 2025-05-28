<script lang="ts">
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import {
        cursorPosition,
        getCurrentWindow,
        LogicalSize,
        PhysicalPosition,
    } from '@tauri-apps/api/window';
    import { onDestroy, onMount, tick } from 'svelte';
    import Icon from '../../components/icon.svelte';
    import type { Config } from '../../util/config';
    import { getFormattedDate } from '../../util/date';

    let unlisten: UnlistenFn;
    let currentDisplay: {
        unix: number;
        formatted: string;
        fetchedIn: string;
    } | null = $state(null);
    let displayDiv = $state<HTMLDivElement>();

    async function updateWindowProperties() {
        // Wait for svelte to update UI.
        await tick();

        // Get the size of the div.
        const size =
            displayDiv?.children[0].children[0].getBoundingClientRect();
        if (!size) return;

        // Get current window and handle.
        const window = getCurrentWindow();
        const cursor = await cursorPosition();
        await window.setPosition(
            new PhysicalPosition(cursor.x + 20, cursor.y + 20)
        );
        await window.setSize(new LogicalSize(size.width, size.height));
        await window.show();
        await window.setFocus();
    }

    async function hideWindow() {
        const window = getCurrentWindow();
        await window.hide();
    }

    onMount(async () => {
        unlisten = await listen<{
            number: number;
            config: Config;
        }>('e_unix_popup', ({ payload }) => {
            // Set the display values.
            const { formatted, fetchedIn } = getFormattedDate(
                payload.number,
                payload.config
            );
            currentDisplay = { unix: payload.number, formatted, fetchedIn };

            // Update window properties.
            updateWindowProperties();
        });

        const innerDiv = displayDiv?.children[0] as HTMLDivElement | undefined;
        if (innerDiv) {
            innerDiv.style.width = `${screen.width}px`;
        }
    });

    onDestroy(() => {
        unlisten();
    });
</script>

<div bind:this={displayDiv} class="overflow-hidden">
    <div data-full-width>
        <div class="relative w-fit flex flex-col p-2 border border-white">
            <button class="absolute top-0 right-0" onclick={() => hideWindow()}>
                <Icon name="x" />
            </button>

            {#if currentDisplay}
                <p class="w-fit pt-2">{currentDisplay.formatted}</p>
                <p class="text-xs self-end">
                    Converted <i>{currentDisplay.unix}</i> using
                    <i>{currentDisplay.fetchedIn}</i>.
                </p>
            {:else}
                <p class="pt-2">Loading...</p>
            {/if}
        </div>
    </div>
</div>
