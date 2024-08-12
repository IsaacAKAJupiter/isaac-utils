import { writable } from 'svelte/store';

export type AlertType = 'success' | 'error' | 'info';
export interface Alert {
    id: string;
    message: string;
    type: AlertType;
    dismissible: boolean;
    timeout?: number;
}

export const alerts = writable<Alert[]>([]);

export const addAlert = (
    alert: Omit<Partial<Alert>, 'message' | 'id'> & { message: string }
) => {
    // Unique ID for the alert.
    const id = `${Date.now()}${Math.floor(Math.random() * 10000)}`;

    // Setup some sensible defaults for an alert.
    const defaults = {
        id,
        type: 'info' as AlertType,
        dismissible: true,
        timeout: 5000,
    };

    // Push the toast to the top of the list of alerts.
    alerts.update((all) => [{ ...defaults, ...alert }, ...all]);

    // If toast is dismissible, dismiss it after "timeout" amount of time.
    if (alert.timeout) setTimeout(() => dismissAlert(id), alert.timeout);
};

export const dismissAlert = (id: string) => {
    alerts.update((all) => all.filter((t) => t.id !== id));
};
