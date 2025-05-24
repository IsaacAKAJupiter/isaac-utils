<script lang="ts">
    import { addAlert } from '../../../stores/alert';
    import { p2pFilesSending, p2pTextSent } from '../../../stores/p2p';
    import { formatBytes } from '../../../util/format';
    import { sendFile, sendText } from '../../../util/ws';
    import Progress from '../../progress.svelte';
    import Tabs from '../../tabs.svelte';

    let file = $state<FileList>();
    let filePeer = $state<string>('');
    let textPeer = $state<string>('');
    let text = $state<string>('');
    let activeTabValue = $state<number>(0);

    function startSendFile() {
        if (!file || file.length !== 1) {
            addAlert({
                type: 'error',
                message: 'Invalid (or no) file.',
                timeout: 5000,
                dismissible: true,
            });
            return;
        }

        if (!filePeer) {
            addAlert({
                type: 'error',
                message: 'No peer given.',
                timeout: 5000,
                dismissible: true,
            });
            return;
        }

        sendFile(filePeer, file[0]);
    }

    async function startSendText() {
        if (!textPeer) {
            addAlert({
                type: 'error',
                message: 'No peer given.',
                timeout: 5000,
                dismissible: true,
            });
            return;
        }

        if (!text) {
            addAlert({
                type: 'error',
                message: 'No text given.',
                timeout: 5000,
                dismissible: true,
            });
            return;
        }

        const result = await sendText(textPeer, text);
        if (result.success) {
            addAlert({
                type: 'success',
                message: 'Sent!',
                timeout: 5000,
                dismissible: true,
            });
            return;
        }

        addAlert({
            type: 'error',
            message: `Failed sending text.`,
            timeout: 10000,
            dismissible: true,
        });
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
                <div class="mb-4">
                    <div>
                        <div class="flex space-x-2 items-center mb-2">
                            <p>Peer:</p>
                            <input
                                class="input"
                                type="text"
                                bind:value={filePeer}
                            />
                        </div>
                        <div class="flex space-x-2 items-center">
                            <p>File:</p>
                            <input type="file" bind:files={file} />
                        </div>
                    </div>
                    <div class="mt-4 flex space-x-2 items-center">
                        <button class="main-btn" onclick={startSendFile}>
                            Send File
                        </button>
                    </div>
                </div>

                <hr />

                <div class="mb-4">
                    <p class="mb-2">Sent Files</p>
                    <table class="table-fixed w-full">
                        <thead>
                            <tr>
                                <th>Name</th>
                                <th>Peer</th>
                                <th>Progress</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each $p2pFilesSending as file}
                                <tr>
                                    <td class="text-center">
                                        {file.file.name}
                                    </td>
                                    <td class="text-center">{file.peer}</td>
                                    <td class="text-center">
                                        {#if file.status == 'error'}
                                            <p class="text-red-600">Error!</p>
                                        {:else if file.status == 'declined'}
                                            <p class="text-orange-600">
                                                Declined!
                                            </p>
                                        {:else if file.status == 'closed'}
                                            <p class="text-red-600">Closed!</p>
                                        {:else if file.status == 'waitingForAcceptOrDecline'}
                                            <p class="text-red-600">
                                                Waiting...
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
                                                        file.file.size) *
                                                        100}
                                                    height="100%"
                                                />
                                                <p class="text-center text-xs">
                                                    {formatBytes(
                                                        file.transferred
                                                    )} /
                                                    {formatBytes(
                                                        file.file.size
                                                    )}
                                                </p>
                                            </div>
                                        {/if}
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            {/if}

            {#if activeTabValue == 1}
                <div class="mb-4">
                    <div>
                        <div class="flex space-x-2 items-center mb-2">
                            <p>Peer:</p>
                            <input
                                class="input"
                                type="text"
                                bind:value={textPeer}
                            />
                        </div>
                        <div class="flex space-x-2 items-center">
                            <p>Text:</p>
                            <input
                                class="input"
                                type="text"
                                bind:value={text}
                            />
                        </div>
                    </div>
                    <div class="mt-4 flex space-x-2 items-center">
                        <button class="main-btn" onclick={startSendText}>
                            Send Text
                        </button>
                    </div>
                </div>

                <div class="mb-4">
                    <p class="mb-2">Sent Text</p>
                    <table class="table-fixed w-full">
                        <thead>
                            <tr>
                                <th>Text</th>
                                <th>Peer</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each $p2pTextSent as text}
                                <tr>
                                    <td>{text.text}</td>
                                    <td>{text.peer}</td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            {/if}
        </div>
    </div>
</div>
