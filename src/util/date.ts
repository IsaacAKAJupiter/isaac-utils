import { DateTime } from 'luxon';
import type { Config } from './config';

export type DateFormat =
    | 'seconds'
    | 'milliseconds'
    | 'microseconds'
    | 'nanoseconds';

export function getUserTimeZone() {
    return Intl.DateTimeFormat().resolvedOptions().timeZone;
}

export function getAvailableTimeZones() {
    return Intl.supportedValuesOf('timeZone');
}

export function nowDateTimeInputValue() {
    return DateTime.now().set({ second: 0, millisecond: 0 }).toISO({
        includeOffset: false,
        suppressMilliseconds: true,
        suppressSeconds: true,
    });
}

export function nowDateInputValue() {
    return DateTime.now().toISODate();
}

export function parseDateTimeIntoTimestamp(
    dateTime: string,
    timeZone: string
): number {
    return DateTime.fromISO(dateTime)
        .setZone(timeZone.includes('UTC') ? 'utc' : timeZone, {
            keepLocalTime: true,
        })
        .toMillis();
}

export function getFormattedDate(n: number, config: Config) {
    // Fetch the date.
    const fetchFormat = config.unix?.fetchFormat || 'auto';
    const timeZone = config.unix?.timeZone || getUserTimeZone();
    const { date, fetchedIn } = getDateFromFetchFormat(n, fetchFormat);

    // Format/return date.
    const formatted = date
        .setZone(timeZone)
        .toLocaleString(DateTime.DATETIME_FULL_WITH_SECONDS, {
            locale: config.locale,
        });
    return { formatted, fetchedIn };
}

function getDateFromFetchFormat(
    n: number,
    fetchFormat: DateFormat | 'auto'
): { date: DateTime; fetchedIn: DateFormat } {
    switch (fetchFormat) {
        case 'auto':
            // Replace the decimal and numbers after it to get proper length.
            const stringLen = `${n}`.replace(/\.[0-9]*/g, '').length;

            if (stringLen >= 12 && stringLen < 15) {
                return getDateInFormat(n, 'milliseconds');
            } else if (stringLen >= 15 && stringLen < 17) {
                return getDateInFormat(n, 'microseconds');
            } else if (stringLen >= 17) {
                return getDateInFormat(n, 'nanoseconds');
            } else {
                return getDateInFormat(n, 'seconds');
            }
        case 'seconds':
            return getDateInFormat(n, 'seconds');
        case 'milliseconds':
            return getDateInFormat(n, 'milliseconds');
        case 'microseconds':
            return getDateInFormat(n, 'microseconds');
        case 'nanoseconds':
            return getDateInFormat(n, 'nanoseconds');
        default:
            return getDateInFormat(n, 'seconds');
    }
}

function getDateInFormat(
    n: number,
    format: 'seconds' | 'milliseconds' | 'microseconds' | 'nanoseconds'
) {
    const correctedNum =
        format === 'seconds'
            ? n * 1000
            : format === 'milliseconds'
            ? n
            : format === 'microseconds'
            ? n / 1000
            : n / 1000000;
    return { date: DateTime.fromMillis(correctedNum), fetchedIn: format };
}
