<script lang="ts">
    import { goto } from '$app/navigation';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { onMount } from 'svelte';

    onMount(async () => {
        const label = getCurrentWindow().label;
        switch (label) {
            case 'main':
                goto('/main', { replaceState: true });
                break;
            case 'unix_popup':
                goto('/unix-popup', { replaceState: true });
                break;
            default:
                console.error(`Missing route for window: ${label}.`);
                break;
        }
    });
</script>

<p>routing...</p>

<style>
</style>
