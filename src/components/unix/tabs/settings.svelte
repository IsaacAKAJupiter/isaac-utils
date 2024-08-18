<script lang="ts">
    import { onMount } from 'svelte';
    import {
        configStore,
        configShortcutLastChange,
    } from '../../../stores/config';
    import {
        defaultConfig,
        writeConfig,
        getConfigCopy,
        type Config,
        type ConfigUnixFetchFormat,
    } from '../../../util/config';
    import { addAlert } from '../../../stores/alert';
    import { invoke } from '@tauri-apps/api/core';
    import { getAvailableTimeZones, getUserTimeZone } from '../../../util/date';

    // Variables.
    let unixToReadableShortcut: string =
        defaultConfig().shortcuts.unixToReadable;
    let changingShortcut: boolean = false;
    let savingShortcut: boolean = false;
    let timeZones: string[];
    let timeZone: string;
    let fetchFormat: ConfigUnixFetchFormat;

    async function handleKeyEvent(event: KeyboardEvent) {
        event.stopPropagation();
        event.preventDefault();

        // If key not valid.
        if (['Control', 'Shift', 'Alt', 'Meta'].includes(event.key)) return;

        const keys: string[] = [];
        if (event.ctrlKey) keys.push('Ctrl');
        if (event.metaKey) keys.push('Cmd');
        if (event.altKey) keys.push('Alt');
        if (event.shiftKey) keys.push('Shift');
        keys.push(event.code);

        // Only update the shortcut if an actual key was pressed alongside modifiers.
        const shortcut = keys.join('+');

        const valid = await invoke<boolean>('c_valid_shortcut', {
            shortcut,
        });
        if (!valid) {
            addAlert({
                type: 'error',
                message: `Invalid key (${event.code}) pressed. See https://github.com/tauri-apps/global-hotkey/blob/dev/src/hotkey.rs#L238 for all possible keys.`,
                timeout: 7500,
            });
            return;
        }

        unixToReadableShortcut = shortcut;
        saveNewShortcut();
    }

    async function saveNewShortcut() {
        // If not new.
        if (!$configStore || savingShortcut) return;

        savingShortcut = true;

        // Update config.
        let newConfig = getConfigCopy($configStore);
        newConfig.shortcuts.unixToReadable = unixToReadableShortcut;
        configStore.set(newConfig);
        await writeConfig(newConfig);

        // Make sure to update the store for the updated shortcuts.
        configShortcutLastChange.set(Date.now());

        addAlert({
            type: 'success',
            message: 'Saved shortcut!',
            timeout: 5000,
        });

        savingShortcut = false;
        changingShortcut = false;
    }

    function changeShortcut() {
        changingShortcut = true;
        unixToReadableShortcut = '';
    }

    function resetShortcut() {
        unixToReadableShortcut =
            $configStore?.shortcuts.unixToReadable ??
            defaultConfig().shortcuts.unixToReadable;
        savingShortcut = false;
        changingShortcut = false;
    }

    async function saveTimeZone() {
        // If the time zone is not valid.
        if (!timeZones.includes(timeZone) || !$configStore) return;

        // Update config.
        let newConfig = getConfigCopy($configStore);
        newConfig.unix.timeZone = timeZone;
        configStore.set(newConfig);
        await writeConfig(newConfig);

        addAlert({
            type: 'success',
            message: 'Saved time zone!',
            timeout: 5000,
        });
    }

    async function saveFetchFormat() {
        if (!$configStore) return;

        // Update config.
        let newConfig = getConfigCopy($configStore);
        newConfig.unix.fetchFormat = fetchFormat;
        configStore.set(newConfig);
        await writeConfig(newConfig);

        addAlert({
            type: 'success',
            message: 'Saved fetch format!',
            timeout: 5000,
        });
    }

    onMount(() => {
        // If unix to readable in config, set it.
        if ($configStore?.shortcuts.unixToReadable) {
            unixToReadableShortcut = $configStore.shortcuts.unixToReadable;
        }

        // Get list of time zones and current time zone.
        timeZones = getAvailableTimeZones().concat(['UTC / GMT']);
        timeZone = $configStore?.unix?.timeZone ?? getUserTimeZone();

        // Set fetch format.
        fetchFormat = $configStore?.unix?.fetchFormat ?? 'auto';
    });
</script>

<svelte:window on:keydown={(e) => changingShortcut && handleKeyEvent(e)} />

<div class="p-4 bg-accent">
    <div class="mb-8">
        <p class="text-lg">Unix To Readable Popup</p>
        <p class="text-xs mb-8">
            Below are the settings for the popup for converting unix numbers to
            a human readable date format.
        </p>

        <div class="mb-8">
            <p>Shortcut</p>
            <div class="flex space-x-2 items-center">
                <p class="text-sm">
                    {unixToReadableShortcut || 'Waiting for shortcut...'}
                </p>
                {#if !changingShortcut}
                    <button class="main-btn" on:click={changeShortcut}>
                        Update
                    </button>
                {:else}
                    <button class="red-btn" on:click={resetShortcut}>
                        Cancel
                    </button>
                {/if}
            </div>
        </div>

        <div class="mb-8">
            <p>Conversion Time Zone</p>
            <div class="flex space-x-2 items-center">
                <div>
                    <input
                        class="input"
                        type="text"
                        list="timeZones"
                        bind:value={timeZone}
                        on:change={saveTimeZone}
                    />
                    {#if timeZones}
                        <datalist id="timeZones">
                            {#each timeZones as t}
                                <option value={t}>{t}</option>
                            {/each}
                        </datalist>
                    {/if}
                </div>
            </div>
        </div>

        <div>
            <p>Fetch Format</p>
            <p class="text-xs">
                This is the format that the unix timestamp will be parsed from
                (auto will determine based on length of the timestamp and is
                recommended).
            </p>
            <div class="flex space-x-2 items-center">
                <div>
                    <select
                        class="input"
                        bind:value={fetchFormat}
                        on:change={saveFetchFormat}
                    >
                        <option value="auto">Auto</option>
                        <option value="seconds">Seconds</option>
                        <option value="milliseconds">Milliseconds</option>
                        <option value="microseconds">Microseconds</option>
                        <option value="nanoseconds">Nanoseconds</option>
                    </select>
                </div>
            </div>
        </div>
    </div>
</div>
