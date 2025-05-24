<script lang="ts">
    import { emit } from '@tauri-apps/api/event';
    import { downloadDir } from '@tauri-apps/api/path';
    import { save } from '@tauri-apps/plugin-dialog';
    import { revealItemInDir } from '@tauri-apps/plugin-opener';
    import { addAlert } from '../../../stores/alert';
    import {
        p2pFilesReceiving,
        p2pTextReceived,
        type P2PFileReceive,
    } from '../../../stores/p2p';
    import { formatBytes } from '../../../util/format';
    import Icon from '../../icon.svelte';
    import Progress from '../../progress.svelte';
    import Tabs from '../../tabs.svelte';

    let activeTabValue = $state<number>(0);

    async function declineFile(file: P2PFileReceive) {
        p2pFilesReceiving.update((v) =>
            v.map((f) => {
                if (f.id !== file.id) return f;

                return { ...f, status: 'declined' };
            })
        );

        try {
            await emit(`e_p2p_ask_file_${file.id}`, `0<|>_`);
        } catch (e) {
            addAlert({
                type: 'error',
                message: `Failed declining with error: ${e}.`,
                timeout: 10000,
                dismissible: true,
            });
        }
    }

    async function acceptFile(file: P2PFileReceive) {
        const result = await save({
            defaultPath: (await downloadDir()) + '/' + file.name,
        });
        if (!result) return;

        p2pFilesReceiving.update((v) =>
            v.map((f) => {
                if (f.id !== file.id) return f;

                return { ...f, savePath: result, status: 'sendingData' };
            })
        );

        try {
            await emit(`e_p2p_ask_file_${file.id}`, `1<|>${result}`);
        } catch (e) {
            addAlert({
                type: 'error',
                message: `Failed declining with error: ${e}.`,
                timeout: 10000,
                dismissible: true,
            });
        }
    }

    async function openFile(path?: string) {
        if (!path) return;

        try {
            await revealItemInDir(path);
        } catch (e) {
            addAlert({
                type: 'error',
                message: `Failed showing in directory with error: ${e}.`,
                timeout: 10000,
                dismissible: true,
            });
        }
    }
</script>

<div class="bg-accent">
    <div class="flex">
        <div class="bg-bg">
            <Tabs
                orientation="vertical"
                spanClasses="w-8"
                items={[
                    {
                        label: 'File',
                        value: 0,
                    },
                    {
                        label: 'Text',
                        value: 1,
                    },
                ]}
                bind:activeTabValue
            />
        </div>
        <div class="flex-1 border-l border-white p-4">
            {#if activeTabValue == 0}
                {#each $p2pFilesReceiving as file}
                    {#if file.status === 'waitingForAcceptOrDecline'}
                        <div class="mb-4">
                            <p>New File!</p>
                            <p>From: {file.peer}</p>
                            <p>Name: {file.name}</p>
                            <p>Size: {formatBytes(file.size)}</p>
                            <div class="mt-2 flex space-x-2 items-center">
                                <button
                                    class="red-btn"
                                    onclick={() => declineFile(file)}
                                >
                                    Decline
                                </button>
                                <button
                                    class="main-btn"
                                    onclick={() => acceptFile(file)}
                                >
                                    Accept
                                </button>
                            </div>
                        </div>
                    {/if}
                {/each}

                <div class="mb-4">
                    <p class="mb-2">Current Files</p>
                    <table class="table-fixed w-full">
                        <thead>
                            <tr>
                                <th class="text-left">Name</th>
                                <th>Peer</th>
                                <th>Progress</th>
                                <th>&nbsp;</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each $p2pFilesReceiving as file}
                                {#if file.status !== 'waitingForAcceptOrDecline'}
                                    <tr>
                                        <td class="text-left">{file.name}</td>
                                        <td class="text-center">{file.peer}</td>
                                        <td>
                                            {#if file.status == 'error'}
                                                <p class="text-red-600">
                                                    Error!
                                                </p>
                                            {:else if file.status == 'declined'}
                                                <p class="text-orange-600">
                                                    Declined!
                                                </p>
                                            {:else if file.status == 'sendingData' || file.status == 'finished'}
                                                <div
                                                    class="{file.status ==
                                                    'sendingData'
                                                        ? 'text-primary'
                                                        : 'text-green-600'} h-4"
                                                >
                                                    <Progress
                                                        progress={(file.transferred /
                                                            file.size) *
                                                            100}
                                                        height="100%"
                                                    />
                                                    <p
                                                        class="text-center text-xs"
                                                    >
                                                        {formatBytes(
                                                            file.transferred
                                                        )} /
                                                        {formatBytes(file.size)}
                                                    </p>
                                                </div>
                                            {/if}
                                        </td>
                                        <td class="text-right">
                                            <div class="flex justify-end">
                                                <button
                                                    class="w-fit text-2xl cursor-pointer"
                                                    onclick={() =>
                                                        openFile(file.savePath)}
                                                >
                                                    <Icon name="folder" />
                                                </button>
                                            </div>
                                        </td>
                                    </tr>
                                {/if}
                            {/each}
                        </tbody>
                    </table>
                </div>
            {/if}

            {#if activeTabValue == 1}
                <div class="mb-4">
                    <p class="mb-2">Received Text</p>
                    <table class="table-fixed w-full">
                        <thead>
                            <tr>
                                <th class="text-left">Text</th>
                                <th class="text-left">Peer</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each $p2pTextReceived as text}
                                <tr>
                                    <td class="text-left">{text.text}</td>
                                    <td class="text-left">{text.peer}</td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            {/if}
        </div>
    </div>
</div>
