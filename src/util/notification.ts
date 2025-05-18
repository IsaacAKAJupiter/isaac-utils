import {
    isPermissionGranted,
    requestPermission,
    sendNotification as tauriSendNotification,
    type Options,
} from '@tauri-apps/plugin-notification';

export async function sendNotification(options: Options | string) {
    // Do you have permission to send a notification?
    let permissionGranted = await isPermissionGranted();

    // If not we need to request it
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }

    // Once permission has been granted we can send the notification
    if (permissionGranted) {
        tauriSendNotification(options);
    }
}
