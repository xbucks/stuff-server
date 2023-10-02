use bytes::Bytes;
use color_eyre::eyre::Result;
use qp2p::{Endpoint, WireMsg};
use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};

#[derive(Default, Ord, PartialEq, PartialOrd, Eq, Clone, Copy)]
struct XId(pub [u8; 32]);


#[tokio::main]
pub async fn p2p() -> Result<()> {
    color_eyre::install()?;

    const MSG_MARCO: &str = "marco";
    const MSG_POLO: &str = "polo";

    // collect cli args
    let args: Vec<String> = env::args().collect();

    // create an endpoint for us to listen on and send from.
    let (node, mut incoming_conns) = Endpoint::builder()
        .addr((Ipv4Addr::LOCALHOST, 0))
        .idle_timeout(60 * 60 * 1_000 /* 3600s = 1h */)
        .server()?;

    // if we received args then we parse them as SocketAddr and send a "marco" msg to each peer.
    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            let peer: SocketAddr = arg
                .parse()
                .expect("Invalid SocketAddr.  Use the form 127.0.0.1:1234");
            let msg = Bytes::from(MSG_MARCO);
            println!("Sending to {peer:?} --> {msg:?}\n");
            let (conn, mut incoming) = node.connect_to(&peer).await?;
            conn.send((Bytes::new(), Bytes::new(), msg.clone())).await?;
            // `Endpoint` no longer having `connection_pool` to hold established connection.
            // Which means the connection get closed immediately when it reaches end of life span.
            // And causes the receiver side a sending error when reply via the in-coming connection.
            // Hence here have to listen for the reply to avoid such error
            let reply = incoming.next().await.unwrap();
            println!("Received from {peer:?} --> {reply:?}");
        }

        println!("Done sending");
    }

    println!("\n---");
    println!("Listening on: {:?}", node.local_addr());
    println!("---\n");

    // loop over incoming connections
    while let Some((connection, mut incoming)) = incoming_conns.next().await {
        let src = connection.remote_address();

        // loop over incoming messages
        while let Ok(Some(WireMsg((_, _, bytes)))) = incoming.next().await {
            println!("Received from {src:?} --> {bytes:?}");
            if bytes == *MSG_MARCO {
                let reply = Bytes::from(MSG_POLO);
                connection
                    .send((Bytes::new(), Bytes::new(), reply.clone()))
                    .await?;
                println!("Replied to {src:?} --> {reply:?}");
            }
            println!();
        }
    }

    Ok(())
}
