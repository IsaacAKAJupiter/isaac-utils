import { getUserTimeZone } from './date';
import configStore from '../stores/config';
import {
    BaseDirectory,
    exists,
    mkdir,
    readTextFile,
    writeTextFile,
} from '@tauri-apps/plugin-fs';
import { extendObject } from './object';
import isEqual from 'lodash.isequal';

export interface Config {
    locale: string;
    shortcuts: { unixToReadable: string };
    unix: {
        fetchFormat:
            | 'auto'
            | 'seconds'
            | 'milliseconds'
            | 'microseconds'
            | 'nanoseconds';
        timeZone: string;
    };
}

export function validateAndUpdateConfig(config: Config): Config {
    return extendObject(true, defaultConfig(), config) as Config;
}

export function defaultConfig(): Config {
    return {
        locale: 'en',
        shortcuts: {
            unixToReadable: 'CommandOrControl+Q',
        },
        unix: {
            fetchFormat: 'auto',
            timeZone: getUserTimeZone(),
        },
    };
}

export async function writeConfig(config: { [key: string]: any }) {
    const dirExists = await exists('', {
        baseDir: BaseDirectory.AppData,
    });
    if (!dirExists) {
        await mkdir('', { baseDir: BaseDirectory.AppData });
    }

    await writeTextFile('config.json', JSON.stringify(config), {
        baseDir: BaseDirectory.AppConfig,
    });
}

export async function getConfig() {
    // Check if the config file exists.
    const doesExist = await exists('config.json', {
        baseDir: BaseDirectory.AppConfig,
    });

    // If not, we need to create it with the default settings.
    if (!doesExist) {
        const config = defaultConfig();
        await writeConfig(config);
        return setStoreAndReturnConfig(config);
    }

    // If exists.
    const file = await readTextFile('config.json', {
        baseDir: BaseDirectory.AppConfig,
    });
    const parsed = JSON.parse(file);

    // Validate and update it (and re-save if updated).
    const newConfig = validateAndUpdateConfig(parsed);
    if (!isEqual(parsed, newConfig)) {
        await writeConfig(newConfig);
    }

    return setStoreAndReturnConfig(newConfig);
}

function setStoreAndReturnConfig(config: Config) {
    configStore.set(config);
    return config;
}
