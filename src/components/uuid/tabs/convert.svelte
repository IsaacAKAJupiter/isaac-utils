<script lang="ts">
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';
    import { v1, v3, v4, v5, v6, v7, validate } from 'uuid';
    import { configStore } from '../../../stores/config';
    import { copy } from '../../../util/tauri';
    import Icon from '../../icon.svelte';

    const UUID_SETTINGS: {
        v1: () => string;
        v3: typeof v3;
        v4: () => string;
        v5: typeof v5;
        v6: () => string;
        v7: () => string;
    } = {
        v1: v1,
        v3: v3,
        v4: v4,
        v5: v5,
        v6: v6,
        v7: v7,
    };

    let version = $state<string>('v4');
    let namespace = $state<string>();
    let namespaceState = $state<'CLEAN' | 'INVALID_NAMESPACE' | 'NO_VALUE'>(
        'CLEAN'
    );
    let namespaceValue = $state<string>();
    let lastUUID = $state<string>();

    onMount(() => {
        namespace = $configStore?.uuid?.namespace;
        version = $configStore?.uuid?.version ?? 'v4';
    });

    function generate() {
        if (!(version in UUID_SETTINGS)) {
            return;
        }

        if (version !== 'v3' && version !== 'v5') {
            namespaceState = 'CLEAN';
            lastUUID = (
                UUID_SETTINGS[
                    version as keyof typeof UUID_SETTINGS
                ] as () => string
            )();
            return;
        }

        if (!namespace || !validate(namespace)) {
            namespaceState = 'INVALID_NAMESPACE';
            return;
        }

        if (!namespaceValue) {
            namespaceState = 'NO_VALUE';
            return;
        }

        namespaceState = 'CLEAN';
        lastUUID = UUID_SETTINGS[version as keyof typeof UUID_SETTINGS](
            namespaceValue,
            namespace
        );
    }
</script>

<div class="p-4 bg-accent">
    <div class="mb-8">
        <div>
            <div class="flex space-x-2 items-center">
                <p>Version:</p>
                <div>
                    <select class="input" bind:value={version}>
                        {#each Object.keys(UUID_SETTINGS) as key}
                            <option value={key}>{key}</option>
                        {/each}
                    </select>
                </div>
            </div>
        </div>
        <div class="mt-4 flex space-x-2 items-center">
            <button class="main-btn" onclick={generate}>Generate</button>
        </div>
        {#if lastUUID}
            <div in:fade out:fade class="flex space-x-2 items-center">
                <p class="text-6xl">=</p>
                <div class="flex flex-col">
                    <div class="flex items-center space-x-2">
                        <p>{lastUUID}</p>
                        <button
                            class="cursor-pointer"
                            onclick={() => copy(lastUUID ?? '', true)}
                        >
                            <Icon name="copy" />
                        </button>
                    </div>
                </div>
            </div>
        {/if}
    </div>
</div>
