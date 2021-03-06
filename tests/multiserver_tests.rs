extern crate httpmock;

use isahc::prelude::*;
use isahc::{get, get_async, HttpClientBuilder};

use httpmock::Method::{GET, POST};
use httpmock::{Mock, MockServer, MockServerRequest, Regex};
use httpmock_macros::test_executors;
use isahc::config::RedirectPolicy;
use std::fs::read_to_string;
use std::time::{Duration, SystemTime};

/// This test shows how to use multiple mock servers in one test.
#[test]
#[test_executors] // Internal macro that executes this test in different async executors. Ignore it.
fn multiple_mock_servers_test() {
    // Arrange
    let _ = env_logger::try_init();
    let mock_server1 = MockServer::start();
    let mock_server2 = MockServer::start();

    let redirect_mock = Mock::new()
        .expect_path("/redirectTest")
        .return_status(302)
        .return_header(
            "Location",
            &format!("http://{}/finalTarget", mock_server2.address()),
        )
        .create_on(&mock_server1);

    let target_mock = Mock::new()
        .expect_path("/finalTarget")
        .return_status(200)
        .create_on(&mock_server2);

    // Act: Send the HTTP request
    let http_client = HttpClientBuilder::new()
        .redirect_policy(RedirectPolicy::Follow)
        .build()
        .unwrap();

    let response = http_client
        .get(&format!("http://{}/redirectTest", mock_server1.address()))
        .unwrap();

    // Assert
    assert_eq!(response.status(), 200);
    assert_eq!(redirect_mock.times_called(), 1);
    assert_eq!(target_mock.times_called(), 1);
}
