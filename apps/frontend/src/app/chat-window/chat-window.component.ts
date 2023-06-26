import {
  AfterViewChecked,
  Component,
  OnInit,
  ViewChild,
  ElementRef,
} from '@angular/core';
import { Message } from '@chat-app/types';
import { ChatService } from '../chat.service';

@Component({
  selector: 'chat-app-chat-window',
  templateUrl: './chat-window.component.html',
  providers: [ChatService],
})
export class ChatWindowComponent implements OnInit, AfterViewChecked {
  messages: Message[] = [];
  userId: number;
  @ViewChild('scrollBottom') private scrollBottom!: ElementRef;

  constructor(public chatService: ChatService) {
    this.chatService.messages.subscribe((msg) => {
      this.messages.push(msg);
    });
    this.userId = this.chatService.userId;
  }

  ngOnInit() {
    this.scrollToBottom();
  }

  ngAfterViewChecked() {
    this.scrollToBottom();
  }

  scrollToBottom(): void {
    try {
      this.scrollBottom.nativeElement.scrollTop =
        this.scrollBottom.nativeElement.scrollHeight;
    } catch (err) { }
  }
}
