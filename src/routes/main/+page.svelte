<script lang="ts">
    import { window } from '@tauri-apps/api';
    import { invoke } from '@tauri-apps/api/core';
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import {
        isRegistered,
        register,
        type ShortcutEvent,
        unregisterAll,
    } from '@tauri-apps/plugin-global-shortcut';
    import { onDestroy, onMount } from 'svelte';
    import type { Unsubscriber } from 'svelte/store';
    import { fade } from 'svelte/transition';
    import Alerts from '../../components/alerts/alerts.svelte';
    import Sidenav from '../../components/sidenav.svelte';
    import UnixBase from '../../components/unix/base.svelte';
    import UUIDBase from '../../components/uuid/base.svelte';
    import { configShortcutLastChange, configStore } from '../../stores/config';
    import { isMinimized } from '../../stores/main-window';
    import { getConfig } from '../../util/config';
    import { checkForAppUpdates } from '../../util/tauri';

    let page = $state<string>('unix');
    let onResizeUnlisten: UnlistenFn;
    let updateCheckUnlisten: UnlistenFn;
    let configShortcutChangeUnsubscriber: Unsubscriber;

    function onUnixToReadableShortcut(event: ShortcutEvent) {
        if (!$configStore) return;

        if (event.state === 'Released') {
            invoke('c_unix_to_readable', { config: $configStore });
        }
    }

    async function registerShortcuts() {
        if (!$configStore) return;

        // Unix to readable shortcut.
        if (!(await isRegistered($configStore.shortcuts.unixToReadable))) {
            await register(
                $configStore.shortcuts.unixToReadable,
                onUnixToReadableShortcut
            );
        }
    }

    async function reregisterShortcuts() {
        // Unregister, register.
        await unregisterAll();
        await registerShortcuts();
    }

    onMount(async () => {
        await getConfig();
        await registerShortcuts();

        // TODO: Now that ports can be checked, we should work on the impl for the actual listener/server stuff (probably toggle enable in that tab).

        // Results is an array of arrays (ip, success). Filter if success is true and map to IP.
        // const results =
        //     (
        //         await invoke<{ results: [string, boolean][] | null }>(
        //             'c_check_ports'
        //         )
        //     ).results ?? [];
        // console.log(results.filter((r) => r[1]).map((r) => r[0]));

        // Listen for shortcut changes.
        configShortcutChangeUnsubscriber =
            configShortcutLastChange.subscribe(reregisterShortcuts);

        // Listen to resize change to get minimized boolean.
        const currentWindow = window.getCurrentWindow();
        isMinimized.set(await currentWindow.isMinimized());
        onResizeUnlisten = await currentWindow.onResized(async () => {
            isMinimized.set(await currentWindow.isMinimized());
        });

        // Listen for update check.
        checkForAppUpdates(false);
        updateCheckUnlisten = await listen('e_check_for_update', () => {
            checkForAppUpdates(true);
        });
    });

    onDestroy(() => {
        onResizeUnlisten?.();
        updateCheckUnlisten?.();
        configShortcutChangeUnsubscriber?.();
    });
</script>

<Alerts />

<div class="flex">
    <Sidenav page={(p) => (page = p)} />

    <main class="flex-1">
        {#if page == 'unix'}
            <div in:fade out:fade>
                <UnixBase />
            </div>
        {:else if page == 'uuid'}
            <div in:fade out:fade>
                <UUIDBase />
            </div>
        {/if}
    </main>
</div>
