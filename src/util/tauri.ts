import { invoke } from '@tauri-apps/api/core';
import { check } from '@tauri-apps/plugin-updater';
import { ask, message } from '@tauri-apps/plugin-dialog';
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

export async function checkForAppUpdates(onUserClick: boolean = false) {
    // If failed to check for updates.
    const update = await check();
    if (update === null) {
        await message('Failed to check for updates.\nPlease try again later.', {
            title: 'Error',
            kind: 'error',
            okLabel: 'OK',
        });

        return;
    }

    // If no update.
    if (!update?.available && onUserClick) {
        if (onUserClick) {
            await message('You are on the latest version.', {
                title: 'No Update Available',
                kind: 'info',
                okLabel: 'OK',
            });
        }

        return;
    }

    // Handle update available.
    const yes = await ask(
        `Update to ${update.version} is available!\n\nRelease notes: ${update.body}`,
        {
            title: 'Update Available',
            kind: 'info',
            okLabel: 'Update',
            cancelLabel: 'Cancel',
        }
    );
    if (yes) {
        await update.downloadAndInstall();
        await invoke('graceful_restart');
    }
}
