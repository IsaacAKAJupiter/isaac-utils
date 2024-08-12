import { writable } from 'svelte/store';
import type { Config } from '../util/config';

export default writable<Config | null>(null);
