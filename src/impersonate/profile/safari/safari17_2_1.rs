use boring::ssl::{
    CertCompressionAlgorithm, SslConnector, SslConnectorBuilder, SslCurve, SslMethod, SslOptions,
    SslVersion,
};
use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    HeaderMap, HeaderValue,
};
use std::sync::Arc;

use crate::impersonate::{Http2Data, ImpersonateSettings};

pub(crate) fn get_settings(headers: HeaderMap) -> ImpersonateSettings {
    ImpersonateSettings {
        tls_builder_func: Arc::new(create_ssl_connector),
        http2: Http2Data {
            initial_stream_window_size: Some(4194304),
            initial_connection_window_size: Some(10551295),
            max_concurrent_streams: Some(100),
            max_header_list_size: None,
            header_table_size: None,
            enable_push: Some(false),
        },
        headers: create_headers(headers),
        gzip: true,
        brotli: true,
    }
}

fn create_ssl_connector(h2: bool) -> SslConnectorBuilder {
    let mut builder = SslConnector::builder(SslMethod::tls_client()).unwrap();

    builder.set_default_verify_paths().unwrap();



    builder.set_options(SslOptions::NO_TICKET);

    builder.set_grease_enabled(true);

    builder.enable_ocsp_stapling();

    let cipher_list = [
        "TLS_AES_128_GCM_SHA256",
        "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
        "TLS_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_RSA_WITH_AES_128_GCM_SHA256",
        "TLS_RSA_WITH_AES_256_CBC_SHA",
        "TLS_RSA_WITH_AES_128_CBC_SHA",
        "TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA",
        "TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA",
        "TLS_RSA_WITH_3DES_EDE_CBC_SHA",
    ];

    builder.set_cipher_list(&cipher_list.join(":")).unwrap();

    let sigalgs_list = [
        "ecdsa_secp256r1_sha256",
        "rsa_pss_rsae_sha256",
        "rsa_pkcs1_sha256",
        "ecdsa_secp384r1_sha384",
        "ecdsa_sha1",
        "rsa_pss_rsae_sha384",
        "rsa_pss_rsae_sha384",
        "rsa_pkcs1_sha384",
        "rsa_pss_rsae_sha512",
        "rsa_pkcs1_sha512",
        "rsa_pkcs1_sha1",
    ];

    builder.set_sigalgs_list(&sigalgs_list.join(":")).unwrap();

    builder
        .set_curves(&[
            SslCurve::X25519,
            SslCurve::SECP256R1,
            SslCurve::SECP384R1,
            SslCurve::SECP521R1,
        ])
        .unwrap();

    if h2 {
        builder.set_alpn_protos(b"\x02h2\x08http/1.1").unwrap();
    } else {
        builder.set_alpn_protos(b"\x08http/1.1").unwrap();
    }

    builder.enable_signed_cert_timestamps();

    builder
        .add_cert_compression_alg(CertCompressionAlgorithm::Zlib)
        .unwrap();

    builder
        .set_min_proto_version(Some(SslVersion::TLS1))
        .unwrap();

    builder
}

fn create_headers(mut headers: HeaderMap) -> HeaderMap {
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"),
    );
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Safari/605.1.15"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );

    headers
}
