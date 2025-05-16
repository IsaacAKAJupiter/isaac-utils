<script lang="ts">
    let {
        id,
        name,
        value,
        group = $bindable(),
        label,
    }: {
        id?: string;
        name: string;
        value: any;
        group: any;
        label: string;
    } = $props();
</script>

<div class="inline-flex items-center">
    <div class="">
        <label class="relative flex items-center cursor-pointer">
            <input
                type="radio"
                class="peer h-5 w-5 cursor-pointer appearance-none rounded-full border border-slate-300 checked:border-primary transition-all"
                {id}
                {name}
                {value}
                bind:group
            />
            <span
                class="absolute bg-primary w-3 h-3 rounded-full opacity-0 peer-checked:opacity-100 transition-opacity duration-200 top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
            >
            </span>
        </label>
    </div>
    <label class="ml-2 cursor-pointer select-none" for={id}>{label}</label>
</div>
