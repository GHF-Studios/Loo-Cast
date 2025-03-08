use std::sync::{Mutex, MutexGuard, OnceLock};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use bevy::prelude::*;

use super::request::*;
use super::response::*;

pub struct ChannelsPlugin;

// --- Workflow Request Receivers ---
#[derive(Resource)]
pub(in super) struct WorkflowRequestReceiver(pub UnboundedReceiver<TypedWorkflowRequest>);
#[derive(Resource)]
pub(in super) struct WorkflowRequestEReceiver(pub UnboundedReceiver<TypedWorkflowRequestE>);
#[derive(Resource)]
pub(in super) struct WorkflowRequestOReceiver(pub UnboundedReceiver<TypedWorkflowRequestO>);
#[derive(Resource)]
pub(in super) struct WorkflowRequestOEReceiver(pub UnboundedReceiver<TypedWorkflowRequestOE>);
#[derive(Resource)]
pub(in super) struct WorkflowRequestIReceiver(pub UnboundedReceiver<TypedWorkflowRequestI>);
#[derive(Resource)]
pub(in super) struct WorkflowRequestIEReceiver(pub UnboundedReceiver<TypedWorkflowRequestIE>);
#[derive(Resource)]
pub(in super) struct WorkflowRequestIOReceiver(pub UnboundedReceiver<TypedWorkflowRequestIO>);
#[derive(Resource)]
pub(in super) struct WorkflowRequestIOEReceiver(pub UnboundedReceiver<TypedWorkflowRequestIOE>);

// --- Workflow Response Senders ---
#[derive(Resource)]
pub(in super) struct WorkflowResponseSender(pub UnboundedSender<()>);
#[derive(Resource)]
pub(in super) struct WorkflowResponseESender(pub UnboundedSender<TypedWorkflowResponseE>);
#[derive(Resource)]
pub(in super) struct WorkflowResponseOSender(pub UnboundedSender<TypedWorkflowResponseO>);
#[derive(Resource)]
pub(in super) struct WorkflowResponseOESender(pub UnboundedSender<TypedWorkflowResponseOE>);
#[derive(Resource)]
pub(in super) struct WorkflowResponseISender(pub UnboundedSender<()>);
#[derive(Resource)]
pub(in super) struct WorkflowResponseIESender(pub UnboundedSender<TypedWorkflowResponseE>);
#[derive(Resource)]
pub(in super) struct WorkflowResponseIOSender(pub UnboundedSender<TypedWorkflowResponseO>);
#[derive(Resource)]
pub(in super) struct WorkflowResponseIOESender(pub UnboundedSender<TypedWorkflowResponseOE>);

static REQUEST_SENDER: OnceLock<Mutex<UnboundedSender<TypedWorkflowRequest>>> = OnceLock::new();
static RESPONSE_RECEIVER: OnceLock<Mutex<UnboundedReceiver<()>>> = OnceLock::new();
static REQUEST_E_SENDER: OnceLock<Mutex<UnboundedSender<TypedWorkflowRequestE>>> = OnceLock::new();
static RESPONSE_E_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseE>>> = OnceLock::new();
static REQUEST_O_SENDER: OnceLock<Mutex<UnboundedSender<TypedWorkflowRequestO>>> = OnceLock::new();
static RESPONSE_O_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseO>>> = OnceLock::new();
static REQUEST_OE_SENDER: OnceLock<Mutex<UnboundedSender<TypedWorkflowRequestOE>>> = OnceLock::new();
static RESPONSE_OE_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseOE>>> = OnceLock::new();
static REQUEST_I_SENDER: OnceLock<Mutex<UnboundedSender<TypedWorkflowRequestI>>> = OnceLock::new();
static RESPONSE_I_RECEIVER: OnceLock<Mutex<UnboundedReceiver<()>>> = OnceLock::new();
static REQUEST_IE_SENDER: OnceLock<Mutex<UnboundedSender<TypedWorkflowRequestIE>>> = OnceLock::new();
static RESPONSE_IE_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseE>>> = OnceLock::new();
static REQUEST_IO_SENDER: OnceLock<Mutex<UnboundedSender<TypedWorkflowRequestIO>>> = OnceLock::new();
static RESPONSE_IO_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseO>>> = OnceLock::new();
static REQUEST_IOE_SENDER: OnceLock<Mutex<UnboundedSender<TypedWorkflowRequestIOE>>> = OnceLock::new();
static RESPONSE_IOE_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseOE>>> = OnceLock::new();

pub(in super) fn initialize_channels() -> (
    UnboundedReceiver<TypedWorkflowRequest>,
    UnboundedSender<()>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_SENDER.set(Mutex::new(request_tx)).is_err();
    let response_err = RESPONSE_RECEIVER.set(Mutex::new(response_rx)).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in super) fn initialize_e_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestE>,
    UnboundedSender<TypedWorkflowResponseE>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_E_SENDER.set(Mutex::new(request_tx)).is_err();
    let response_err = RESPONSE_E_RECEIVER.set(Mutex::new(response_rx)).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in super) fn initialize_o_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestO>,
    UnboundedSender<TypedWorkflowResponseO>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_O_SENDER.set(Mutex::new(request_tx)).is_err();
    let response_err = RESPONSE_O_RECEIVER.set(Mutex::new(response_rx)).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in super) fn initialize_oe_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestOE>,
    UnboundedSender<TypedWorkflowResponseOE>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_OE_SENDER.set(Mutex::new(request_tx)).is_err();
    let response_err = RESPONSE_OE_RECEIVER.set(Mutex::new(response_rx)).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in super) fn initialize_i_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestI>,
    UnboundedSender<()>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_I_SENDER.set(Mutex::new(request_tx)).is_err();
    let response_err = RESPONSE_I_RECEIVER.set(Mutex::new(response_rx)).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in super) fn initialize_ie_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestIE>,
    UnboundedSender<TypedWorkflowResponseE>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_IE_SENDER.set(Mutex::new(request_tx)).is_err();
    let response_err = RESPONSE_IE_RECEIVER.set(Mutex::new(response_rx)).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in super) fn initialize_io_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestIO>,
    UnboundedSender<TypedWorkflowResponseO>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_IO_SENDER.set(Mutex::new(request_tx)).is_err();
    let response_err = RESPONSE_IO_RECEIVER.set(Mutex::new(response_rx)).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in super) fn initialize_ioe_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestIOE>,
    UnboundedSender<TypedWorkflowResponseOE>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_IOE_SENDER.set(Mutex::new(request_tx)).is_err();
    let response_err = RESPONSE_IOE_RECEIVER.set(Mutex::new(response_rx)).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}

pub fn get_request_sender() -> MutexGuard<'static, UnboundedSender<TypedWorkflowRequest>>  {
    REQUEST_SENDER.get().expect("Request sender accessed before initialization!").lock().unwrap()
}
pub fn get_response_receiver() -> MutexGuard<'static, UnboundedReceiver<()>> {
    RESPONSE_RECEIVER.get().expect("Response receiver accessed before initialization!").lock().unwrap()
}
pub fn get_request_e_sender() -> MutexGuard<'static, UnboundedSender<TypedWorkflowRequestE>> {
    REQUEST_E_SENDER.get().expect("Request sender accessed before initialization!").lock().unwrap()
}
pub fn get_response_e_receiver() -> MutexGuard<'static, UnboundedReceiver<TypedWorkflowResponseE>> {
    RESPONSE_E_RECEIVER.get().expect("Response receiver accessed before initialization!").lock().unwrap()
}
pub fn get_request_o_sender() -> MutexGuard<'static, UnboundedSender<TypedWorkflowRequestO>> {
    REQUEST_O_SENDER.get().expect("Request sender accessed before initialization!").lock().unwrap()
}
pub fn get_response_o_receiver() -> MutexGuard<'static, UnboundedReceiver<TypedWorkflowResponseO>> {
    RESPONSE_O_RECEIVER.get().expect("Response receiver accessed before initialization!").lock().unwrap()
}
pub fn get_request_oe_sender() -> MutexGuard<'static, UnboundedSender<TypedWorkflowRequestOE>> {
    REQUEST_OE_SENDER.get().expect("Request sender accessed before initialization!").lock().unwrap()
}
pub fn get_response_oe_receiver() -> MutexGuard<'static, UnboundedReceiver<TypedWorkflowResponseOE>> {
    RESPONSE_OE_RECEIVER.get().expect("Response receiver accessed before initialization!").lock().unwrap()
}
pub fn get_request_i_sender() -> MutexGuard<'static, UnboundedSender<TypedWorkflowRequestI>> {
    REQUEST_I_SENDER.get().expect("Request sender accessed before initialization!").lock().unwrap()
}
pub fn get_response_i_receiver() -> MutexGuard<'static, UnboundedReceiver<()>> {
    RESPONSE_I_RECEIVER.get().expect("Response receiver accessed before initialization!").lock().unwrap()
}
pub fn get_request_ie_sender() -> MutexGuard<'static, UnboundedSender<TypedWorkflowRequestIE>> {
    REQUEST_IE_SENDER.get().expect("Request sender accessed before initialization!").lock().unwrap()
}
pub fn get_response_ie_receiver() -> MutexGuard<'static, UnboundedReceiver<TypedWorkflowResponseE>> {
    RESPONSE_IE_RECEIVER.get().expect("Response receiver accessed before initialization!").lock().unwrap()
}
pub fn get_request_io_sender() -> MutexGuard<'static, UnboundedSender<TypedWorkflowRequestIO>> {
    REQUEST_IO_SENDER.get().expect("Request sender accessed before initialization!").lock().unwrap()
}
pub fn get_response_io_receiver() -> MutexGuard<'static, UnboundedReceiver<TypedWorkflowResponseO>> {
    RESPONSE_IO_RECEIVER.get().expect("Response receiver accessed before initialization!").lock().unwrap()
}
pub fn get_request_ioe_sender() -> MutexGuard<'static, UnboundedSender<TypedWorkflowRequestIOE>> {
    REQUEST_IOE_SENDER.get().expect("Request sender accessed before initialization!").lock().unwrap()
}
pub fn get_response_ioe_receiver() -> MutexGuard<'static, UnboundedReceiver<TypedWorkflowResponseOE>> {
    RESPONSE_IOE_RECEIVER.get().expect("Response receiver accessed before initialization!").lock().unwrap()
}
