<script lang="ts">
    let { progress, height }: { progress: number; height: string } = $props();
</script>

<div
    class="relative w-full rounded-lg overflow-hidden [box-shadow:_0_0_5px_currentColor]"
    style="height: {height};"
>
    <div
        class="absolute left-0 top-0 bottom-0 bg-current"
        style="right: {100 - progress}%;"
    ></div>
</div>
