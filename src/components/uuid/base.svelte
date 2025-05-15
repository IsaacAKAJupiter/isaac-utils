<script lang="ts">
    import Tabs from '../tabs.svelte';
    import Convert from './tabs/convert.svelte';
    import Settings from './tabs/settings.svelte';
</script>

<main>
    <p class="mt-8 mb-2 text-center text-2xl">UUID Utilities</p>

    <div class="px-4">
        <Tabs
            items={[
                {
                    label: 'Convert',
                    value: 1,
                    component: Convert,
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
