use colored::Colorize;
use futures::executor::block_on;
use serde_json::Value;
use serial_test::serial;

use self::daemon::IpfsDaemon;
use super::*;
use crate::ipfs::Ipfs;
use crate::utils::file_management;

//TODO: Setup pizza
const PEER_ADDRS:&'static  [&'static str] = &[
        "/dns4/production-ipfs-cluster-us-east-1-node2.runfission.com/tcp/4003/wss/p2p/12D3KooWQ2hL9NschcJ1Suqa1TybJc2ZaacqoQMBT3ziFC7Ye2BZ",
        "/dns4/production-ipfs-cluster-eu-north-1-node1.runfission.com/tcp/4003/wss/p2p/12D3KooWRwbRrSN2cPAKz4yt1vxBFdh53CpgWjSFK5hZPkzHHz5h",
        "/dns4/production-ipfs-cluster-eu-north-1-node1.runfission.com/tcp/4001/p2p/12D3KooWRwbRrSN2cPAKz4yt1vxBFdh53CpgWjSFK5hZPkzHHz5h",
        "/dns4/production-ipfs-cluster-mega-us-east-1-node0.runfission.com/tcp/4001/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/ip4/54.235.17.70/tcp/4001/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/ip4/54.235.17.70/udp/4001/quic/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/dns4/production-ipfs-cluster-mega-us-east-1-node0.runfission.com/tcp/4003/wss/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/dns4/production-ipfs-cluster-us-east-1-node2.runfission.com/tcp/4001/p2p/12D3KooWQ2hL9NschcJ1Suqa1TybJc2ZaacqoQMBT3ziFC7Ye2BZ",
        "/dns4/production-ipfs-cluster-eu-north-1-node0.runfission.com/tcp/4001/p2p/12D3KooWDTUTdVJfW7Rwb6kKhceEwevTatPXnavPwkfZp2A6r1Fn",
        "/dns4/production-ipfs-cluster-us-east-1-node1.runfission.com/tcp/4001/p2p/12D3KooWNntMEXRUa2dNgkQsVgzao6zGSYxm1oAs83YtRy6uBuxv",
        "/dns4/production-ipfs-cluster-eu-north-1-node0.runfission.com/tcp/4003/wss/p2p/12D3KooWDTUTdVJfW7Rwb6kKhceEwevTatPXnavPwkfZp2A6r1Fn",
        "/ip4/54.235.17.70/tcp/4003/wss/p2p/12D3KooWJQHUo1snJrv5NWVesFspjwhkNkaMu5M9cMdF1oF7ucTz",
        "/dns4/production-ipfs-cluster-us-east-1-node1.runfission.com/tcp/4003/wss/p2p/12D3KooWNntMEXRUa2dNgkQsVgzao6zGSYxm1oAs83YtRy6uBuxv",
    ];

fn connect_to_peers() -> IpfsDaemon {
    let mut ipfs = IpfsDaemon::new().unwrap();
    for peer in PEER_ADDRS {
        block_on(ipfs.connect_to(peer)).unwrap();
        println!("Connected to peer! {}", peer);
    }
    return ipfs;
}

#[test]
#[serial]
fn can_add_directory() {
    let test_dir = "./test-dir/more-tests";
    let mut ipfs = connect_to_peers();
    let hashes = block_on(ipfs.add_directory(test_dir)).unwrap();
    println!("{}", "Finnished Hashes:".green());
    for (path, hash) in &hashes {
        println!("{}: {}", path.green(), hash.blue())
    }

    let files = file_management::get_files_in(test_dir).unwrap();
    let folders = file_management::get_dirs_in(test_dir).unwrap();

    files
        .iter()
        .map(|(path, _)| path)
        .chain(folders.iter())
        .for_each(|path_to_match| {
            let fixed_path_to_match: String = path_to_match
                .split("/")
                .filter(|seg| seg != &"." && !seg.is_empty())
                .map(|seg| "/".to_string() + seg)
                .collect();
            assert!(hashes
                .iter()
                .any(|(path, _)| fixed_path_to_match == path.to_owned()))
        });
}
#[test]
#[serial]
fn can_add_file() {
    let test_file = "./test-dir/test.txt";
    let mut ipfs = connect_to_peers();
    let hashes = block_on(ipfs.add_file(test_file)).unwrap();
    println!("{}", "Finnished Hashes:\n".green());
    for (path, hash) in &hashes {
        println!("{}: {}", path.green(), hash.blue())
    }
    let matchable_path: String = test_file
        .split("/")
        .filter(|seg| seg != &"." && !seg.is_empty())
        .map(|seg| "/".to_string() + seg)
        .collect();
    assert!(hashes
        .iter()
        .any(|(path, _)| matchable_path == path.to_owned()));
}
#[test]
#[serial]
fn can_config() {
    let mut ipfs = IpfsDaemon::new().unwrap();
    let mut config = block_on(ipfs.get_config()).unwrap();
    config
        .datastore
        .as_object_mut()
        .unwrap()
        .insert("StorageMax".to_string(), Value::String("11GB".to_string()));
    config
        .datastore
        .as_object_mut()
        .unwrap()
        .insert("GCPeriod".to_string(), Value::String("2h".to_string()));
    block_on(ipfs.set_config(&config)).unwrap();
    config
        .datastore
        .as_object_mut()
        .unwrap()
        .insert("StorageMax".to_string(), Value::String("10GB".to_string()));
    config
        .datastore
        .as_object_mut()
        .unwrap()
        .insert("GCPeriod".to_string(), Value::String("1h".to_string()));
    block_on(ipfs.set_config(&config)).unwrap();
    assert!(true)
}
