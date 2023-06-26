import { isDevMode } from '@angular/core';

const ws_protocol = window.location.protocol == 'https:' ? 'wss://' : 'ws://';
let ws_host = window.location.host;

const port = window.location.port;

// Use port 8000 if the app is not being served from the backend
if (
  (!!port && port !== '80' && port !== '443' && port !== '8000') ||
  isDevMode()
) {
  const ws_host_arr = ws_host.split(':');
  if (ws_host_arr.length > 1) {
    ws_host_arr.pop();
    ws_host_arr.push('8000');
    ws_host = ws_host_arr.join(':');
  }
}

export const WS_ENDPOINT = ws_protocol + ws_host + '/ws';
