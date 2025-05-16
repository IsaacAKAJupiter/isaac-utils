<!-- @migration-task Error while migrating Svelte code: $$props is used together with named props in a way that cannot be automatically migrated. -->
<script lang="ts">
    export let name: keyof typeof icons;
    export let width = '1rem';
    export let height = '1rem';
    export let focusable: string | number | null | undefined = undefined;

    let icons = {
        copy: `<path stroke="none" d="M0 0h24v24H0z"/><path d="M7 9.667A2.667 2.667 0 0 1 9.667 7h8.666A2.667 2.667 0 0 1 21 9.667v8.666A2.667 2.667 0 0 1 18.333 21H9.667A2.667 2.667 0 0 1 7 18.333z"/><path d="M4.012 16.737A2.005 2.005 0 0 1 3 15V5c0-1.1.9-2 2-2h10c.75 0 1.158.385 1.5 1"/>`,
        x: `<path stroke="none" d="M0 0h24v24H0z"/><path d="M18 6 6 18M6 6l12 12"/>`,
        '123': `<path stroke="none" d="M0 0h24v24H0z"/><path d="m3 10 2-2v8M9 8h3a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-2a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h3M17 8h2.5A1.5 1.5 0 0 1 21 9.5v1a1.5 1.5 0 0 1-1.5 1.5H18h1.5a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1-1.5 1.5H17"/>`,
        settings: `<path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M10.325 4.317c.426 -1.756 2.924 -1.756 3.35 0a1.724 1.724 0 0 0 2.573 1.066c1.543 -.94 3.31 .826 2.37 2.37a1.724 1.724 0 0 0 1.065 2.572c1.756 .426 1.756 2.924 0 3.35a1.724 1.724 0 0 0 -1.066 2.573c.94 1.543 -.826 3.31 -2.37 2.37a1.724 1.724 0 0 0 -2.572 1.065c-.426 1.756 -2.924 1.756 -3.35 0a1.724 1.724 0 0 0 -2.573 -1.066c-1.543 .94 -3.31 -.826 -2.37 -2.37a1.724 1.724 0 0 0 -1.065 -2.572c-1.756 -.426 -1.756 -2.924 0 -3.35a1.724 1.724 0 0 0 1.066 -2.573c-.94 -1.543 .826 -3.31 2.37 -2.37c1 .608 2.296 .07 2.572 -1.065z" /><path d="M9 12a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" />`,
    } as const;
    let displayIcon = icons[name];
</script>

<svg
    {focusable}
    {width}
    {height}
    class="stroke-current w-[1em] h-[1em] min-w-[1em] max-w-[1em] align-middle overflow-hidden {$$props.class ??
        ''}"
    stroke-linecap="round"
    stroke-linejoin="round"
    stroke-width="2"
    fill="none"
    viewBox="0 0 24 24"
>
    {@html displayIcon}
</svg>
