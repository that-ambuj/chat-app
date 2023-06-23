import { isDevMode } from '@angular/core';

const ws_protocol = window.location.protocol == 'https:' ? 'wss://' : 'ws://';
let ws_host = window.location.hostname;

// Use port 8000 when running in local env
if (isDevMode()) {
  ws_host += ':8000';
}

export const WS_ENDPOINT = ws_protocol + ws_host + '/ws';
