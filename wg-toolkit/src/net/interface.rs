//! This module provides a callback-based interface for sending and
//! receiving bundles' elements.

use std::collections::HashMap;
use std::net::{SocketAddrV4, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::io;

use blowfish::Blowfish;
use rand::rngs::OsRng;
use rand::RngCore;

use thiserror::Error;

use super::bundle::{ElementReader, TopElementReader, BundleElement, BundleResult, BundleError, Bundle, ReplyElementReader, BundleElementWriter};
use super::socket::{WgSocket, Event, EventKind, PacketError};
use super::element::TopElement;
use super::packet::Packet;


pub mod login;
// pub mod base;

pub use login::{LoginInterface, LoginShared};
// pub use base::{BaseAppInterface, BaseAppShared};


/// A callback-based interface for sending and receiving elements, 
/// backed by a [`WgSocket`], itself backed by an [`UdpSocket`].
/// 
/// It take a generic data type that will be stored internally and 
/// passed to callbacks when called, this usually is the shared state 
/// of the application. The shared data will be passed by mutable 
/// reference because this interface is single-threaded.
pub struct Interface<S: Shared> {
    /// The inner socket providing interface for sending and receiving 
    /// bundles of packets (themselves containing elements).
    socket: WgSocket,
    /// The shared data that is passed by mutable reference to all 
    /// callbacks as the first parameter (allowing methods).
    shared: S,
    /// The bundle used to stack elements when writing to peers.
    bundle: Bundle,
    /// The array of callbacks for each top element's id (except for 
    /// the reply element's id, 0xFF).
    top_callbacks: Box<[Option<TopCallback<S>>; 255]>,
    /// Internal request tracker.
    request_manager: RequestManager<S>,
}

impl<S: Shared> Interface<S> {

    /// Internal init value for no callback, used in array initializer.
    const INIT_TOP_CALLBACK: Option<TopCallback<S>> = None;

    /// Try to create a new interface bound to the given socket address,
    /// and using the given shared state.
    pub fn new(addr: SocketAddrV4, shared: S) -> io::Result<Self> {
        Ok(Self {
            socket: WgSocket::new(addr)?,
            shared,
            bundle: Bundle::new(),
            top_callbacks: Box::new([Self::INIT_TOP_CALLBACK; 255]),
            request_manager: RequestManager::new(),
        })
    }

    /// Get a shared reference to the shared state.
    #[inline]
    pub fn shared(&self) -> &S {
        &self.shared
    }

    /// Get a shared mutable reference to the shared state.
    #[inline]
    pub fn shared_mut(&mut self) -> &mut S {
        &mut self.shared
    }

    /// Register a raw callback, such callback only takes the element
    /// reader with the peer. The callback make what it want with the
    /// reader.
    #[inline]
    pub fn register_raw<U>(&mut self, id: u8, callback: U)
    where
        U: 'static + FnMut(&mut S, TopElementReader, Peer<S>) -> BundleResult<bool>,
    {
        assert_ne!(id, 0xFF, "id 0xFF reserved for reply elements");
        self.top_callbacks[id as usize] = Some(Box::new(callback));
    }

    /// Register a callback for the given element id and type. This 
    /// callback accepts the shared state of the interface, the decoded 
    /// element and the socket address of the peer sending this element.
    /// 
    /// This function doesn't accept elements with required configuration.
    #[inline]
    pub fn register_simple<E, U>(&mut self, id: u8, mut callback: U)
    where
        E: TopElement<Config = ()>,
        U: 'static + FnMut(&mut S, BundleElement<E>, Peer<S>),
    {
        self.register_raw(id, move |state, reader, peer| {
            callback(state, reader.read_simple::<E>()?, peer);
            Ok(true) // True to continue reading.
        });
    }

    /// Register a callback for the given element id and type. This 
    /// callback accepts the shared state of the interface, the decoded 
    /// element and the socket address of the peer sending this element.
    /// 
    /// This function accepts elements that takes a config, it must be 
    /// provided by the config supplier closure, that takes the state 
    /// and the peer address.
    #[inline]
    pub fn register<E, C, U, V>(&mut self, id: u8, mut callback: U, mut callback_config: V)
    where
        E: TopElement<Config = C>,
        U: 'static + FnMut(&mut S, BundleElement<E>, Peer<S>),
        V: 'static + FnMut(&mut S, SocketAddr) -> C,
    {
        self.register_raw(id, move |state, reader, peer| {
            let config = callback_config(state, peer.addr);
            callback(state, reader.read::<E>(&config)?, peer);
            Ok(true) // True to continue reading.
        });
    }

    /// Poll events from the underlying socket and route them to 
    /// registered callbacks, while routing events to the watcher.
    pub fn poll(&mut self, events: &mut Vec<Event>, timeout: Option<Duration>) -> Result<(), InterfaceError> {

        self.socket.poll(events, timeout)?;

        for event in events {

            match &event.kind {
                EventKind::Bundle(bundle) => {
                    let mut reader = bundle.element_reader();
                    while let Some(element) = reader.next_element() {
                        self.handle_element(event.addr, element)?;
                    }
                }
                EventKind::PacketError(packet, error) => {
                    self.shared.on_packet_error(&**packet, error);
                }
            }

        }

        Ok(())

    }

    /// Internal function that handle a received bundle element, it 
    /// returns true if if the iterator of elements should continue.
    fn handle_element(&mut self, addr: SocketAddr, element: ElementReader) -> BundleResult<bool> {
        debug_assert!(self.bundle.is_empty());
        match element {
            ElementReader::Top(reader) => {
                // Note: the id should not be equal to 0xFF, which is 
                //   the reply element's ID, so it's safe to .
                if let Some(mut callback) = self.top_callbacks[reader.id() as usize].take() {
                    callback(&mut self.shared, reader, Peer {
                        addr,
                        bundle: &mut self.bundle,
                        socket: &mut self.socket,
                        request_manager: &mut self.request_manager,
                    })
                } else {
                    self.shared.on_element(reader)
                }
            }
            ElementReader::Reply(reader) => {

                if let Some((
                    mut callback, 
                    _instant
                )) = self.request_manager.callbacks.remove(&reader.request_id()) {
                    callback(&mut self.shared, reader, Peer {
                        addr,
                        bundle: &mut self.bundle,
                        socket: &mut self.socket,
                        request_manager: &mut self.request_manager,
                    })
                } else {
                    self.shared.on_reply(reader)
                }

            }
        }
    }

    /// Obtain a handle to a peer given its address. This handle can be
    /// used to sent elements and requests to it. This inherently makes
    /// no IO, but will do when elements are added.
    pub fn peer(&mut self, addr: SocketAddr) -> Peer<S> {
        debug_assert!(self.bundle.is_empty());
        Peer { 
            addr, 
            bundle: &mut self.bundle, 
            socket: &mut self.socket, 
            request_manager: &mut self.request_manager,
        }
    }

}


/// This is called when an element reader is received for the associated 
/// element's id. This closure is not typed for the element's type, but 
/// this closure's implementation will internally read the actual 
/// element's type and transfer it to the user's callback.
type TopCallback<S> = Box<dyn FnMut(&mut S, TopElementReader, Peer<S>) -> BundleResult<bool>>;

/// Same kind of type alias as [`TopCallback`], but for requests' replies.
type ReplyCallback<S> = Box<dyn FnMut(&mut S, ReplyElementReader, Peer<S>) -> BundleResult<bool>>;


/// This trait must be implemented by the shared state type of the
/// interface, it doesn't require to implement functions but you can
/// implement "watcher" functions, that are like global callbacks.
/// 
/// *For now the implementors must be static in order to simplify the
/// internal implementation of the [`Interface`] structure.*
pub trait Shared: 'static {

    /// Called when a packet was lost because it cannot be reconstructed
    /// into a bundle.
    fn on_packet_error(&mut self, packet: &Packet, error: &PacketError) {
        let _ = (packet, error);
    }

    /// Called when an element has no registered callback, in such case 
    /// this function is a fallback responsible for handling or not the 
    /// element. It should return no error and true in order to continue 
    /// iterating over next elements in the bundle.
    fn on_element(&mut self, reader: TopElementReader) -> BundleResult<bool> {
        let _ = reader;
        Ok(false) // Default to false to stop iteration.
    }

    /// Called when a reply has no registered request callback, in such
    /// case this function is a fallback responsible for handling or not
    /// the element. It should return no error and true in order to 
    /// continue iterating over next elements in the bundle.
    fn on_reply(&mut self, reader: ReplyElementReader) -> BundleResult<bool> {
        let _ = reader;
        Ok(false) // Default to false to stop iteration.
    }

}


/// The internal structure used to keep track of requests and allocate
/// new requests handlers.
pub struct RequestManager<S> {
    /// Reply callbacks, we also keeps track of the instant they were
    /// registered in order to clean them when timing out and notify
    /// this to watchers.
    callbacks: HashMap<u32, (ReplyCallback<S>, Instant)>,
    /// Counter for request ids, used to map request ids to their
    /// reply handler. We initialize it to a random id by default.
    next_request_id: u32,
    /// Callbacks pending to be added, this is used by [`InterfacePeer`].
    /// These are only pushed into the real callbacks when the peer
    /// bundle is flushed.
    pending_callbacks: Vec<(u32, ReplyCallback<S>)>,
}

impl<S> RequestManager<S> {

    fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
            next_request_id: OsRng.next_u32(),
            pending_callbacks: Vec::new(),
        }
    }

    /// Flush pending callbacks in the real callbacks map, this is 
    /// internally called by [`ElementPeer`] when it's flushed with 
    /// the bundle.
    fn flush(&mut self) {
        if self.pending_callbacks.is_empty() {
            let instant = Instant::now();
            for (request_id, callback) in self.pending_callbacks.drain(..) {
                self.callbacks.insert(request_id, (callback, instant));
            }
        }
    }

    /// Abort all internal pending callbacks by dropping them. It's 
    /// called by [`ElementPeer`] when its aborted with the bundle.
    fn abort(&mut self) {
        self.pending_callbacks.clear();
    }

    /// Add a pending callback and associate it to a request ID, and
    /// return this ID.
    fn add_pending_callback(&mut self, callback: ReplyCallback<S>) -> u32 {
        let request_id = self.next_request_id;
        self.next_request_id = request_id.wrapping_add(1);
        self.pending_callbacks.push((request_id, callback));
        request_id
    }

    /// Register a raw request callback, such callback only takes the 
    /// reply element reader with the peer. The callback make what it 
    /// want with the reader.
    #[inline]
    pub fn register_raw<U>(&mut self, callback: U) -> u32
    where
        U: 'static + FnMut(&mut S, ReplyElementReader, Peer<S>) -> BundleResult<bool>,
    {
        self.add_pending_callback(Box::new(callback))
    }

    /// Register a request callback, it's only valid for simple elements
    /// and therefore have no provided configuration. The request id
    /// associated to this callback is then returned.
    #[inline]
    pub fn register_simple<E, U>(&mut self, mut callback: U) -> u32
    where
        E: TopElement<Config = ()>,
        U: 'static + FnMut(&mut S, BundleElement<E>, Peer<S>),
    {
        self.register_raw(move |state, reader, peer| {
            callback(state, reader.read_simple::<E>()?, peer);
            Ok(true) // True to continue reading.
        })
    }

    /// Register a request callback, this callback can accept element
    /// with an associated config that is provided by a second closure.
    /// The request id associated to this callback is then returned.
    #[inline]
    pub fn register<E, C, U, V>(&mut self, mut callback: U, mut callback_config: V) -> u32
    where
        E: TopElement<Config = C>,
        U: 'static + FnMut(&mut S, BundleElement<E>, Peer<S>),
        V: 'static + FnMut(&mut S, SocketAddr) -> C,
    {
        self.register_raw(move |state, reader, peer| {
            let config = callback_config(state, peer.addr);
            callback(state, reader.read::<E>(&config)?, peer);
            Ok(true) // True to continue reading.
        })
    }

}


/// The structure represent a handle to a interface's peer.
pub struct Peer<'a, S> {
    /// The peer socket address.
    addr: SocketAddr,
    /// The bundle to write elements into.
    bundle: &'a mut Bundle,
    /// The socket where bundles are sent.
    socket: &'a mut WgSocket,
    /// Request tracker.
    request_manager: &'a mut RequestManager<S>,
}

impl<'a, S> Peer<'a, S> {

    /// Get the peer's socket address.
    #[inline]
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Enable channel encryption for this peer with the given blowfish
    /// key. This key will be used to encrypt and decrypt each next
    /// transaction with this particular peer's socket address.
    #[inline]
    pub fn set_channel(&mut self, blowfish: Arc<Blowfish>) {
        self.socket.set_channel(self.addr, blowfish);
    }

    /// Get the element writer to use for writing elements, if you need
    /// to send requests, you should use the [`RequestManager`] from
    /// [`request_manager`] and register your request's callback to get
    /// the associated request id.
    #[inline]
    pub fn element_writer(&mut self) -> BundleElementWriter<'_> {
        self.bundle.element_writer()
    }

    /// Retrieve the request manager to use for registering requests'
    /// callbacks and get their associated request id that you can use
    /// for writing requests elements.
    #[inline]
    pub fn request_manager(&mut self) -> &mut RequestManager<S> {
        self.request_manager
    }

    /// Flush the content of the bundle to the underlying socket. This
    /// is automatically done when the element is dropped.
    pub fn flush(&mut self) {

        if !self.bundle.is_empty() {

            self.socket.send(self.bundle, self.addr).expect("TODO: Change this");
            self.bundle.clear();

            self.request_manager.flush();
            
        } else {
            // If the bundle is empty, it's useless to keep callbacks
            // because they will never be called under normal conditions.
            self.request_manager.abort();
        }

    }

    /// Abort added elements and request callbacks.
    pub fn abort(&mut self) {
        self.bundle.clear();
        self.request_manager.abort();
    }

}

impl<'a, S> Drop for Peer<'a, S> {
    fn drop(&mut self) {
        self.flush();
    }
}


/// Standard error possible for interface.
#[derive(Debug, Error)]
pub enum InterfaceError {
    /// Underlying bundle error, maybe while reading an element.
    #[error("bundle error: {0}")]
    Bundle(#[from] BundleError),
    /// Underlying IO error while polling events from the socket.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

pub type InterfaceResult<T> = Result<T, InterfaceError>;
