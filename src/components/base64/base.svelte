<script lang="ts">
    import Tabs from '../tabs.svelte';
    import File from './tabs/file.svelte';
    import Text from './tabs/text.svelte';
</script>

<main>
    <p class="mt-8 mb-2 text-center text-2xl">Base64</p>

    <div class="px-4">
        <Tabs
            ulClasses="ml-[3.2rem]"
            items={[
                {
                    label: 'Text',
                    value: 1,
                    component: Text,
                },
                {
                    label: 'File',
                    value: 2,
                    component: File,
                },
            ]}
        />
    </div>
</main>
