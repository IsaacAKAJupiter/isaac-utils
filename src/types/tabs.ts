import type { Component } from 'svelte';

export interface Tab {
    label: string;
    value: number;
    component: Component;
}
