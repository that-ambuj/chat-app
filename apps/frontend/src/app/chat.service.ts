import { Injectable } from '@angular/core';
import { WebsocketService } from './websocket.service';
import { Message } from '@chat-app/types';
import { Subject, first } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class ChatService {
  public messages: Subject<Message>;
  public userId!: number;

  constructor(private wsService: WebsocketService) {
    this.messages = this.wsService.connect();

    this.messages.pipe(first()).subscribe((msg) => {
      this.userId = msg.from;
    });
  }

  public sendMessage(message: string): void {
    this.messages.next({ message, from: this.userId });
  }
}
