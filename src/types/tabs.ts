import type { ComponentType } from 'svelte';

export interface Tab {
    label: string;
    value: number;
    component: ComponentType;
}
