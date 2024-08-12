<script lang="ts">
    import { slide } from 'svelte/transition';
    import type { AlertType } from '../../stores/alert';
    import Icon from '../icon.svelte';
    import { createEventDispatcher } from 'svelte';

    export let type: AlertType;
    export let dismissible: boolean;

    let typeClasses: { [key in AlertType]: string } = {
        info: 'bg-blue-400 text-white',
        success: 'bg-green-600 text-white',
        error: 'bg-red-600 text-white',
    };

    const dispath = createEventDispatcher();
</script>

<article
    transition:slide
    role="alert"
    class="w-80 py-2 px-4 rounded flex space-x-4 items-center mx-auto mb-2 {typeClasses[
        type
    ]}"
>
    <!-- Text. -->
    <div class="flex-1">
        <slot />
    </div>

    {#if dismissible}
        <button
            class="bg-transparent border-none p-0 m-0 text-base"
            on:click={() => dispath('dismiss')}
        >
            <Icon name="x"></Icon>
        </button>
    {/if}
</article>
