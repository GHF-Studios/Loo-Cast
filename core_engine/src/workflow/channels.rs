use bevy::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::{Mutex, MutexGuard, OnceLock};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use super::events::*;
use super::request::*;
use super::response::*;

pub struct ChannelsPlugin;

// --- Workflow Request Receivers ---
#[derive(Resource)]
pub(super) struct WorkflowRequestReceiver(pub UnboundedReceiver<TypedWorkflowRequest>);
#[derive(Resource)]
pub(super) struct WorkflowRequestEReceiver(pub UnboundedReceiver<TypedWorkflowRequestE>);
#[derive(Resource)]
pub(super) struct WorkflowRequestOReceiver(pub UnboundedReceiver<TypedWorkflowRequestO>);
#[derive(Resource)]
pub(super) struct WorkflowRequestOEReceiver(pub UnboundedReceiver<TypedWorkflowRequestOE>);
#[derive(Resource)]
pub(super) struct WorkflowRequestIReceiver(pub UnboundedReceiver<TypedWorkflowRequestI>);
#[derive(Resource)]
pub(super) struct WorkflowRequestIEReceiver(pub UnboundedReceiver<TypedWorkflowRequestIE>);
#[derive(Resource)]
pub(super) struct WorkflowRequestIOReceiver(pub UnboundedReceiver<TypedWorkflowRequestIO>);
#[derive(Resource)]
pub(super) struct WorkflowRequestIOEReceiver(pub UnboundedReceiver<TypedWorkflowRequestIOE>);

// --- Workflow Response Senders ---
#[derive(Resource)]
pub(super) struct WorkflowResponseSender(pub UnboundedSender<TypedWorkflowResponse>);
#[derive(Resource)]
pub(super) struct WorkflowResponseESender(pub UnboundedSender<TypedWorkflowResponseE>);
#[derive(Resource)]
pub(super) struct WorkflowResponseOSender(pub UnboundedSender<TypedWorkflowResponseO>);
#[derive(Resource)]
pub(super) struct WorkflowResponseOESender(pub UnboundedSender<TypedWorkflowResponseOE>);
#[derive(Resource)]
pub(super) struct WorkflowResponseISender(pub UnboundedSender<TypedWorkflowResponse>);
#[derive(Resource)]
pub(super) struct WorkflowResponseIESender(pub UnboundedSender<TypedWorkflowResponseE>);
#[derive(Resource)]
pub(super) struct WorkflowResponseIOSender(pub UnboundedSender<TypedWorkflowResponseO>);
#[derive(Resource)]
pub(super) struct WorkflowResponseIOESender(pub UnboundedSender<TypedWorkflowResponseOE>);

// --- Stage Channels ---
static STAGE_SETUP_SENDER: OnceLock<Sender<StageSetupEvent>> = OnceLock::new();
static STAGE_WAIT_SENDER: OnceLock<Sender<StageWaitEvent>> = OnceLock::new();
static STAGE_COMPLETION_SENDER: OnceLock<Sender<StageCompletionEvent>> = OnceLock::new();
static STAGE_FAILURE_SENDER: OnceLock<Sender<StageFailureEvent>> = OnceLock::new();

pub(super) fn initialize_stage_channels() -> (
    Receiver<StageSetupEvent>,
    Receiver<StageWaitEvent>,
    Receiver<StageCompletionEvent>,
    Receiver<StageFailureEvent>,
) {
    let (setup_sender, setup_receiver) = unbounded();
    let (wait_sender, wait_receiver) = unbounded();
    let (completion_sender, completion_receiver) = unbounded();
    let (failure_sender, failure_receiver) = unbounded();

    STAGE_SETUP_SENDER
        .set(setup_sender.clone())
        .expect("Setup sender already initialized!");
    STAGE_WAIT_SENDER
        .set(wait_sender)
        .expect("Wait sender already initialized!");
    STAGE_COMPLETION_SENDER
        .set(completion_sender)
        .expect("Completion sender already initialized!");
    STAGE_FAILURE_SENDER
        .set(failure_sender)
        .expect("Failure sender already initialized!");

    (
        setup_receiver,
        wait_receiver,
        completion_receiver,
        failure_receiver,
    )
}

pub fn get_stage_setup_sender() -> Sender<StageSetupEvent> {
    STAGE_SETUP_SENDER
        .get()
        .expect("Setup sender not initialized!")
        .clone()
}
pub fn get_stage_wait_sender() -> Sender<StageWaitEvent> {
    STAGE_WAIT_SENDER
        .get()
        .expect("Wait sender not initialized!")
        .clone()
}
pub fn get_stage_completion_sender() -> Sender<StageCompletionEvent> {
    STAGE_COMPLETION_SENDER
        .get()
        .expect("Completion sender not initialized!")
        .clone()
}
pub fn get_stage_failure_sender() -> Sender<StageFailureEvent> {
    STAGE_FAILURE_SENDER
        .get()
        .expect("Failure sender not initialized!")
        .clone()
}

// --- Workflow Channels ---
macro_rules! init_tokio_channel_pair {
    ($req_sender:ident, $resp_receiver:ident, $req_val:expr, $resp_val:expr) => {
        if $req_sender.set($req_val).is_err() {
            unreachable!("Request sender already initialized!");
        }
        if $resp_receiver.set(Mutex::new($resp_val)).is_err() {
            unreachable!("Response receiver already initialized!");
        }
    };
}

static REQUEST_SENDER: OnceLock<UnboundedSender<TypedWorkflowRequest>> = OnceLock::new();
static RESPONSE_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponse>>> = OnceLock::new();
static REQUEST_E_SENDER: OnceLock<UnboundedSender<TypedWorkflowRequestE>> = OnceLock::new();
static RESPONSE_E_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseE>>> =
    OnceLock::new();
static REQUEST_O_SENDER: OnceLock<UnboundedSender<TypedWorkflowRequestO>> = OnceLock::new();
static RESPONSE_O_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseO>>> =
    OnceLock::new();
static REQUEST_OE_SENDER: OnceLock<UnboundedSender<TypedWorkflowRequestOE>> = OnceLock::new();
static RESPONSE_OE_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseOE>>> =
    OnceLock::new();
static REQUEST_I_SENDER: OnceLock<UnboundedSender<TypedWorkflowRequestI>> = OnceLock::new();
static RESPONSE_I_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponse>>> = OnceLock::new();
static REQUEST_IE_SENDER: OnceLock<UnboundedSender<TypedWorkflowRequestIE>> = OnceLock::new();
static RESPONSE_IE_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseE>>> =
    OnceLock::new();
static REQUEST_IO_SENDER: OnceLock<UnboundedSender<TypedWorkflowRequestIO>> = OnceLock::new();
static RESPONSE_IO_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseO>>> =
    OnceLock::new();
static REQUEST_IOE_SENDER: OnceLock<UnboundedSender<TypedWorkflowRequestIOE>> = OnceLock::new();
static RESPONSE_IOE_RECEIVER: OnceLock<Mutex<UnboundedReceiver<TypedWorkflowResponseOE>>> =
    OnceLock::new();

macro_rules! define_tokio_init_fn {
    ($fn_name:ident, $req_sender:ident, $resp_receiver:ident, $Req:ty, $Resp:ty) => {
        pub(super) fn $fn_name() -> (UnboundedReceiver<$Req>, UnboundedSender<$Resp>) {
            let (request_tx, request_rx) = unbounded_channel();
            let (response_tx, response_rx) = unbounded_channel();
            init_tokio_channel_pair!($req_sender, $resp_receiver, request_tx, response_rx);
            (request_rx, response_tx)
        }
    };
}

define_tokio_init_fn!(
    initialize_channels,
    REQUEST_SENDER,
    RESPONSE_RECEIVER,
    TypedWorkflowRequest,
    TypedWorkflowResponse
);
define_tokio_init_fn!(
    initialize_e_channels,
    REQUEST_E_SENDER,
    RESPONSE_E_RECEIVER,
    TypedWorkflowRequestE,
    TypedWorkflowResponseE
);
define_tokio_init_fn!(
    initialize_o_channels,
    REQUEST_O_SENDER,
    RESPONSE_O_RECEIVER,
    TypedWorkflowRequestO,
    TypedWorkflowResponseO
);
define_tokio_init_fn!(
    initialize_oe_channels,
    REQUEST_OE_SENDER,
    RESPONSE_OE_RECEIVER,
    TypedWorkflowRequestOE,
    TypedWorkflowResponseOE
);
define_tokio_init_fn!(
    initialize_i_channels,
    REQUEST_I_SENDER,
    RESPONSE_I_RECEIVER,
    TypedWorkflowRequestI,
    TypedWorkflowResponse
);
define_tokio_init_fn!(
    initialize_ie_channels,
    REQUEST_IE_SENDER,
    RESPONSE_IE_RECEIVER,
    TypedWorkflowRequestIE,
    TypedWorkflowResponseE
);
define_tokio_init_fn!(
    initialize_io_channels,
    REQUEST_IO_SENDER,
    RESPONSE_IO_RECEIVER,
    TypedWorkflowRequestIO,
    TypedWorkflowResponseO
);
define_tokio_init_fn!(
    initialize_ioe_channels,
    REQUEST_IOE_SENDER,
    RESPONSE_IOE_RECEIVER,
    TypedWorkflowRequestIOE,
    TypedWorkflowResponseOE
);

// --- Getters ---
macro_rules! define_sender_getter {
    ($fn_name:ident, $static_ref:ident, $T:ty) => {
        pub fn $fn_name() -> $T {
            $static_ref
                .get()
                .expect(concat!(
                    stringify!($fn_name),
                    " accessed before initialization!"
                ))
                .clone()
        }
    };
}

macro_rules! define_receiver_getter {
    ($fn_name:ident, $static_ref:ident, $T:ty) => {
        pub fn $fn_name() -> MutexGuard<'static, $T> {
            $static_ref
                .get()
                .expect(concat!(
                    stringify!($fn_name),
                    " accessed before initialization!"
                ))
                .lock()
                .unwrap()
        }
    };
}

// Sender Getters
define_sender_getter!(
    get_request_sender,
    REQUEST_SENDER,
    UnboundedSender<TypedWorkflowRequest>
);
define_sender_getter!(
    get_request_e_sender,
    REQUEST_E_SENDER,
    UnboundedSender<TypedWorkflowRequestE>
);
define_sender_getter!(
    get_request_o_sender,
    REQUEST_O_SENDER,
    UnboundedSender<TypedWorkflowRequestO>
);
define_sender_getter!(
    get_request_oe_sender,
    REQUEST_OE_SENDER,
    UnboundedSender<TypedWorkflowRequestOE>
);
define_sender_getter!(
    get_request_i_sender,
    REQUEST_I_SENDER,
    UnboundedSender<TypedWorkflowRequestI>
);
define_sender_getter!(
    get_request_ie_sender,
    REQUEST_IE_SENDER,
    UnboundedSender<TypedWorkflowRequestIE>
);
define_sender_getter!(
    get_request_io_sender,
    REQUEST_IO_SENDER,
    UnboundedSender<TypedWorkflowRequestIO>
);
define_sender_getter!(
    get_request_ioe_sender,
    REQUEST_IOE_SENDER,
    UnboundedSender<TypedWorkflowRequestIOE>
);

// Receiver Getters
define_receiver_getter!(
    get_response_receiver,
    RESPONSE_RECEIVER,
    UnboundedReceiver<TypedWorkflowResponse>
);
define_receiver_getter!(
    get_response_e_receiver,
    RESPONSE_E_RECEIVER,
    UnboundedReceiver<TypedWorkflowResponseE>
);
define_receiver_getter!(
    get_response_o_receiver,
    RESPONSE_O_RECEIVER,
    UnboundedReceiver<TypedWorkflowResponseO>
);
define_receiver_getter!(
    get_response_oe_receiver,
    RESPONSE_OE_RECEIVER,
    UnboundedReceiver<TypedWorkflowResponseOE>
);
define_receiver_getter!(
    get_response_i_receiver,
    RESPONSE_I_RECEIVER,
    UnboundedReceiver<TypedWorkflowResponse>
);
define_receiver_getter!(
    get_response_ie_receiver,
    RESPONSE_IE_RECEIVER,
    UnboundedReceiver<TypedWorkflowResponseE>
);
define_receiver_getter!(
    get_response_io_receiver,
    RESPONSE_IO_RECEIVER,
    UnboundedReceiver<TypedWorkflowResponseO>
);
define_receiver_getter!(
    get_response_ioe_receiver,
    RESPONSE_IOE_RECEIVER,
    UnboundedReceiver<TypedWorkflowResponseOE>
);
