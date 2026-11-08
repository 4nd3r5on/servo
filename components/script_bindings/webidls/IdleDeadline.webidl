/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// Source: requestIdleCallback() (https://w3c.github.io/requestidlecallback/)
// Direct source: https://github.com/w3c/webref/blob/curated/ed/idl/requestidlecallback.idl

partial interface Window {
  [Pref="dom_request_idle_callback_enabled"] unsigned long requestIdleCallback(IdleRequestCallback callback, optional IdleRequestOptions options = {});
  [Pref="dom_request_idle_callback_enabled"] undefined cancelIdleCallback(unsigned long handle);
};

dictionary IdleRequestOptions {
  unsigned long timeout;
};

[Exposed=Window, Pref="dom_request_idle_callback_enabled"] interface IdleDeadline {
  DOMHighResTimeStamp timeRemaining();
  readonly attribute boolean didTimeout;
};

callback IdleRequestCallback = undefined (IdleDeadline deadline);
