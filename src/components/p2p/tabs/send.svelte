<script lang="ts">
    // TODO: Now that ports can be checked, we should work on the impl for the actual listener/server stuff (probably toggle enable in that tab).

    // Results is an array of arrays (ip, success). Filter if success is true and map to IP.
    // const results =
    //     (
    //         await invoke<{ results: [string, boolean][] | null }>(
    //             'c_check_ports'
    //         )
    //     ).results ?? [];
    // console.log(results.filter((r) => r[1]).map((r) => r[0]));
</script>

<div class="p-4 bg-accent">
    <div></div>
</div>
