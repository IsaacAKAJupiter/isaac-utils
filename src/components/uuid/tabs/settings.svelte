<script lang="ts">
    import { onMount } from 'svelte';
    import { validate } from 'uuid';
    import { addAlert } from '../../../stores/alert';
    import { configStore } from '../../../stores/config';
    import {
        defaultConfig,
        getConfigCopy,
        writeConfig,
    } from '../../../util/config';

    let namespace = $state<string>();
    let version = $state(defaultConfig().uuid.version);

    async function saveNamespace() {
        if (!$configStore) return;

        if (namespace && !validate(namespace)) {
            addAlert({
                type: 'error',
                message: 'Invalid namespace, must be a valid UUID.',
                timeout: 5000,
            });
            return;
        }

        // Update config.
        let newConfig = getConfigCopy($configStore);
        newConfig.uuid.namespace = namespace || undefined;
        configStore.set(newConfig);
        await writeConfig(newConfig);

        addAlert({
            type: 'success',
            message: 'Saved settings!',
            timeout: 5000,
        });
    }

    async function saveVersion() {
        if (!$configStore) return;

        // Update config.
        let newConfig = getConfigCopy($configStore);
        newConfig.uuid.version = version;
        configStore.set(newConfig);
        await writeConfig(newConfig);

        addAlert({
            type: 'success',
            message: 'Saved settings!',
            timeout: 5000,
        });
    }

    onMount(() => {
        namespace = $configStore?.uuid?.namespace;
        version = $configStore?.uuid?.version ?? 'v4';
    });
</script>

<div class="p-4 bg-accent">
    <div class="mb-8">
        <div class="mb-8">
            <p>Namespace</p>
            <p class="text-xs">
                This is the default namespace used for v3/v5 UUID generation.
            </p>
            <div class="flex space-x-2 items-center">
                <input
                    class="input max-w-80"
                    type="text"
                    bind:value={namespace}
                />
                <button class="main-btn" onclick={saveNamespace}>Update</button>
            </div>
        </div>

        <div>
            <p>Default Version</p>
            <p class="text-xs">
                This is the default version used for generating UUIDs.
            </p>
            <div class="flex space-x-2 items-center">
                <div>
                    <select
                        class="input"
                        bind:value={version}
                        onchange={saveVersion}
                    >
                        <option value="v1">v1</option>
                        <option value="v3">v3</option>
                        <option value="v4">v4</option>
                        <option value="v5">v5</option>
                        <option value="v6">v6</option>
                        <option value="v7">v7</option>
                    </select>
                </div>
            </div>
        </div>
    </div>
</div>
