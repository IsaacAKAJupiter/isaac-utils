import { invoke } from '@tauri-apps/api/core';
import { addAlert } from '../stores/alert';

export async function copy(value: string | number, alert: boolean) {
    const success = await invoke<boolean>('c_copy', { value: `${value}` });

    // Alert if needed.
    if (alert) {
        addAlert({
            type: success ? 'success' : 'error',
            message: success
                ? 'Copied to clipboard!'
                : 'Failed copying to clipboard.',
            dismissible: true,
            timeout: 5000,
        });
    }

    return success;
}
