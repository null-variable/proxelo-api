use prost::Message;
use proxelo_api::snapshot::v1::{
    Action, PathMatch, ProxyAction, Route, Site, Snapshot, action, path_match,
};

#[test]
fn snapshot_roundtrip() {
    let snap = Snapshot {
        revision: 7,
        sites: vec![Site {
            routes: vec![Route {
                path: Some(PathMatch {
                    kind: path_match::Kind::Prefix as i32,
                    value: "/api/".into(),
                }),
                action: Some(Action {
                    kind: Some(action::Kind::Proxy(ProxyAction { upstream: 0 })),
                }),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let bytes = snap.encode_to_vec();
    let back = Snapshot::decode(bytes.as_slice()).expect("decode");
    assert_eq!(back, snap);
}
