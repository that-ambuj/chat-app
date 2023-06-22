import { Component } from '@angular/core';

@Component({
  selector: 'chat-app-chat-input',
  templateUrl: './chat-input.component.html',
})
export class ChatInputComponent {
  message = '';

  debug() {
    console.log(this.message);
  }
}
