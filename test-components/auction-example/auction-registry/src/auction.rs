use std::env;
use crate::bindings::auction::auction_stub::stub_auction;
use crate::bindings::auction::auction_stub::stub_auction::RunningAuction;

use crate::model::*;

pub fn create(auction: Auction) {
    let component_id = env::var("AUCTION_COMPONENT_ID").expect("AUCTION_COMPONENT_ID not set");
    let uri = stub_auction::Uri {
        value: format!("worker://{component_id}/auction-{}", auction.auction_id.auction_id)
    };
    let worker = stub_auction::Api::new(&uri);
    let wit_auction = auction.into();
    worker.initialize(&wit_auction);
}

pub fn create_res(auction: Auction) -> RunningAuction {
    let component_id = env::var("AUCTION_COMPONENT_ID").expect("AUCTION_COMPONENT_ID not set");
    let uri = stub_auction::Uri {
        value: format!("worker://{component_id}/auction")
    };
    let wit_auction = auction.into();
    RunningAuction::new(&uri, &wit_auction)
}
