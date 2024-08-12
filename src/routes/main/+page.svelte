<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import {
        register,
        isRegistered,
        type ShortcutEvent,
    } from '@tauri-apps/plugin-global-shortcut';
    import { onDestroy, onMount } from 'svelte';
    import { fade } from 'svelte/transition';
    import UnixBase from '../../components/unix/base.svelte';
    import { getConfig } from '../../util/config';
    import type { UnlistenFn } from '@tauri-apps/api/event';
    import { window } from '@tauri-apps/api';
    import { isMinimized } from '../../stores/main-window';
    import Alerts from '../../components/alerts/alerts.svelte';
    import Sidenav from '../../components/sidenav.svelte';

    let config: { [key: string]: any };
    let page: string = 'unix';
    let onResizeUnlisten: UnlistenFn;

    function onUnixToReadableShortcut(event: ShortcutEvent) {
        if (event.state === 'Released') {
            invoke('c_unix_to_readable', { config });
        }
    }

    async function registerShortcuts() {
        // Unix to readable shortcut.
        if (!(await isRegistered(config.shortcuts.unixToReadable))) {
            await register(
                config.shortcuts.unixToReadable,
                onUnixToReadableShortcut
            );
        }
    }

    onMount(async () => {
        config = await getConfig();
        await registerShortcuts();

        const currentWindow = window.getCurrentWindow();
        isMinimized.set(await currentWindow.isMinimized());
        onResizeUnlisten = await currentWindow.onResized(async () => {
            isMinimized.set(await currentWindow.isMinimized());
        });
    });

    onDestroy(() => {
        onResizeUnlisten?.();
    });
</script>

<Alerts />

<div class="flex">
    <Sidenav on:page={(p) => (page = p.detail)} />

    <main class="flex-1">
        {#if page == 'unix'}
            <div in:fade out:fade>
                <UnixBase></UnixBase>
            </div>
        {/if}
    </main>
</div>
