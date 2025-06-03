<script lang="ts">
    import BwipJs from '@bwip-js/browser';
    import { invoke } from '@tauri-apps/api/core';
    import { open, save } from '@tauri-apps/plugin-dialog';
    import { addAlert } from '../../../stores/alert';

    const BARCODE_OPTIONS = [
        'auspost',
        'azteccode',
        'azteccodecompact',
        'aztecrune',
        'bc412',
        'channelcode',
        'codablockf',
        'code11',
        'code128',
        'code16k',
        'code2of5',
        'code32',
        'code39',
        'code39ext',
        'code49',
        'code93',
        'code93ext',
        'codeone',
        'coop2of5',
        'daft',
        'databarexpanded',
        'databarexpandedcomposite',
        'databarexpandedstacked',
        'databarexpandedstackedcomposite',
        'databarlimited',
        'databarlimitedcomposite',
        'databaromni',
        'databaromnicomposite',
        'databarstacked',
        'databarstackedcomposite',
        'databarstackedomni',
        'databarstackedomnicomposite',
        'databartruncated',
        'databartruncatedcomposite',
        'datalogic2of5',
        'datamatrix',
        'datamatrixrectangular',
        'datamatrixrectangularextension',
        'dotcode',
        'ean13',
        'ean13composite',
        'ean14',
        'ean2',
        'ean5',
        'ean8',
        'ean8composite',
        'flattermarken',
        'gs1_128',
        'gs1_128composite',
        'gs1_cc',
        'gs1datamatrix',
        'gs1datamatrixrectangular',
        'gs1dldatamatrix',
        'gs1dlqrcode',
        'gs1dotcode',
        'gs1northamericancoupon',
        'gs1qrcode',
        'hanxin',
        'hibcazteccode',
        'hibccodablockf',
        'hibccode128',
        'hibccode39',
        'hibcdatamatrix',
        'hibcdatamatrixrectangular',
        'hibcmicropdf417',
        'hibcpdf417',
        'hibcqrcode',
        'iata2of5',
        'identcode',
        'industrial2of5',
        'interleaved2of5',
        'isbn',
        'ismn',
        'issn',
        'itf14',
        'jabcode',
        'japanpost',
        'kix',
        'leitcode',
        'mailmark',
        'mands',
        'matrix2of5',
        'maxicode',
        'micropdf417',
        'microqrcode',
        'msi',
        'onecode',
        'pdf417',
        'pdf417compact',
        'pharmacode',
        'pharmacode2',
        'planet',
        'plessey',
        'posicode',
        'postnet',
        'pzn',
        'qrcode',
        'rationalizedCodabar',
        'raw',
        'rectangularmicroqrcode',
        'royalmail',
        'sscc18',
        'swissqrcode',
        'symbol',
        'telepen',
        'telepennumeric',
        'ultracode',
        'upca',
        'upcacomposite',
        'upce',
        'upcecomposite',
    ] as const;

    let hasGenerated = $state<boolean>(false);
    let type = $state<string>('qrcode');
    let text = $state<string>('');
    let errorCorrection = $state<'L' | 'M' | 'Q' | 'H'>('M');
    let includeText = $state<boolean>(false);
    let barcodePerLine = $state<boolean>(false);
    let canvas = $state<HTMLCanvasElement>();
    let hiddenCanvas = $state<HTMLCanvasElement>();

    async function generate() {
        if (!canvas) return;

        if (!text) {
            addAlert({
                type: 'error',
                message: 'No text given to generate.',
                timeout: 5000,
                dismissible: true,
            });
            return;
        }

        if (!barcodePerLine) {
            const success = _generate(type, text);
            if (success) hasGenerated = true;
            return;
        }

        const dir = await open({ directory: true });
        if (!dir) return;

        const lines = text.split(/\n/g);

        const errors: string[] = [];
        const successes: string[] = [];
        for (const line of lines) {
            if (!line.trim()) continue;

            if (!_generate(type, line, hiddenCanvas)) continue;

            try {
                const blob = await new Promise<Blob | null>((res) =>
                    hiddenCanvas!.toBlob((blob) => {
                        res(blob);
                    }, 'image/png')
                );
                if (!blob) {
                    errors.push(line);
                    continue;
                }

                const data = await blob.arrayBuffer();
                const success = await invoke<boolean>('c_save_file', {
                    path: dir,
                    name: `${line.trim()}.png`,
                    data,
                });
                if (!success) {
                    errors.push(line);
                    continue;
                }

                successes.push(line);
            } catch (e) {
                errors.push(line);
            }
        }

        addAlert({
            type: 'info',
            message: `Successfully saved: ${successes.length} files. Failed saving: ${errors.length} files.`,
            dismissible: true,
            timeout: 10000,
        });
    }

    async function download() {
        if (!canvas || !hasGenerated) return;

        const path = await save({
            filters: [{ extensions: ['png'], name: 'png' }],
        });
        if (!path) return;

        try {
            const blob = await new Promise<Blob | null>((res) =>
                canvas!.toBlob((blob) => {
                    res(blob);
                }, 'image/png')
            );
            if (!blob) {
                addAlert({
                    type: 'error',
                    message: `Failed saving file.`,
                    dismissible: true,
                    timeout: 5000,
                });
                return;
            }

            const data = await blob.arrayBuffer();
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
        } catch (e) {
            addAlert({
                type: 'error',
                message: `Failed saving with error: ${e}.`,
                dismissible: true,
                timeout: 10000,
            });
        }
    }

    function _generate(
        type: string,
        text: string,
        givenCanvas?: HTMLCanvasElement
    ) {
        if (!givenCanvas && !canvas) return false;

        try {
            BwipJs.toCanvas((givenCanvas ?? canvas)!, {
                bcid: type,
                text,
                paddingwidth: 10,
                paddingheight: 10,
                backgroundcolor: '#FFF',
                includetext: includeText,
                eclevel: type === 'qrcode' ? errorCorrection : undefined,
            } as any);
            return true;
        } catch (e: any) {
            addAlert({
                type: 'error',
                message: `Error generating: ${e.message}`,
                timeout: 10000,
                dismissible: true,
            });
            return false;
        }
    }
</script>

<div class="p-4 bg-accent">
    <div>
        <div>
            <div class="mb-2 flex space-x-2 items-center">
                <p class="w-32">Version:</p>
                <div>
                    <select class="input" bind:value={type}>
                        {#each BARCODE_OPTIONS as key}
                            <option value={key}>{key}</option>
                        {/each}
                    </select>
                </div>
            </div>

            <div class="mb-2 flex space-x-2">
                <p class="w-32">Text:</p>
                <div class="flex-1">
                    <textarea class="input" rows="5" bind:value={text}
                    ></textarea>
                </div>
            </div>

            {#if type === 'qrcode'}
                <div class="mb-2 flex space-x-2 items-center">
                    <p class="w-32">Error Correction:</p>
                    <div>
                        <select class="input" bind:value={errorCorrection}>
                            <option value="L">L (Low): 7%</option>
                            <option value="M">M (Medium): 15%</option>
                            <option value="Q">Q (Quartile): 25%</option>
                            <option value="H">H (High): 30%</option>
                        </select>
                    </div>
                </div>
            {/if}

            <div class="mb-4">
                <label class="flex space-x-2 items-center">
                    <input
                        type="checkbox"
                        class="checkbox"
                        bind:checked={includeText}
                    />
                    <span>Include Text</span>
                </label>
            </div>

            <div class="mb-4">
                <label class="flex space-x-2 items-center">
                    <input
                        type="checkbox"
                        class="checkbox"
                        bind:checked={barcodePerLine}
                    />
                    <span>Generate One Barcode Per Line</span>
                </label>
            </div>
        </div>
        <div class="mt-4 flex space-x-2 items-center">
            <button class="main-btn" onclick={generate}>Generate</button>
        </div>

        <div class="mt-8 max-w-[80vw] overflow-auto">
            <canvas width="0" height="0" bind:this={canvas}></canvas>
        </div>

        {#if hasGenerated}
            <div class="mt-4 flex space-x-2 items-center">
                <button class="main-btn" onclick={download}>Download</button>
            </div>
        {/if}

        <canvas width="0" height="0" class="hidden" bind:this={hiddenCanvas}
        ></canvas>
    </div>
</div>
