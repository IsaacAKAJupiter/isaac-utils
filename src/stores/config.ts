import { writable } from 'svelte/store';
import type { Config } from '../util/config';

export const configStore = writable<Config | null>(null);
export const configShortcutLastChange = writable<number | null>(null);
