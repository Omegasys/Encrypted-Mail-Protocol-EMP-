use crate::onion::node::Node;
use crate::routing::path_selection::PathSelector;

#[test]
fn test_path_selection() {
    let nodes = vec![
        Node::new("1".into(), "a".into(), vec![]),
        Node::new("2".into(), "b".into(), vec![]),
        Node::new("3".into(), "c".into(), vec![]),
        Node::new("4".into(), "d".into(), vec![]),
    ];

    let path = PathSelector::select_path(&nodes, 3);

    assert_eq!(path.len(), 3);
}
