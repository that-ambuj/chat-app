import { Component } from '@angular/core';
import { ApiMessage } from '@chat-app/types';

@Component({
  selector: 'chat-app-chat-window',
  templateUrl: './chat-window.component.html',
})
export class ChatWindowComponent {
  user = 2736;

  messages: ApiMessage[] = [
    { message: 'Hello', to: 2736 },
    { message: 'Hi from Angular', to: 4589 },
    { message: 'Hello from React', to: 3013 },
  ];
}
