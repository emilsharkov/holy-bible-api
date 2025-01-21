use tower_http::compression::CompressionLayer;

pub fn get_compression_layer() -> CompressionLayer {
    CompressionLayer::new()
}