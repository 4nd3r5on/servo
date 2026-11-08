/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::context::JSContext;
use script_bindings::reflector::{Reflector, reflect_dom_object};

use crate::dom::bindings::codegen::Bindings::IdleDeadlineBinding::IdleDeadlineMethods;
use crate::dom::bindings::codegen::Bindings::PerformanceBinding::DOMHighResTimeStamp;
use crate::dom::bindings::num::Finite;
use crate::dom::bindings::root::DomRoot;
use crate::dom::globalscope::GlobalScope;

/// <https://w3c.github.io/requestidlecallback/#the-idledeadline-interface>
#[dom_struct]
pub(crate) struct IdleDeadline {
    reflector_: Reflector,
    /// <https://w3c.github.io/requestidlecallback/#dfn-timeout>
    did_timeout: bool,
}

impl IdleDeadline {
    fn new_inherited(did_timeout: bool) -> Self {
        Self {
            reflector_: Reflector::new(),
            did_timeout,
        }
    }

    #[expect(dead_code, reason = "Idle callbacks are not invoked yet")]
    pub(crate) fn new(
        cx: &mut JSContext,
        global: &GlobalScope,
        did_timeout: bool,
    ) -> DomRoot<Self> {
        reflect_dom_object(cx, Box::new(Self::new_inherited(did_timeout)), global)
    }
}

impl IdleDeadlineMethods<crate::DomTypeHolder> for IdleDeadline {
    /// <https://w3c.github.io/requestidlecallback/#dom-idledeadline-timeremaining>
    fn TimeRemaining(&self) -> DOMHighResTimeStamp {
        // TODO: Compute the remaining time from the deadline of the current idle period.
        Finite::wrap(0.0)
    }

    /// <https://w3c.github.io/requestidlecallback/#dom-idledeadline-didtimeout>
    fn DidTimeout(&self) -> bool {
        self.did_timeout
    }
}
