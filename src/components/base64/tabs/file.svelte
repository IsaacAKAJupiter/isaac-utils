<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { save } from '@tauri-apps/plugin-dialog';
    import { fileTypeFromBuffer } from 'file-type';
    import mimeLib from 'mime';
    import { fade } from 'svelte/transition';
    import { addAlert } from '../../../stores/alert';
    import { formatBytes } from '../../../util/format';
    import { copy } from '../../../util/tauri';
    import Icon from '../../icon.svelte';
    import Tabs from '../../tabs.svelte';

    let activeTabValue = $state<number>(0);
    let file = $state<FileList>();
    let outputFormat = $state<'dataURI' | 'plain'>('dataURI');
    let output = $state<string>();
    let base64 = $state<string>('');
    let fileOutput = $state<File>();

    function fileToBase64() {
        if (!file || file.length !== 1 || !outputFormat) {
            addAlert({
                type: 'error',
                message: 'No file given or no output format specified.',
                dismissible: true,
                timeout: 7500,
            });
            return;
        }

        var reader = new FileReader();
        reader.readAsDataURL(file[0]);
        reader.onload = function () {
            const result =
                typeof reader.result == 'string' ? reader.result : undefined;
            if (!result) return;

            output =
                outputFormat === 'dataURI'
                    ? result
                    : result.replace(/^data:[^;]+;base64,/, '');
        };
        reader.onerror = function (error) {
            addAlert({
                type: 'error',
                message: `Failed converting with error: ${error}.`,
                dismissible: true,
                timeout: 7500,
            });
        };
    }

    async function base64ToFile() {
        if (!base64) {
            addAlert({
                type: 'error',
                message: 'No base64 text given.',
                dismissible: true,
                timeout: 7500,
            });
            return;
        }

        const split = base64.split(',');
        const b64Mime =
            split.length > 1 ? split[0].match(/:(.+);/)?.[1] : undefined;
        const bytes = atob(split[split.length - 1]);
        let len = bytes.length;
        let u8Arr = new Uint8Array(len);
        while (len--) {
            u8Arr[len] = bytes.charCodeAt(len);
        }

        const fileType = await fileTypeFromBuffer(u8Arr);
        const extension = fileType
            ? fileType.ext
            : b64Mime
              ? mimeLib.getExtension(b64Mime)
              : null;
        const mime = fileType ? fileType.mime : b64Mime;
        const name = `unnamed${extension ? `.${extension}` : ''}`;
        const file = new File([u8Arr], name, { type: mime });

        fileOutput = file;
    }

    async function saveFile() {
        if (!fileOutput) return;

        const path = await save({ defaultPath: fileOutput.name });
        if (!path) return;

        const data = await fileOutput.arrayBuffer();

        const success = await invoke<boolean>('c_save_file', {
            path: path,
            name: '',
            data,
        });
        if (!success) {
            addAlert({
                type: 'error',
                message: `Failed saving file.`,
                dismissible: true,
                timeout: 5000,
            });
            return;
        }

        addAlert({
            type: 'success',
            message: 'Successfully saved file.',
            dismissible: true,
            timeout: 5000,
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
                        label: 'To',
                        value: 0,
                    },
                    {
                        label: 'From',
                        value: 1,
                    },
                ]}
                bind:activeTabValue
            />
        </div>
        <div class="flex-1 border-l border-white p-4">
            {#if activeTabValue == 0}
                <div>
                    <div>
                        <div class="mb-2 flex space-x-2 max-w-2xl">
                            <p class="w-32">Text:</p>
                            <input type="file" bind:files={file} />
                        </div>

                        <div class="mb-2 flex space-x-2 items-center">
                            <p class="w-32">Output Format:</p>
                            <div>
                                <select class="input" bind:value={outputFormat}>
                                    <option value="dataURI">Data URI</option>
                                    <option value="plain">Plain Text</option>
                                </select>
                            </div>
                        </div>
                    </div>

                    <div class="mt-4 flex space-x-2 items-center">
                        <button class="main-btn" onclick={fileToBase64}>
                            Convert
                        </button>
                    </div>

                    {#if output}
                        <div
                            in:fade
                            out:fade
                            class="mt-2 flex space-x-2 items-center max-w-2xl"
                        >
                            <p class="text-6xl">=</p>
                            <div class="flex-1 flex flex-col">
                                <div class="flex items-center space-x-2">
                                    <textarea
                                        readonly
                                        class="input"
                                        rows="5"
                                        autocomplete="off"
                                        autocorrect="off"
                                        autocapitalize="off"
                                        spellcheck="false">{output}</textarea
                                    >
                                    <button
                                        class="cursor-pointer"
                                        onclick={() => copy(output ?? '', true)}
                                    >
                                        <Icon name="copy" class="text-2xl" />
                                    </button>
                                </div>
                            </div>
                        </div>
                    {/if}
                </div>
            {/if}

            {#if activeTabValue == 1}
                <div>
                    <div>
                        <div class="mb-2 flex space-x-2 max-w-2xl">
                            <p class="w-32">Base64:</p>
                            <div class="flex-1">
                                <textarea
                                    class="input"
                                    rows="5"
                                    autocomplete="off"
                                    autocorrect="off"
                                    autocapitalize="off"
                                    spellcheck="false"
                                    bind:value={base64}
                                ></textarea>
                            </div>
                        </div>
                    </div>

                    <div class="mt-4 flex space-x-2 items-center">
                        <button class="main-btn" onclick={base64ToFile}>
                            Convert
                        </button>
                    </div>

                    {#if fileOutput}
                        <div
                            in:fade
                            out:fade
                            class="mt-2 flex space-x-2 items-center max-w-2xl"
                        >
                            <p class="text-6xl">=</p>
                            <div class="flex-1 flex flex-col">
                                <div>
                                    <p>Mime Type: {fileOutput.type}</p>
                                    <p>Size: {formatBytes(fileOutput.size)}</p>
                                    <button
                                        class="main-btn"
                                        onclick={() => saveFile()}
                                    >
                                        Download
                                    </button>
                                </div>
                            </div>
                        </div>
                    {/if}
                </div>
            {/if}
        </div>
    </div>
</div>
