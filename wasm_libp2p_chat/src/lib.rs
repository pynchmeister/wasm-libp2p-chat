// Import necessary crates
use libp2p::{identity, floodsub::{self, Floodsub, FloodsubEvent}, Swarm, Transport};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::console;

// Create an async wrapper function
#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
    // Initialize logging for debugging purposes
    console_error_panic_hook::set_once();
    console::log_1(&"Running libp2p chat".into());

    // Generate a keypair for the local node
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = identity::PeerId::from(local_key.public());

    // Create a WebSocket transport
    let transport = libp2p::wasm_ext::ffi::websocket_transport();

    // Create a Floodsub protocol
    let floodsub_topic = floodsub::Topic::new("wasm-libp2p-chat");
    let mut floodsub = Floodsub::new(local_peer_id.clone());

    // Subscribe to the chat topic
    floodsub.subscribe(floodsub_topic.clone());

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let noise_keys = libp2p::noise::Keypair::<libp2p::noise::X25519Spec>::new().into_authentic(&local_key).unwrap();
        let noise = libp2p::noise::NoiseConfig::xx(noise_keys).into_authenticated();
        let mut swarm_builder = Swarm::builder(local_key.clone().into());
        swarm_builder.add_transport(transport);
        swarm_builder.add_protocol(noise);
        swarm_builder.add_protocol(floodsub.clone());
        swarm_builder.build()
    };

    // Connect to a bootstrap node
    let addr = "/dnsaddr/bootstrap.libp2p.io/tcp/443/wss/p2p-websocket-star";
    let addr = addr.parse().unwrap();
    swarm.dial_addr(addr).unwrap();

    // Set up an event listener
    let swarm_stream = async_stream::stream! {
        loop {
            let event = swarm
            .next().await;
            yield event;
            }
    };
    // Run the event loop to handle events from the Swarm
    let mut swarm_stream = swarm_stream.boxed_local();
    while let Some(event) = JsFuture::from(future_to_promise(swarm_stream.next())).await?.into_serde().unwrap() {
    match event {
        // Handle incoming messages
        FloodsubEvent::Message(message) => {
            let decoded_msg = std::str::from_utf8(&message.data).unwrap();
            console::log_2(&"Received a message from".into(), &message.source.to_string().into());
            console::log_1(&format!("Message: {}", decoded_msg).into());
        }
        _ => {}
    }
}

Ok(())
  }

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
// Set the panic hook for better error messages in the console
std::panic::set_hook(Box::new(console_error_panic_hook::hook));
// Start the application
wasm_bindgen_futures::spawn_local(run());

Ok(())


