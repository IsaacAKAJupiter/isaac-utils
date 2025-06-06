import { get } from 'svelte/store';
import { v4 } from 'uuid';
import {
    p2pFilesReceiving,
    p2pFilesSending,
    p2pTextReceived,
    p2pTextSent,
    type P2PFileSend,
    type P2PFileStatus,
} from '../stores/p2p';
import { sendNotification } from './notification';

const WS_PORT = 15446;

export function handleP2PReceiveMessage(payload: { event: string; data: any }) {
    switch (payload.event) {
        case 'ask_file':
            const peerAsk = payload.data.peer.split(':')[0];
            sendNotification({
                title: 'New P2P File Request',
                body: `New file from ${peerAsk}: ${payload.data.file_name}.`,
            });
            p2pFilesReceiving.update((v) => [
                ...v,
                {
                    id: payload.data.id,
                    peer: peerAsk,
                    status: 'waitingForAcceptOrDecline',
                    name: payload.data.file_name,
                    size: payload.data.file_size,
                    transferred: 0,
                },
            ]);
            break;
        case 'file_data':
            p2pFilesReceiving.update((v) =>
                v.map((f) => {
                    if (f.id !== payload.data.id) return f;

                    const transferred = payload.data.total_processed;
                    const isDone = transferred >= f.size;
                    return {
                        ...f,
                        transferred,
                        status: isDone ? 'finished' : 'sendingData',
                    };
                })
            );
            break;
        case 'text_received':
            const peerText = payload.data.peer.split(':')[0];
            sendNotification({
                title: 'New P2P Text',
                body: `New text from ${peerText}.`,
            });
            p2pTextReceived.update((v) => [
                ...v,
                {
                    id: v4(),
                    peer: peerText,
                    text: payload.data.text,
                },
            ]);
            break;
        default:
            alert(
                `Invalid payload from backend WS: ${JSON.stringify(payload)}`
            );
    }
}

export async function sendText(ip: string, text: string) {
    return new Promise<{ success: true } | { success: false; error: any }>(
        (resolve) => {
            const ws = new WebSocket(`ws://${ip}:${WS_PORT}`);

            const timeout = setTimeout(() => {
                ws.close();
            }, 1500);

            ws.addEventListener('open', () => {
                clearTimeout(timeout);
                ws.send('text');
                ws.send(text);
                ws.close();
                p2pTextSent.update((v) => [
                    ...v,
                    {
                        id: v4(),
                        peer: ip,
                        text,
                    },
                ]);
                resolve({ success: true });
            });
            ws.addEventListener('error', (e) =>
                resolve({ success: false, error: e })
            );
        }
    );
}

export function sendFile(ip: string, file: File) {
    const ws = new WebSocket(`ws://${ip}:${WS_PORT}`);
    const id = v4();

    const p2pFile: P2PFileSend = {
        id,
        peer: ip,
        ws,
        status: 'waitingForAcceptOrDecline',
        file,
        transferred: 0,
    };

    const timeout = setTimeout(() => {
        updateP2PFileSendStatus(id, 'error');
        ws.close();
    }, 1500);

    ws.addEventListener('open', (e) => _onOpen(p2pFile, timeout, e));
    ws.addEventListener('message', (e) => _onMessage(p2pFile, e));
    ws.addEventListener('close', (e) => _onClose(p2pFile, e));
    ws.addEventListener('error', (e) => _onError(p2pFile, e));

    p2pFilesSending.update((v) => [...v, p2pFile]);
}

function startPing(ws: WebSocket) {
    if (ws.readyState != WebSocket.OPEN) return;

    ws.send('ping');
    setTimeout(() => startPing(ws), 1000);
}

function updateP2PFileSendStatus(id: string, status: P2PFileStatus) {
    p2pFilesSending.update((v) =>
        v.map((f) => {
            if (f.id !== id) return f;

            return {
                ...f,
                status,
            };
        })
    );
}

function updateP2PFileSendTransferred(id: string, transferred: number) {
    p2pFilesSending.update((v) =>
        v.map((f) => {
            if (f.id !== id) return f;

            return {
                ...f,
                transferred:
                    transferred > f.file.size ? f.file.size : transferred,
            };
        })
    );
}

function _onOpen(p2pFile: P2PFileSend, timeout: number, _event: Event) {
    clearTimeout(timeout);
    startPing(p2pFile.ws);
    p2pFile.ws.send('file');
    p2pFile.ws.send(
        `${p2pFile.id}<|>${p2pFile.file.name}<|>${p2pFile.file.size}`
    );
}

function _onMessage(p2pFile: P2PFileSend, event: MessageEvent) {
    if (
        event.data === 'tick' ||
        event.data === 'ping' ||
        event.data === 'pong'
    ) {
        return;
    }

    const currentFile = get(p2pFilesSending).find((f) => f.id == p2pFile.id);
    if (!currentFile) return;

    if (currentFile.status == 'waitingForAcceptOrDecline') {
        switch (event.data) {
            case '0':
                updateP2PFileSendStatus(currentFile.id, 'declined');
                currentFile.ws.close();
                break;
            case '1':
                _sendFileStart(currentFile);
                break;
            default:
                break;
        }
        return;
    }

    if (currentFile.status == 'sendingData') {
        if (
            typeof event.data === 'string' &&
            event.data.startsWith('__processed')
        ) {
            const [_, fileID, _dataLen, fileProcessed, _successfulWrite] =
                event.data.split('<|>');
            const processed = +fileProcessed;
            updateP2PFileSendTransferred(fileID, processed);

            if (processed >= currentFile.file.size) {
                updateP2PFileSendStatus(fileID, 'finished');
                p2pFile.ws.close();
            }
            return;
        }

        return;
    }
}

function _onClose(p2pFile: P2PFileSend, _event: Event) {
    const currentFile = get(p2pFilesSending).find((f) => f.id == p2pFile.id);
    if (['error', 'declined', 'finished'].includes(currentFile?.status ?? '')) {
        return;
    }

    updateP2PFileSendStatus(p2pFile.id, 'closed');
}

function _onError(p2pFile: P2PFileSend, _event: any) {
    updateP2PFileSendStatus(p2pFile.id, 'error');
    p2pFile.ws.close();
}

function _sendFileStart(p2pFile: P2PFileSend) {
    updateP2PFileSendStatus(p2pFile.id, 'sendingData');

    const stream = p2pFile.file.stream();
    stream
        .pipeTo(
            new WritableStream({
                write: (chunk) => {
                    p2pFile.ws.send(chunk);
                },
                abort: (err) => {
                    _onError(p2pFile, err);
                },
            })
        )
        .catch((err) => _onError(p2pFile, err))
        .then(() => {});
}
