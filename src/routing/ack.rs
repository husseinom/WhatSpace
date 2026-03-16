use chrono::Utc;
use uuid::Uuid;

use super::model::{Bundle, BundleKind, MsgStatus};

impl Bundle{

    /// Create an Ack bundle from a successfully delivered Data bundle.
    /// Called by the destination node upon receiving a Data bundle
    /// intended for it. Source and destination are automatically swapped.
    pub fn new_ack(delivered_bundle: &Bundle) -> Self {
        Bundle {
            id: Uuid::new_v4().to_string(),
            // The destination of the Data becomes the source of the Ack
            source: delivered_bundle.destination.clone(),
            // The source of the Data becomes the destination of the Ack
            destination: delivered_bundle.source.clone(),
            timestamp: Utc::now(),
            // Same TTL as the original so the Ack has time to propagate back
            ttl: delivered_bundle.ttl,
            // Ack bundles carry no payload,
            kind: BundleKind::Ack {
                ack_bundle_id: delivered_bundle.id.clone(),
            },
            shipment_status: MsgStatus::Pending,
        }
    }

    /// Returns true if this bundle has exceeded its TTL.
    pub fn is_expired(&self) -> bool {
        let age = Utc::now()
            .signed_duration_since(self.timestamp)
            .num_seconds();
        age > self.ttl as i64
    }
}


