use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{lightclient::lightclient_config::LightClientConfig, lightwallet::data::BlockData};

use super::{block_witness_data::BlockAndWitnessData, nullifier_data::NullifierData, sync_status::SyncStatus};

pub struct BlazeSyncData {
    pub(crate) sync_status: Arc<RwLock<SyncStatus>>,
    pub(crate) nullifier_data: NullifierData,
    pub(crate) block_data: BlockAndWitnessData,
}

impl BlazeSyncData {
    pub fn new(config: &LightClientConfig) -> Self {
        let sync_status = Arc::new(RwLock::new(SyncStatus::default()));

        Self {
            sync_status: sync_status.clone(),
            nullifier_data: NullifierData::new(),
            block_data: BlockAndWitnessData::new(config, sync_status),
        }
    }

    pub async fn setup_for_sync(&mut self, start_block: u64, end_block: u64, existing_blocks: Vec<BlockData>) {
        if start_block < end_block {
            panic!("Blocks should be backwards");
        }

        // Replace the contents with a new syncstatus, essentially clearing it
        {
            let mut guard = self.sync_status.write().await;
            let prev_sync_status = guard.clone();
            (*guard) = SyncStatus::new_sync(prev_sync_status.sync_id + 1, start_block, end_block);
        }

        self.nullifier_data.setup_sync().await;
        self.block_data.setup_sync(existing_blocks).await;
    }

    // Finish up the sync
    pub async fn finish(&self) {
        self.nullifier_data.finish().await;
        self.sync_status.write().await.finish();
    }
}
