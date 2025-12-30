use async_trait::async_trait;
use qdrant_client::{
    qdrant::{
        vectors_config::Config, CreateCollection, Distance, PointStruct, Value, VectorParams,
        VectorsConfig,
    },
    Qdrant,
};
use sea_orm::DbErr;
use std::collections::HashMap;

#[async_trait]
pub trait QdrantRepo: Send + Sync {
    async fn upsert_memo(
        &self,
        memo_id: i32,
        user_id: i32,
        vector: Vec<f32>,
    ) -> Result<(), DbErr>;

    async fn search_similar(
        &self,
        user_id: i32,
        query_vector: Vec<f32>,
        limit: u64,
    ) -> Result<Vec<i32>, DbErr>;

    async fn delete_memo(&self, memo_id: i32) -> Result<(), DbErr>;
}

#[derive(Clone)]
pub struct QdrantRepository {
    client: Qdrant,
    collection_name: String,
}

impl QdrantRepository {
    pub async fn new(qdrant_url: String) -> Result<Self, DbErr> {
        let client = Qdrant::from_url(&qdrant_url)
            .build()
            .map_err(|e| DbErr::Custom(format!("Failed to create Qdrant client: {}", e)))?;

        let repo = Self {
            client,
            collection_name: "memo_embeddings".to_string(),
        };

        repo.ensure_collection().await?;

        Ok(repo)
    }

    pub async fn ensure_collection(&self) -> Result<(), DbErr> {
        let collections = self
            .client
            .list_collections()
            .await
            .map_err(|e| DbErr::Custom(format!("Failed to list collections: {}", e)))?;

        let collection_exists = collections
            .collections
            .iter()
            .any(|c| c.name == self.collection_name);

        if !collection_exists {
            self.client
                .create_collection(CreateCollection {
                    collection_name: self.collection_name.clone(),
                    vectors_config: Some(VectorsConfig {
                        config: Some(Config::Params(VectorParams {
                            size: 768,
                            distance: Distance::Cosine.into(),
                            ..Default::default()
                        })),
                    }),
                    ..Default::default()
                })
                .await
                .map_err(|e| DbErr::Custom(format!("Failed to create collection: {}", e)))?;
        }

        Ok(())
    }
}

#[async_trait]
impl QdrantRepo for QdrantRepository {
    async fn upsert_memo(
        &self,
        memo_id: i32,
        user_id: i32,
        vector: Vec<f32>,
    ) -> Result<(), DbErr> {
        use qdrant_client::qdrant::UpsertPoints;

        let mut payload: HashMap<String, Value> = HashMap::new();
        payload.insert("user_id".to_string(), (user_id as i64).into());
        payload.insert("memo_id".to_string(), (memo_id as i64).into());

        let point = PointStruct::new(memo_id as u64, vector, payload);

        self.client
            .upsert_points(UpsertPoints {
                collection_name: self.collection_name.clone(),
                points: vec![point],
                ..Default::default()
            })
            .await
            .map_err(|e| DbErr::Custom(format!("Failed to upsert memo: {}", e)))?;

        Ok(())
    }

    async fn search_similar(
        &self,
        user_id: i32,
        query_vector: Vec<f32>,
        limit: u64,
    ) -> Result<Vec<i32>, DbErr> {
        use qdrant_client::qdrant::{Condition, Filter, SearchPoints};

        let search_result = self
            .client
            .search_points(SearchPoints {
                collection_name: self.collection_name.clone(),
                vector: query_vector,
                limit,
                filter: Some(Filter::must([Condition::matches(
                    "user_id",
                    user_id as i64,
                )])),
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await
            .map_err(|e| DbErr::Custom(format!("Failed to search similar memos: {}", e)))?;

        let memo_ids = search_result
            .result
            .into_iter()
            .filter_map(|point| {
                point
                    .payload
                    .get("memo_id")
                    .and_then(|v| v.as_integer())
                    .map(|id| id as i32)
            })
            .collect();

        Ok(memo_ids)
    }

    async fn delete_memo(&self, memo_id: i32) -> Result<(), DbErr> {
        use qdrant_client::qdrant::{
            points_selector::PointsSelectorOneOf, DeletePoints, PointsIdsList, PointsSelector,
        };

        self.client
            .delete_points(DeletePoints {
                collection_name: self.collection_name.clone(),
                points: Some(PointsSelector {
                    points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                        ids: vec![(memo_id as u64).into()],
                    })),
                }),
                ..Default::default()
            })
            .await
            .map_err(|e| DbErr::Custom(format!("Failed to delete memo: {}", e)))?;

        Ok(())
    }
}
