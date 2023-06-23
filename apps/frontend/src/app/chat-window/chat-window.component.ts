import { Component } from '@angular/core';
import { WsMessage } from '@chat-app/types';
import { ChatService } from '../chat.service';

@Component({
  selector: 'chat-app-chat-window',
  templateUrl: './chat-window.component.html',
  providers: [ChatService],
})
export class ChatWindowComponent {
  messages: WsMessage[] = [];
  userId: number;

  constructor(public chatService: ChatService) {
    this.chatService.messages.subscribe((msg) => {
      this.messages.push(msg);
    });
    this.userId = this.chatService.userId;
  }
}
