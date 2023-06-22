import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { AppComponent } from './app.component';

import { CommonUiModule } from '@chat-app/common-ui';
import { ChatWindowComponent } from './chat-window/chat-window.component';

@NgModule({
  declarations: [AppComponent, ChatWindowComponent],
  imports: [BrowserModule, CommonUiModule],
  providers: [],
  bootstrap: [AppComponent],
  exports: [ChatWindowComponent],
})
export class AppModule {}
