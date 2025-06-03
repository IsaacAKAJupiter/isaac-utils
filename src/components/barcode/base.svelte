<script lang="ts">
    import Tabs from '../tabs.svelte';
    import Generate from './tabs/generate.svelte';
    import Settings from './tabs/settings.svelte';
</script>

<main>
    <p class="mt-8 mb-2 text-center text-2xl">Barcode</p>

    <div class="px-4">
        <Tabs
            items={[
                {
                    label: 'Generate',
                    value: 1,
                    component: Generate,
                },
                {
                    label: 'Settings',
                    value: 2,
                    component: Settings,
                },
            ]}
        />
    </div>
</main>
