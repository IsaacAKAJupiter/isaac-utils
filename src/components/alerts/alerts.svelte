<script lang="ts">
    import { alerts, dismissAlert } from '../../stores/alert';
    import Alert from './alert.svelte';
</script>

{#if $alerts}
    <section
        class="fixed top-0 left-0 right-0 w-full flex flex-col mt-4 justify-center z-50"
    >
        {#each $alerts as alert (alert.id)}
            <Alert
                type={alert.type}
                dismissible={alert.dismissible}
                on:dismiss={() => dismissAlert(alert.id)}
            >
                {alert.message}
            </Alert>
        {/each}
    </section>
{/if}
