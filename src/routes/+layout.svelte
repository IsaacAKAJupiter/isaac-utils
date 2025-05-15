<script>
    /** @type {{children?: import('svelte').Snippet}} */
    let { children } = $props();
</script>

<script module>
    import '../app.scss';
</script>

{@render children?.()}
