export enum LoadState {
    NotLoading,
    Loading,
    Done
}

export class HandshakePayload {
    status: string;
    sender: string;
    id: string;
}

export class ConnectedPayload {
    name: string;

    public constructor(name: string) {
        this.name = name;
    }
}

export class Message {
    id: number
    sender: string;
    dst: string;
    content: string;
}