<script lang="ts">
    import { onMount } from 'svelte';
    import { addAlert } from '../../../stores/alert';
    import { configStore } from '../../../stores/config';
    import { p2pFilesSending, p2pTextSent } from '../../../stores/p2p';
    import { getConfigCopy, writeConfig } from '../../../util/config';
    import { formatBytes } from '../../../util/format';
    import { sendFile, sendText } from '../../../util/ws';
    import Progress from '../../progress.svelte';
    import Tabs from '../../tabs.svelte';

    let file = $state<FileList>();
    let filePeer = $state<string>('');
    let fileSendToCustomIP = $state<boolean>();
    let fileChosenContact = $state<string>('');
    let fileContactName = $state<string>();
    let textPeer = $state<string>('');
    let textSendToCustomIP = $state<boolean>();
    let textChosenContact = $state<string>('');
    let textContactName = $state<string>();
    let text = $state<string>('');
    let activeTabValue = $state<number>(0);

    async function startSendFile() {
        if (!file || file.length !== 1) {
            addAlert({
                type: 'error',
                message: 'Invalid (or no) file.',
                timeout: 5000,
                dismissible: true,
            });
            return;
        }

        if (
            (!fileSendToCustomIP && !fileChosenContact) ||
            (fileSendToCustomIP && !filePeer)
        ) {
            addAlert({
                type: 'error',
                message: 'No peer given.',
                timeout: 5000,
                dismissible: true,
            });
            return;
        }

        sendFile(!fileSendToCustomIP ? fileChosenContact : filePeer, file[0]);

        if (fileSendToCustomIP && fileContactName) {
            if ($configStore) {
                let newConfig = getConfigCopy($configStore);

                const contactIndex = newConfig.p2p.contacts.findIndex(
                    (c) => c.ip == filePeer
                );
                if (contactIndex != -1) {
                    newConfig.p2p.contacts.splice(contactIndex, 1);
                }
                newConfig.p2p.contacts = [
                    ...newConfig.p2p.contacts,
                    { name: fileContactName, ip: filePeer },
                ];

                configStore.set(newConfig);
                await writeConfig(newConfig);
            } else {
                addAlert({
                    type: 'info',
                    message: 'Could not save the contact in the config.',
                    dismissible: true,
                    timeout: 5000,
                });
            }
        }

        fileSendToCustomIP = false;
        fileChosenContact = '';
        filePeer = '';
        fileContactName = '';
    }

    async function startSendText() {
        if (
            (!textSendToCustomIP && !textChosenContact) ||
            (textSendToCustomIP && !textPeer)
        ) {
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

        if (textSendToCustomIP && textContactName) {
            if ($configStore) {
                let newConfig = getConfigCopy($configStore);

                const contactIndex = newConfig.p2p.contacts.findIndex(
                    (c) => c.ip == textPeer
                );
                if (contactIndex != -1) {
                    newConfig.p2p.contacts.splice(contactIndex, 1);
                }
                newConfig.p2p.contacts = [
                    ...newConfig.p2p.contacts,
                    { name: textContactName, ip: textPeer },
                ];

                configStore.set(newConfig);
                await writeConfig(newConfig);
            } else {
                addAlert({
                    type: 'info',
                    message: 'Could not save the contact in the config.',
                    dismissible: true,
                    timeout: 5000,
                });
            }
        }

        const result = await sendText(
            !textSendToCustomIP ? textChosenContact : textPeer,
            text
        );
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

        textSendToCustomIP = false;
        textChosenContact = '';
        textPeer = '';
        textContactName = '';
    }

    function peerContact(ip: string) {
        return ($configStore?.p2p.contacts ?? []).find((c) => c.ip === ip);
    }

    onMount(() => {
        const contacts = $configStore?.p2p.contacts ?? [];
        fileSendToCustomIP = contacts.length < 1;
        fileChosenContact = contacts.length > 0 ? contacts[0].ip : '';
    });
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
                        <div class="flex space-x-2 items-center mb-4 max-w-96">
                            <p>Contact:</p>
                            <select
                                class="input !w-auto flex-1"
                                bind:value={fileChosenContact}
                            >
                                {#each $configStore?.p2p.contacts ?? [] as contact}
                                    <option value={contact.ip}>
                                        {contact.name}
                                    </option>
                                {:else}
                                    <option disabled value="">
                                        No Contacts Available
                                    </option>
                                {/each}
                            </select>
                        </div>

                        <div class="mb-4">
                            <label class="flex space-x-2 items-center">
                                <input
                                    type="checkbox"
                                    class="checkbox"
                                    bind:checked={fileSendToCustomIP}
                                />
                                <span>Send To Non-Contact</span>
                            </label>
                        </div>

                        {#if fileSendToCustomIP}
                            <div
                                class="flex space-x-2 items-center mb-4 max-w-96"
                            >
                                <p class="w-28">Custom IP:</p>
                                <input
                                    class="input !w-auto flex-1"
                                    type="text"
                                    bind:value={filePeer}
                                />
                            </div>
                            <div
                                class="flex space-x-2 items-center mb-4 max-w-96"
                            >
                                <div class="flex items-center space-x-1 w-28">
                                    <div>
                                        <p>Contact Name</p>
                                        <p class="text-xs text-right">
                                            (Optional)
                                        </p>
                                    </div>
                                    <p>:</p>
                                </div>
                                <input
                                    class="input !w-auto flex-1"
                                    type="text"
                                    bind:value={fileContactName}
                                />
                            </div>
                        {/if}

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
                                    <td class="text-center">
                                        {peerContact(file.peer)?.name ??
                                            file.peer}
                                    </td>
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
                        <div class="flex space-x-2 items-center mb-4 max-w-96">
                            <p>Contact:</p>
                            <select
                                class="input !w-auto flex-1"
                                bind:value={textChosenContact}
                            >
                                {#each $configStore?.p2p.contacts ?? [] as contact}
                                    <option value={contact.ip}>
                                        {contact.name}
                                    </option>
                                {:else}
                                    <option disabled value="">
                                        No Contacts Available
                                    </option>
                                {/each}
                            </select>
                        </div>

                        <div class="mb-4">
                            <label class="flex space-x-2 items-center">
                                <input
                                    type="checkbox"
                                    class="checkbox"
                                    bind:checked={textSendToCustomIP}
                                />
                                <span>Send To Non-Contact</span>
                            </label>
                        </div>

                        {#if textSendToCustomIP}
                            <div
                                class="flex space-x-2 items-center mb-4 max-w-96"
                            >
                                <p class="w-28">Custom IP:</p>
                                <input
                                    class="input !w-auto flex-1"
                                    type="text"
                                    bind:value={textPeer}
                                />
                            </div>
                            <div
                                class="flex space-x-2 items-center mb-4 max-w-96"
                            >
                                <div class="flex items-center space-x-1 w-28">
                                    <div>
                                        <p>Contact Name</p>
                                        <p class="text-xs text-right">
                                            (Optional)
                                        </p>
                                    </div>
                                    <p>:</p>
                                </div>
                                <input
                                    class="input !w-auto flex-1"
                                    type="text"
                                    bind:value={textContactName}
                                />
                            </div>
                        {/if}

                        <div class="flex space-x-2">
                            <p>Text:</p>
                            <textarea class="input" rows="5" bind:value={text}
                            ></textarea>
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
