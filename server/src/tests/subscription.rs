use chrono;

use prelude::*;

fn make_subscription() -> Subscription {
    let subscription_interval = 1000f64;
    Subscription::new(0, true, subscription_interval, 9, 3, 0)
}

#[test]
fn basic_subscription() {
    let s = make_subscription();
    assert!(s.state == SubscriptionState::Creating);
}

#[test]
fn update_state_3() {
    let mut s = make_subscription();
    let mut publish_requests: Vec<PublishRequest> = vec!();

    // Test #3 - state changes from Creating -> Normal
    let now = chrono::UTC::now();
    let items_changed = false;
    let publishing_timer_expired = false;
    let (handled_state, action) = s.update_state(&mut publish_requests, &now, items_changed, publishing_timer_expired);

    assert_eq!(handled_state, 3);
    assert_eq!(action, UpdateStateAction::None);
    assert_eq!(s.message_sent, false);
    assert_eq!(s.state, SubscriptionState::Normal);
}

#[test]
fn update_state_4() {
    // Test #4 -
    // Create a subscription in the normal state, and an incoming publish request. Tick on a subscription
    // with no changes and ensure the request is still queued afterwards

    let mut s = make_subscription();
    let mut publish_requests: Vec<PublishRequest> = vec!(
        PublishRequest {
            request_header: RequestHeader::new(&NodeId::null(), &DateTime::now(), 1),
            subscription_acknowledgements: None,
        }
    );

    s.state = SubscriptionState::Normal;
    s.publishing_enabled = false;

    // Receive Publish Request
    //    &&
    //    (
    //        PublishingEnabled == FALSE
    //            ||
    //            (PublishingEnabled == TRUE
    //                && MoreNotifications == FALSE)
    //    )

    let now = chrono::UTC::now();
    let items_changed = false;
    let publishing_timer_expired = false;
    let (handled_state, action) = s.update_state(&mut publish_requests, &now, items_changed, publishing_timer_expired);

    assert_eq!(handled_state, 4);
    assert_eq!(action, UpdateStateAction::None);
    assert_eq!(s.state, SubscriptionState::Normal);
    assert_eq!(publish_requests.len(), 1);
}

#[test]
fn update_state_5() {
    // x
}

#[test]
fn update_state_6() {
    // x
}

#[test]
fn update_state_7() {
    // x
}

#[test]
fn update_state_8() {
    // x
}

#[test]
fn update_state_9() {
    // x
}

#[test]
fn update_state_10() {
    // x
}

#[test]
fn update_state_11() {
    // x
}

#[test]
fn update_state_12() {
    // x
}

#[test]
fn update_state_13() {
    // x
}

#[test]
fn update_state_14() {
    // x
}

#[test]
fn update_state_15() {
    // x
}

#[test]
fn update_state_16() {
    // x
}

#[test]
fn update_state_17() {
    // x
}