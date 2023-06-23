import { Injectable } from '@angular/core';
import { webSocket, WebSocketSubject } from 'rxjs/webSocket';
import { Subject } from 'rxjs';
import { Message } from '@chat-app/types';
import { WS_ENDPOINT } from './environment';

@Injectable({
  providedIn: 'root',
})
export class WebsocketService {
  private wsSubject!: WebSocketSubject<Message>;

  constructor() {}

  public connect(): Subject<Message> {
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
