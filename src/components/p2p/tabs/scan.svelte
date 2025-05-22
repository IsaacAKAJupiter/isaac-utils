<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { copy } from '../../../util/tauri';
    import Icon from '../../icon.svelte';
    import Loader from '../../loader.svelte';

    let scanning = $state(false);
    let results = $state<{ ip: string; hostname?: string }[]>();
    let fetchingHostnames = $state<string[]>([]);

    async function scan() {
        scanning = true;

        const { results: invokeResults } = await invoke<{
            results: { ip: string }[];
        }>('c_check_ports');
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
        <button class="main-btn" onclick={() => scan()}>Scan</button>
    </div>

    <div class="mt-4">
        {#if scanning}
            <Loader />
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
