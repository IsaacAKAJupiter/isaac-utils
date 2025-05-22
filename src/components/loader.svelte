<script lang="ts">
    const {
        classes,
        circleClasses,
        style,
    }: { classes?: string; circleClasses?: string; style?: 'circle' | 'dots' } =
        $props();
</script>

{#if (style ?? 'circle') == 'circle'}
    <span
        class="circle w-12 h-12 border-[5px] border-white border-b-[var(--primary-colour)] rounded-[50%] inline-block box-border {classes}"
    ></span>
{:else if style == 'dots'}
    <div class="dots flex space-x-2 items-center {classes}">
        <span class="bg-white w-[1em] h-[1em] rounded-[50%] {circleClasses}">
        </span>
        <span
            class="bg-white w-[1em] h-[1em] rounded-[50%] ![animation-delay:_0.33s] {circleClasses}"
        >
        </span>
        <span
            class="bg-white w-[1em] h-[1em] rounded-[50%] ![animation-delay:_0.66s] {circleClasses}"
        >
        </span>
    </div>
{/if}

<style>
    span.circle {
        animation: rotation 1s linear infinite;
    }

    @keyframes rotation {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    .dots > span {
        animation: dots 0.5s infinite alternate both ease-in-out;
    }

    @keyframes dots {
        0% {
            scale: 0;
        }

        100% {
            scale: 1;
        }
    }
</style>
