import { Injectable } from '@angular/core';
import { webSocket, WebSocketSubject } from 'rxjs/webSocket';
import { Subject } from 'rxjs';
import { WsMessage } from '@chat-app/types';

const WS_ENDPOINT = 'ws://localhost:8000/ws';

@Injectable({
  providedIn: 'root',
})
export class WebsocketService {
  private wsSubject!: WebSocketSubject<WsMessage>;

  constructor() {}

  public connect(): Subject<WsMessage> {
    if (!this.wsSubject) {
      this.wsSubject = webSocket(WS_ENDPOINT);
      console.log('Connected to Websocket: ', WS_ENDPOINT);
    }

    return this.wsSubject;
  }

  close() {
    this.wsSubject.complete();
  }
}
