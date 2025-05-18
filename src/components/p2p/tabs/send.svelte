<script lang="ts">
    import { addAlert } from '../../../stores/alert';
    import { p2pFilesSending, p2pTextSent } from '../../../stores/p2p';
    import { sendFile, sendText } from '../../../util/ws';

    let file = $state<FileList>();
    let filePeer = $state<string>('');
    let textPeer = $state<string>('');
    let text = $state<string>('');

    // TODO: Now that ports can be checked, we should work on the impl for the actual listener/server stuff (probably toggle enable in that tab).
    // Results is an array of arrays (ip, success). Filter if success is true and map to IP.
    // const results =
    //     (
    //         await invoke<{ results: [string, boolean][] | null }>(
    //             'c_check_ports'
    //         )
    //     ).results ?? [];
    // console.log(results.filter((r) => r[1]).map((r) => r[0]));

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
            message: `Failed sending text with error: ${result.error}.`,
            timeout: 10000,
            dismissible: true,
        });
    }
</script>

<div class="p-4 bg-accent">
    <div>
        <div class="mb-4">
            <p class="mb-2">Send File</p>
            <div>
                <div class="flex space-x-2 items-center mb-2">
                    <p>Peer:</p>
                    <input class="input" type="text" bind:value={filePeer} />
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

        <div class="mb-4">
            <p class="mb-2">Sent Files</p>
            <table class="table-fixed w-full">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Peer</th>
                        <th>Size</th>
                        <th>Sent</th>
                        <th>Percent</th>
                    </tr>
                </thead>
                <tbody>
                    {#each $p2pFilesSending as file}
                        <tr>
                            <td>{file.file.name}</td>
                            <td>{file.peer}</td>
                            <td>{file.file.size}</td>
                            <td>{file.transferred}</td>
                            <td>{(file.transferred / file.file.size) * 100}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>

        <div class="mb-4">
            <p class="mb-2">Send Text</p>
            <div>
                <div class="flex space-x-2 items-center mb-2">
                    <p>Peer</p>
                    <input class="input" type="text" bind:value={textPeer} />
                </div>
                <div class="flex space-x-2 items-center">
                    <p>Text</p>
                    <input class="input" type="text" bind:value={text} />
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
    </div>
</div>
