import { NgModule } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';
import { ButtonComponent } from './button/button.component';
import { ChatInputComponent } from './chat-input/chat-input.component';
import { TitleComponent } from './title/title.component';

@NgModule({
  imports: [CommonModule, FormsModule],
  declarations: [ButtonComponent, ChatInputComponent, TitleComponent],
  exports: [ButtonComponent, ChatInputComponent, TitleComponent],
})
export class CommonUiModule {}
