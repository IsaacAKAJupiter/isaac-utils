<script lang="ts">
    import { fade } from 'svelte/transition';
    import { addAlert } from '../../../stores/alert';
    import { copy } from '../../../util/tauri';
    import Icon from '../../icon.svelte';
    import Tabs from '../../tabs.svelte';

    let activeTabValue = $state<number>(0);
    let text = $state<string>('');
    let outputFormat = $state<'dataURI' | 'plain'>('dataURI');
    let output = $state<string>();
    let base64 = $state<string>('');
    let textOutput = $state<string>();

    function textToBase64() {
        if (!text || !outputFormat) {
            addAlert({
                type: 'error',
                message: 'No text given or no output format specified.',
                dismissible: true,
                timeout: 7500,
            });
            return;
        }

        const converted = window.btoa(text);
        const prefix = 'data:text/plain;base64,';
        output =
            outputFormat == 'dataURI' ? `${prefix}${converted}` : converted;
    }

    function base64ToText() {
        if (!base64) {
            addAlert({
                type: 'error',
                message: 'No base64 text given.',
                dismissible: true,
                timeout: 7500,
            });
            return;
        }

        textOutput = window.atob(base64.replace(/^data:[^;]+;base64,/, ''));
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
                            <div class="flex-1">
                                <textarea
                                    class="input"
                                    rows="5"
                                    bind:value={text}
                                ></textarea>
                            </div>
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
                        <button class="main-btn" onclick={textToBase64}>
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
                                    <textarea readonly class="input" rows="5"
                                        >{output}</textarea
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
                                    bind:value={base64}
                                ></textarea>
                            </div>
                        </div>
                    </div>

                    <div class="mt-4 flex space-x-2 items-center">
                        <button class="main-btn" onclick={base64ToText}>
                            Convert
                        </button>
                    </div>

                    {#if textOutput}
                        <div
                            in:fade
                            out:fade
                            class="mt-2 flex space-x-2 items-center max-w-2xl"
                        >
                            <p class="text-6xl">=</p>
                            <div class="flex-1 flex flex-col">
                                <div class="flex items-center space-x-2">
                                    <textarea readonly class="input" rows="5"
                                        >{textOutput}</textarea
                                    >
                                    <button
                                        class="cursor-pointer"
                                        onclick={() =>
                                            copy(textOutput ?? '', true)}
                                    >
                                        <Icon name="copy" class="text-2xl" />
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
