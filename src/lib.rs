use crate::actions::{ActionContext, action_map};
use crate::enums::{IgmpV2MulticastGroupEvents, IgmpV2MulticastGroupStates};
use crate::services::{RoutingService, TimerService};
use crate::transitions::TransitionTable;

pub struct Fsm {
    state: IgmpV2MulticastGroupStates,
    table: TransitionTable,
}
impl Fsm {
    pub fn new(table: TransitionTable) -> Self {
        Self {
            state: IgmpV2MulticastGroupStates::NoMembersPresent,
            table,
        }
    }

    pub fn on_event<T: TimerService, R: RoutingService>(
        &mut self,
        event: IgmpV2MulticastGroupEvents,
        ctx: &ActionContext<T, R>,
    ) {
        if let Some((next_state, actions)) = self.table.get(&(self.state.clone(), event.clone())) {
            for &action in actions {
                let func = action_map(action);
                func(ctx, &event);
            }
            self.state = next_state.clone();
        }
    }

    pub fn state(&self) -> &IgmpV2MulticastGroupStates {
        &self.state
    }
}

pub mod actions;
pub mod enums;
pub mod services;
pub mod transitions;
