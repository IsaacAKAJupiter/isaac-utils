<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import { onDestroy, onMount } from 'svelte';
    import { addAlert } from '../../../stores/alert';
    import { configStore } from '../../../stores/config';
    import { getConfigCopy, writeConfig } from '../../../util/config';
    import { copy } from '../../../util/tauri';
    import Icon from '../../icon.svelte';
    import Loader from '../../loader.svelte';
    import Progress from '../../progress.svelte';

    let scanning = $state(false);
    let chunkSize = $state<number>(100);
    let resultState = $state<{ total: number; progress: number }>({
        total: 0,
        progress: 0,
    });
    let results = $state<{ ip: string; hostname?: string }[]>();
    let fetchingHostnames = $state<string[]>([]);

    let onScanEventUnlisten: UnlistenFn;

    onMount(async () => {
        chunkSize = $configStore?.p2p.scanSize ?? 100;

        onScanEventUnlisten = await listen<{
            event: 'count' | 'processed';
            data: number;
        }>('e_p2p_scan', (event) => {
            if (event.payload.event == 'count') {
                resultState = {
                    total: event.payload.data,
                    progress: 0,
                };
                return;
            }

            resultState = {
                total: resultState.total,
                progress: resultState.progress + event.payload.data,
            };
        });
    });

    onDestroy(() => {
        onScanEventUnlisten();
    });

    async function scan() {
        if (!chunkSize || chunkSize < 1) {
            addAlert({
                type: 'error',
                message: 'Chunk size must be set and greater than 0.',
                dismissible: true,
                timeout: 5000,
            });
            return;
        }

        scanning = true;

        // Update config.
        if ($configStore) {
            let newConfig = getConfigCopy($configStore);
            newConfig.p2p.scanSize = chunkSize;
            configStore.set(newConfig);
            await writeConfig(newConfig);
        } else {
            addAlert({
                type: 'info',
                message: 'Could not save the chunk size in the config.',
                dismissible: true,
                timeout: 5000,
            });
        }

        const { results: invokeResults } = await invoke<{
            results: { ip: string }[];
        }>('c_check_ports', { size: chunkSize });
        results = invokeResults;

        scanning = false;
    }

    async function getHostname(ip: string) {
        fetchingHostnames = [...fetchingHostnames, ip];

        const hostname = await invoke<string>('c_get_hostname', { ip });
        results = (results ?? []).map((v) =>
            v.ip == ip ? { ...v, hostname } : v
        );

        fetchingHostnames = fetchingHostnames.filter((v) => v != ip);
    }
</script>

<div class="p-4 bg-accent">
    <div>
        <p class="mb-4 text-sm">
            This is for scanning the local network using the default network
            interface for other devices with the port used for P2P interactions.
            Please note that average time for each chunk is 1 second. The best
            chunk size is dependant on your computer as it can take up a
            significant portion of your resources if large chunk size is used
            and a long time if a small chunk size is used. Also note, this
            action cannot be cancelled mid scan and will show you the progress
            as it is happening.
        </p>

        <div class="flex space-x-2 items-center mb-4">
            <p>Chunk Size:</p>
            <div class="flex-1 max-w-80">
                <input class="input" type="number" bind:value={chunkSize} />
            </div>
        </div>

        <button class="main-btn" onclick={() => scan()}>Scan</button>
    </div>

    <div class="mt-4">
        {#if scanning}
            <div class="flex flex-col items-center max-w-32">
                <div class="text-xs">
                    <Loader style="dots" />
                </div>
                <div class="text-primary h-4 w-full my-4">
                    <Progress
                        progress={(resultState.progress / resultState.total) *
                            100}
                        height="100%"
                    />
                    <p class="text-center text-xs">
                        {resultState.progress} /
                        {resultState.total}
                    </p>
                </div>
            </div>
        {:else if results != undefined}
            <table class="table-fixed w-full">
                <thead>
                    <tr>
                        <th class="text-left">IP</th>
                        <th class="text-right">Hostname</th>
                    </tr>
                </thead>
                <tbody>
                    {#each results as result}
                        <tr class="border-t">
                            <td class="text-left py-1">
                                <div class="flex items-center space-x-2">
                                    <p>{result.ip}</p>
                                    <button
                                        class="cursor-pointer"
                                        onclick={() => copy(result.ip, true)}
                                    >
                                        <Icon name="copy" />
                                    </button>
                                </div>
                            </td>
                            <td class="text-right py-1">
                                {#if result.hostname == undefined}
                                    {#if fetchingHostnames.includes(result.ip)}
                                        <div class="flex justify-end">
                                            <Loader
                                                style="dots"
                                                classes="text-sm"
                                            />
                                        </div>
                                    {:else}
                                        <div class="flex justify-end">
                                            <button
                                                class="main-btn !w-auto !px-2"
                                                onclick={() =>
                                                    getHostname(result.ip)}
                                            >
                                                Fetch
                                            </button>
                                        </div>
                                    {/if}
                                {:else}
                                    {result.hostname || 'None Found'}
                                {/if}
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/if}
    </div>
</div>
