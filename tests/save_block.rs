mod utils;

#[cfg(test)]
mod save_block {
    use std::fs;
    use tendermint::block::Block;
    use tendermint_rpc::endpoint::block_results;

    use crate::utils::{create_test_db, destroy_test_db, helper_db, TESTING_DB_NAME};

    #[tokio::test]
    async fn save_block() {
        let helper_db = helper_db().await;

        destroy_test_db(helper_db.pool(), TESTING_DB_NAME).await;

        // now create a fresh database for tests
        let db = create_test_db(helper_db.pool(), TESTING_DB_NAME).await;

        let data = fs::read_to_string("./tests/blocks_vector.json").unwrap();
        let blocks: Vec<Block> = serde_json::from_str(&data).unwrap();
        let data = fs::read_to_string("./tests/block_results_vector.json").unwrap();
        let block_results: Vec<block_results::Response> = serde_json::from_str(&data).unwrap();

        db.create_tables().await.unwrap();

        for i in 0..blocks.len() {
            db.save_block(&blocks[i], &block_results[i]).await.unwrap();
        }

        db.create_indexes()
            .await
            .expect("Something went wrong creating database indexes");
    }
}
