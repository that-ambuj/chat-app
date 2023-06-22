import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { AppComponent } from './app.component';
import { NxWelcomeComponent } from './nx-welcome.component';

import { CommonUiModule } from '@chat-app/common-ui';
import { ChatWindowComponent } from './chat-window/chat-window.component';

@NgModule({
  declarations: [AppComponent, NxWelcomeComponent, ChatWindowComponent],
  imports: [BrowserModule, CommonUiModule],
  providers: [],
  bootstrap: [AppComponent],
  exports: [ChatWindowComponent],
})
export class AppModule {}
