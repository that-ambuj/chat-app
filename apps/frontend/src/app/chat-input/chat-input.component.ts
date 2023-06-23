import { Component } from '@angular/core';
import { ChatService } from '../chat.service';

@Component({
  selector: 'chat-app-chat-input',
  templateUrl: './chat-input.component.html',
})
export class ChatInputComponent {
  input = '';

  constructor(private chatService: ChatService) {}

  sendMessage() {
    if (this.input) {
      this.chatService.sendMessage(this.input);
      this.input = '';
    }
  }
}
