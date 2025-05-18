import { writable } from 'svelte/store';

export type P2PFileStatus =
    | 'waitingForAcceptOrDecline'
    | 'sendingData'
    | 'declined'
    | 'finished'
    | 'error'
    | 'closed';

export interface P2PFileSend {
    id: string;
    peer: string;
    ws: WebSocket;
    status: P2PFileStatus;
    file: File;
    transferred: number;
}

export interface P2PFileReceive {
    id: string;
    peer: string;
    status: P2PFileStatus;
    size: number;
    name: string;
    transferred: number;
    savePath?: string;
}

export interface P2PText {
    id: string;
    peer: string;
    text: string;
}

export const p2pFilesSending = writable<P2PFileSend[]>([]);
export const p2pFilesReceiving = writable<P2PFileReceive[]>([]);
export const p2pTextSent = writable<P2PText[]>([]);
export const p2pTextReceived = writable<P2PText[]>([]);
