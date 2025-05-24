<script lang="ts">
    import Tabs from '../tabs.svelte';
    import Receive from './tabs/receive.svelte';
    import Scan from './tabs/scan.svelte';
    import Send from './tabs/send.svelte';
</script>

<main>
    <p class="mt-8 text-center text-2xl">Peer-to-Peer</p>
    <div class="text-sm mb-2 text-center">
        You can share files and text to other people with the application
        (usually on the local network only).
    </div>

    <div class="px-4">
        <Tabs
            ulClasses="ml-[3.2rem]"
            items={[
                {
                    label: 'Send',
                    value: 1,
                    component: Send,
                },
                {
                    label: 'Receive',
                    value: 2,
                    component: Receive,
                },
                {
                    label: 'Scan',
                    value: 3,
                    component: Scan,
                },
            ]}
        />
    </div>
</main>
