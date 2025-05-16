<script lang="ts">
    import { downloadDir } from '@tauri-apps/api/path';
    import { save } from '@tauri-apps/plugin-dialog';
    import { writeTextFile } from '@tauri-apps/plugin-fs';
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';
    import { v1, v3, v4, v5, v6, v7, validate } from 'uuid';
    import { addAlert } from '../../../stores/alert';
    import { configStore } from '../../../stores/config';
    import { copy } from '../../../util/tauri';
    import Icon from '../../icon.svelte';
    import Radio from '../../radio.svelte';

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
    let namespaceValue = $state<string>();
    let generationType = $state<'single' | 'multi'>('single');
    let bulkGenerationAmount = $state<number>(1);
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
            generateNoNamespace();
            return;
        }

        generateWithNamespace();
    }

    async function generateNoNamespace() {
        if (
            generationType == 'multi' &&
            (!bulkGenerationAmount || bulkGenerationAmount < 1)
        ) {
            addAlert({
                type: 'error',
                message: 'Missing or invalid bulk generation amount.',
                timeout: 5000,
            });
            return;
        }

        if (generationType == 'single') {
            lastUUID = (
                UUID_SETTINGS[
                    version as keyof typeof UUID_SETTINGS
                ] as () => string
            )();
            return;
        }

        const uuids = new Array(bulkGenerationAmount)
            .fill(null)
            .map((_) =>
                (
                    UUID_SETTINGS[
                        version as keyof typeof UUID_SETTINGS
                    ] as () => string
                )()
            );

        const path = await save({
            defaultPath: (await downloadDir()) + '/' + 'uuids',
            filters: [
                {
                    name: 'Text Files',
                    extensions: ['txt'],
                },
            ],
        });
        if (!path) return;

        await writeTextFile(path, uuids.join('\n'));
    }

    function generateWithNamespace() {
        if (!namespace || !validate(namespace)) {
            addAlert({
                type: 'error',
                message: 'Invalid namespace, must be a UUID.',
                timeout: 5000,
            });
            return;
        }

        if (!namespaceValue) {
            addAlert({
                type: 'error',
                message: 'Missing value to encode into a UUID.',
                timeout: 5000,
            });
            return;
        }

        lastUUID = UUID_SETTINGS[version as keyof typeof UUID_SETTINGS](
            namespaceValue,
            namespace
        );
    }
</script>

<div class="p-4 bg-accent">
    <div>
        <div>
            <div class="mb-2 flex space-x-2 items-center">
                <p class="w-32">Version:</p>
                <div>
                    <select class="input" bind:value={version}>
                        {#each Object.keys(UUID_SETTINGS) as key}
                            <option value={key}>{key}</option>
                        {/each}
                    </select>
                </div>
            </div>
            {#if version == 'v3' || version == 'v5'}
                <div class="mb-2 flex space-x-2 items-center">
                    <p class="w-32">Namespace:</p>
                    <div class="flex-1 max-w-80">
                        <input class="input" bind:value={namespace} />
                    </div>
                </div>
                <div class="flex space-x-2 items-center">
                    <p class="w-32">Value:</p>
                    <div class="flex-1 max-w-80">
                        <input class="input" bind:value={namespaceValue} />
                    </div>
                </div>
            {:else}
                <div class="mb-2 flex space-x-2 items-center">
                    <p class="w-32">Generation Type:</p>
                    <div class="flex items-center space-x-4">
                        <Radio
                            name="generation-type"
                            id="generation-type-single"
                            value="single"
                            label="Single"
                            bind:group={generationType}
                        />
                        <Radio
                            name="generation-type"
                            id="generation-type-multi"
                            value="multi"
                            label="Multiple"
                            bind:group={generationType}
                        />
                    </div>
                </div>
                {#if generationType == 'multi'}
                    <div class="flex space-x-2 items-center">
                        <p class="w-32">How Many:</p>
                        <div class="flex-1 max-w-80">
                            <input
                                class="input"
                                type="number"
                                bind:value={bulkGenerationAmount}
                            />
                        </div>
                    </div>
                {/if}
            {/if}
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
