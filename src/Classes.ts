export enum LoadState {
    NotLoading,
    Loading,
    Done
}

export class MessagePayload {
    content: string = "";
}

export class HandshakePayload {
    status: string = "";
    sender: string = "";
    id: string = "";
}

export class ConnectedPayload {
    name: string = "";
    id: string = "";

    public constructor(name: string, id: string) {
        this.name = name;
        this.id = id;
    }
}

export class Message {
    id: number = 0;
    sender: string = "";
    dst: string = "";
    content: string = "";
}
