<script lang="ts">
    import { onMount, tick } from 'svelte';
    import { fade } from 'svelte/transition';
    import {
        getAvailableTimeZones,
        getFormattedDate,
        getUserTimeZone,
        nowDateInputValue,
        nowDateTimeInputValue,
        parseDateTimeIntoTimestamp,
        type DateFormat,
    } from '../../../util/date';
    import config from '../../../stores/config';
    import { addAlert } from '../../../stores/alert';
    import { defaultConfig } from '../../../util/config';
    import { copy } from '../../../util/tauri';
    import Icon from '../../icon.svelte';
    import { DateTime } from 'luxon';

    let timeZones: string[];

    // Timestamp.
    let timeStamp: number;
    let timeStampConverted:
        | {
              formatted: string;
              fetchedIn: DateFormat;
          }
        | undefined;

    // Date time.
    let dateTime: any;
    let timeZone: string;
    let dateTimeConverted: { s: number; ms: number } | undefined;

    // Last 24h.
    let last24hDate: string;
    let last24hTime: string;
    let last24hTz: string;
    let last24hDisplayType: 'seconds' | 'milliseconds' = 'milliseconds';
    let last24hConverted: { start: number; end: number } | undefined;

    async function convertTimestamp(n: number) {
        // Reset.
        timeStampConverted = undefined;
        await tick();

        // Set to new formatted.
        timeStampConverted = getFormattedDate(n, $config || defaultConfig());
    }

    async function convertDate(dateTime: string, timeZone: string) {
        // Reset.
        dateTimeConverted = undefined;
        await tick();

        // If invalid time zone.
        if (!timeZones.includes(timeZone)) {
            addAlert({ message: 'Invalid time zone.', type: 'error' });
            return;
        }

        // Parse.
        const parsedMS = parseDateTimeIntoTimestamp(dateTime, timeZone);
        const parsedS = Math.floor(parsedMS);
        dateTimeConverted = { s: parsedS, ms: parsedMS };
    }

    async function convertLast24Hour() {
        // Reset.
        last24hConverted = undefined;
        await tick();

        // If invalid time zone.
        if (!timeZones.includes(last24hTz)) {
            addAlert({ message: 'Invalid time zone.', type: 'error' });
            return;
        }

        // Get the start and end.
        const end = DateTime.fromISO(
            `${last24hDate}T${last24hTime}:00.000`
        ).setZone(last24hTz.includes('UTC') ? 'utc' : last24hTz, {
            keepLocalTime: true,
        });
        const start = end.minus({ days: 1 });

        // Set to either ms or s.
        const method =
            last24hDisplayType === 'seconds' ? 'toUnixInteger' : 'toMillis';
        last24hConverted = { start: start[method](), end: end[method]() };
    }

    onMount(() => {
        // Get list of time zones and current time zone.
        timeZones = getAvailableTimeZones().concat(['UTC / GMT']);
        timeZone = $config?.unix?.timeZone ?? getUserTimeZone();
        last24hTz = $config?.unix?.timeZone ?? getUserTimeZone();
        timeStamp = Date.now();
        dateTime = nowDateTimeInputValue();
        last24hDate = nowDateInputValue();
        last24hTime = '22:59';
    });
</script>

<div class="p-4 bg-accent">
    <!-- Timestamp -> Human Readable -->
    <div class="mb-8">
        <p class="text-lg">Convert timestamp to human readable.</p>
        <p class="text-xs mb-2">
            Supports timestamps in seconds, milliseconds, microseconds, and
            nanoseconds.
        </p>
        <div class="flex space-x-2 items-center">
            <div>
                <input
                    class="input number-input"
                    type="number"
                    bind:value={timeStamp}
                />
            </div>
            <button
                class="main-btn"
                on:click={() => convertTimestamp(timeStamp)}
            >
                Convert
            </button>
        </div>
        {#if timeStampConverted}
            <div in:fade out:fade class="mt-2 flex space-x-2 items-center">
                <p class="text-6xl">=</p>
                <div class="flex flex-col">
                    <p class="w-fit">{timeStampConverted.formatted}</p>
                    <p class="text-xs">
                        Converted using
                        <i>{timeStampConverted.fetchedIn}</i>.
                    </p>
                </div>
            </div>
        {/if}
    </div>

    <!-- Datetime -> Timestamp -->
    <div class="mb-8">
        <p class="text-lg mb-1">Convert datetime to timestamp.</p>
        <div class="flex space-x-2 items-center">
            <div>
                <input
                    class="input date"
                    type="datetime-local"
                    bind:value={dateTime}
                />
            </div>
            <div>
                <input
                    class="input"
                    type="text"
                    list="timeZones"
                    bind:value={timeZone}
                />
                {#if timeZones}
                    <datalist id="timeZones">
                        {#each timeZones as t}
                            <option value={t}>{t}</option>
                        {/each}
                    </datalist>
                {/if}
            </div>
            <button
                class="main-btn"
                on:click={() => convertDate(dateTime, timeZone)}
            >
                Convert
            </button>
        </div>
        {#if dateTimeConverted}
            <div in:fade out:fade class="mt-2 flex space-x-2 items-center">
                <p class="text-6xl">=</p>
                <div class="flex flex-col">
                    <div class="flex items-center space-x-2">
                        <p>In Seconds: {dateTimeConverted.s}</p>
                        <button
                            class="cursor-pointer"
                            on:click={() =>
                                copy(dateTimeConverted?.s ?? 0, true)}
                        >
                            <Icon name="copy"></Icon>
                        </button>
                    </div>
                    <div class="flex items-center space-x-2">
                        <p>In Milliseconds: {dateTimeConverted.ms}</p>
                        <button
                            class="cursor-pointer"
                            on:click={() =>
                                copy(dateTimeConverted?.ms ?? 0, true)}
                        >
                            <Icon name="copy"></Icon>
                        </button>
                    </div>
                </div>
            </div>
        {/if}
    </div>

    <!-- Timestamp last 24h. -->
    <div class="mb-8">
        <p class="text-lg">Timestamps for last 24 hours.</p>
        <p class="text-xs mb-2">
            e.g: Jan 5 / 10:59pm will get timestamps for Jan 4 10:59pm -> Jan 5
            10:59pm
        </p>
        <div class="flex space-x-2 items-end">
            <div class="grid grid-cols-2 gap-2">
                <div>
                    <input
                        class="input date"
                        type="date"
                        bind:value={last24hDate}
                    />
                </div>
                <div>
                    <input
                        class="input date"
                        type="time"
                        bind:value={last24hTime}
                    />
                </div>
                <div>
                    <input
                        class="input"
                        type="text"
                        list="timeZones"
                        bind:value={last24hTz}
                    />
                    {#if timeZones}
                        <datalist id="timeZones">
                            {#each timeZones as t}
                                <option value={t}>{t}</option>
                            {/each}
                        </datalist>
                    {/if}
                </div>
                <div>
                    <select class="input" bind:value={last24hDisplayType}>
                        <option value="seconds">Seconds</option>
                        <option value="milliseconds">Milliseconds</option>
                    </select>
                </div>
            </div>
            <button class="main-btn" on:click={convertLast24Hour}>
                Convert
            </button>
        </div>
        {#if last24hConverted}
            <div in:fade out:fade class="mt-2 flex space-x-2 items-center">
                <p class="text-6xl">=</p>
                <div class="flex flex-col">
                    <div class="flex items-center space-x-2">
                        <p>Start: {last24hConverted.start}</p>
                        <button
                            class="cursor-pointer"
                            on:click={() =>
                                copy(last24hConverted?.start ?? 0, true)}
                        >
                            <Icon name="copy"></Icon>
                        </button>
                    </div>
                    <div class="flex items-center space-x-2">
                        <p>End: {last24hConverted.end}</p>
                        <button
                            class="cursor-pointer"
                            on:click={() =>
                                copy(last24hConverted?.end ?? 0, true)}
                        >
                            <Icon name="copy"></Icon>
                        </button>
                    </div>
                </div>
            </div>
        {/if}
    </div>
</div>
