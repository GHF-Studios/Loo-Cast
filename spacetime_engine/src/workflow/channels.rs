use once_cell::sync::OnceCell;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};
use bevy::prelude::*;

use super::request::*;
use super::response::*;

pub struct ChannelsPlugin;

// --- Workflow Request Receivers ---
#[derive(Resource)]
pub struct WorkflowRequestReceiver(pub UnboundedReceiver<TypedWorkflowRequest>);
#[derive(Resource)]
pub struct WorkflowRequestEReceiver(pub UnboundedReceiver<TypedWorkflowRequestE>);
#[derive(Resource)]
pub struct WorkflowRequestOReceiver(pub UnboundedReceiver<TypedWorkflowRequestO>);
#[derive(Resource)]
pub struct WorkflowRequestOEReceiver(pub UnboundedReceiver<TypedWorkflowRequestOE>);
#[derive(Resource)]
pub struct WorkflowRequestIReceiver(pub UnboundedReceiver<TypedWorkflowRequestI>);
#[derive(Resource)]
pub struct WorkflowRequestIEReceiver(pub UnboundedReceiver<TypedWorkflowRequestIE>);
#[derive(Resource)]
pub struct WorkflowRequestIOReceiver(pub UnboundedReceiver<TypedWorkflowRequestIO>);
#[derive(Resource)]
pub struct WorkflowRequestIOEReceiver(pub UnboundedReceiver<TypedWorkflowRequestIOE>);

// --- Workflow Response Senders ---
#[derive(Resource)]
pub struct WorkflowResponseSender(pub UnboundedSender<()>);
#[derive(Resource)]
pub struct WorkflowResponseESender(pub UnboundedSender<TypedWorkflowResponseE>);
#[derive(Resource)]
pub struct WorkflowResponseOSender(pub UnboundedSender<TypedWorkflowResponseO>);
#[derive(Resource)]
pub struct WorkflowResponseOESender(pub UnboundedSender<TypedWorkflowResponseOE>);
#[derive(Resource)]
pub struct WorkflowResponseISender(pub UnboundedSender<()>);
#[derive(Resource)]
pub struct WorkflowResponseIESender(pub UnboundedSender<TypedWorkflowResponseE>);
#[derive(Resource)]
pub struct WorkflowResponseIOSender(pub UnboundedSender<TypedWorkflowResponseO>);
#[derive(Resource)]
pub struct WorkflowResponseIOESender(pub UnboundedSender<TypedWorkflowResponseOE>);

static REQUEST_SENDER: OnceCell<UnboundedSender<TypedWorkflowRequest>> = OnceCell::new();
static RESPONSE_RECEIVER: OnceCell<UnboundedReceiver<()>> = OnceCell::new();
static REQUEST_E_SENDER: OnceCell<UnboundedSender<TypedWorkflowRequestE>> = OnceCell::new();
static RESPONSE_E_RECEIVER: OnceCell<UnboundedReceiver<TypedWorkflowResponseE>> = OnceCell::new();
static REQUEST_O_SENDER: OnceCell<UnboundedSender<TypedWorkflowRequestO>> = OnceCell::new();
static RESPONSE_O_RECEIVER: OnceCell<UnboundedReceiver<TypedWorkflowResponseO>> = OnceCell::new();
static REQUEST_OE_SENDER: OnceCell<UnboundedSender<TypedWorkflowRequestOE>> = OnceCell::new();
static RESPONSE_OE_RECEIVER: OnceCell<UnboundedReceiver<TypedWorkflowResponseOE>> = OnceCell::new();
static REQUEST_I_SENDER: OnceCell<UnboundedSender<TypedWorkflowRequestI>> = OnceCell::new();
static RESPONSE_I_RECEIVER: OnceCell<UnboundedReceiver<()>> = OnceCell::new();
static REQUEST_IE_SENDER: OnceCell<UnboundedSender<TypedWorkflowRequestIE>> = OnceCell::new();
static RESPONSE_IE_RECEIVER: OnceCell<UnboundedReceiver<TypedWorkflowResponseE>> = OnceCell::new();
static REQUEST_IO_SENDER: OnceCell<UnboundedSender<TypedWorkflowRequestIO>> = OnceCell::new();
static RESPONSE_IO_RECEIVER: OnceCell<UnboundedReceiver<TypedWorkflowResponseO>> = OnceCell::new();
static REQUEST_IOE_SENDER: OnceCell<UnboundedSender<TypedWorkflowRequestIOE>> = OnceCell::new();
static RESPONSE_IOE_RECEIVER: OnceCell<UnboundedReceiver<TypedWorkflowResponseOE>> = OnceCell::new();

pub(in crate) fn initialize_channels() -> (
    UnboundedReceiver<TypedWorkflowRequest>,
    UnboundedSender<()>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_SENDER.set(request_tx).is_err();
    let response_err = RESPONSE_RECEIVER.set(response_rx).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in crate) fn initialize_e_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestE>,
    UnboundedSender<TypedWorkflowResponseE>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_E_SENDER.set(request_tx).is_err();
    let response_err = RESPONSE_E_RECEIVER.set(response_rx).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in crate) fn initialize_o_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestO>,
    UnboundedSender<TypedWorkflowResponseO>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_O_SENDER.set(request_tx).is_err();
    let response_err = RESPONSE_O_RECEIVER.set(response_rx).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in crate) fn initialize_oe_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestOE>,
    UnboundedSender<TypedWorkflowResponseOE>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_OE_SENDER.set(request_tx).is_err();
    let response_err = RESPONSE_OE_RECEIVER.set(response_rx).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in crate) fn initialize_i_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestI>,
    UnboundedSender<()>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_I_SENDER.set(request_tx).is_err();
    let response_err = RESPONSE_I_RECEIVER.set(response_rx).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in crate) fn initialize_ie_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestIE>,
    UnboundedSender<TypedWorkflowResponseE>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_IE_SENDER.set(request_tx).is_err();
    let response_err = RESPONSE_IE_RECEIVER.set(response_rx).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in crate) fn initialize_io_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestIO>,
    UnboundedSender<TypedWorkflowResponseO>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_IO_SENDER.set(request_tx).is_err();
    let response_err = RESPONSE_IO_RECEIVER.set(response_rx).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}
pub(in crate) fn initialize_ioe_channels() -> (
    UnboundedReceiver<TypedWorkflowRequestIOE>,
    UnboundedSender<TypedWorkflowResponseOE>,
) {
    let (request_tx, request_rx) = unbounded_channel();
    let (response_tx, response_rx) = unbounded_channel();

    let request_err = REQUEST_IOE_SENDER.set(request_tx).is_err();
    let response_err = RESPONSE_IOE_RECEIVER.set(response_rx).is_err();

    if request_err {
        panic!("Request sender already initialized!");
    }
    if response_err {
        panic!("Response receiver already initialized!");
    }

    (request_rx, response_tx)
}

pub fn get_request_sender() -> &'static UnboundedSender<TypedWorkflowRequest> {
    REQUEST_SENDER.get().expect("Request sender accessed before initialization!")
}
pub fn get_response_receiver() -> &'static UnboundedReceiver<()> {
    RESPONSE_RECEIVER.get().expect("Response receiver accessed before initialization!")
}
pub fn get_request_e_sender() -> &'static UnboundedSender<TypedWorkflowRequestE> {
    REQUEST_E_SENDER.get().expect("Request sender accessed before initialization!")
}
pub fn get_response_e_receiver() -> &'static UnboundedReceiver<TypedWorkflowResponseE> {
    RESPONSE_E_RECEIVER.get().expect("Response receiver accessed before initialization!")
}
pub fn get_request_o_sender() -> &'static UnboundedSender<TypedWorkflowRequestO> {
    REQUEST_O_SENDER.get().expect("Request sender accessed before initialization!")
}
pub fn get_response_o_receiver() -> &'static UnboundedReceiver<TypedWorkflowResponseO> {
    RESPONSE_O_RECEIVER.get().expect("Response receiver accessed before initialization!")
}
pub fn get_request_oe_sender() -> &'static UnboundedSender<TypedWorkflowRequestOE> {
    REQUEST_OE_SENDER.get().expect("Request sender accessed before initialization!")
}
pub fn get_response_oe_receiver() -> &'static UnboundedReceiver<TypedWorkflowResponseOE> {
    RESPONSE_OE_RECEIVER.get().expect("Response receiver accessed before initialization!")
}
pub fn get_request_i_sender() -> &'static UnboundedSender<TypedWorkflowRequestI> {
    REQUEST_I_SENDER.get().expect("Request sender accessed before initialization!")
}
pub fn get_response_i_receiver() -> &'static UnboundedReceiver<()> {
    RESPONSE_I_RECEIVER.get().expect("Response receiver accessed before initialization!")
}
pub fn get_request_ie_sender() -> &'static UnboundedSender<TypedWorkflowRequestIE> {
    REQUEST_IE_SENDER.get().expect("Request sender accessed before initialization!")
}
pub fn get_response_ie_receiver() -> &'static UnboundedReceiver<TypedWorkflowResponseE> {
    RESPONSE_IE_RECEIVER.get().expect("Response receiver accessed before initialization!")
}
pub fn get_request_io_sender() -> &'static UnboundedSender<TypedWorkflowRequestIO> {
    REQUEST_IO_SENDER.get().expect("Request sender accessed before initialization!")
}
pub fn get_response_io_receiver() -> &'static UnboundedReceiver<TypedWorkflowResponseO> {
    RESPONSE_IO_RECEIVER.get().expect("Response receiver accessed before initialization!")
}
pub fn get_request_ioe_sender() -> &'static UnboundedSender<TypedWorkflowRequestIOE> {
    REQUEST_IOE_SENDER.get().expect("Request sender accessed before initialization!")
}
pub fn get_response_ioe_receiver() -> &'static UnboundedReceiver<TypedWorkflowResponseOE> {
    RESPONSE_IOE_RECEIVER.get().expect("Response receiver accessed before initialization!")
}
