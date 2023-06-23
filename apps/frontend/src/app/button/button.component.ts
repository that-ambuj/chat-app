import { Component, Input } from '@angular/core';

@Component({
  selector: 'chat-app-button',
  templateUrl: './button.component.html',
})
export class ButtonComponent {
  @Input() text = '';
}
