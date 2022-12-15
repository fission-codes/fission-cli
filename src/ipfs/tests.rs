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


fn run_ipfs_test<T>(test: T) -> ()
    where T: FnOnce(&IpfsDaemon) -> bool
{
    let mut ipfs = IpfsDaemon::new().unwrap();
    // for peer in PEER_ADDRS {
    //     block_on(ipfs.connect_to(peer)).unwrap();
    //     println!("Connected to peer! {}", peer);
    // }
    block_on(ipfs.launch()).unwrap();
    let has_passed = test(&ipfs);
    ipfs.shutdown().unwrap();
    assert!(has_passed)
}

fn are_files_uploaded(uploaded_paths:Vec<String>, os_paths:Vec<String>) -> bool{
    let mut is_uploaded = true;
    for os_path in os_paths{
        let mut fixed_path_os_path = String::new();
        let mut i = 0;
        for seg in os_path.split("/") {
            if seg != "." && !seg.is_empty() {
                fixed_path_os_path += &(match i {
                    0 => String::new(),
                    1 => seg.to_string(),
                    _ => "/".to_string() + seg
                });
                i += 1; 
            }
        }

        let mut is_any_matching = false;
        'any: for uploaded_path in &uploaded_paths {
            if &fixed_path_os_path == uploaded_path {
                println!("{} == {}", fixed_path_os_path.green(), uploaded_path.green());
                is_any_matching = true;
                break 'any;
            }else{
                println!("{} == {}", fixed_path_os_path.red(), uploaded_path.red());
            }
        };
        if !is_any_matching{
            println!("Failed to match {}", fixed_path_os_path.red());
            is_uploaded = false;
        }
    }
    is_uploaded
}

#[test]
#[serial]
fn can_add_directory() {
    let test_dir = "./test-dir/more-tests";
    run_ipfs_test(|ipfs| {
        
        let hashes = block_on(ipfs.add(Path::new(test_dir))).unwrap();
        println!("{}", "Finnished Hashes:".green());
        for (path, hash) in &hashes {
            println!("{}: {}", path.green(), hash.blue())
        }

        let files = file_management::get_files_in(test_dir).unwrap();
        
        let uploaded_paths = hashes.into_iter()
            .map(|(path, _)| path)
            .collect();
        let os_paths = files.into_iter()
            .map(|(path, _)| path)
            .collect();
        are_files_uploaded(uploaded_paths, os_paths)
    })
}
#[test]
#[serial]
fn can_add_file() {
    let test_file = "./test-dir/test.txt";
    run_ipfs_test(|ipfs| {
        let hashes = block_on(ipfs.add(Path::new(test_file))).unwrap();
        println!("{}", "Finnished Hashes:\n".green());
        for (path, hash) in &hashes {
            println!("{}: {}", path.green(), hash.blue())
        }
        let uploaded_paths = hashes.into_iter()
            .map(|(path, _)| path)
            .collect();
        let os_paths = vec![test_file.to_string()];
        are_files_uploaded(uploaded_paths, os_paths)
    })    
}

// #[test]
// #[serial]
// fn can_config() {
//     let mut ipfs = IpfsDaemon::new().unwrap();
//     let mut config = block_on(ipfs.get_config()).unwrap();
//     config
//         .datastore
//         .as_object_mut()
//         .unwrap()
//         .insert("StorageMax".to_string(), Value::String("11GB".to_string()));
//     config
//         .datastore
//         .as_object_mut()
//         .unwrap()
//         .insert("GCPeriod".to_string(), Value::String("2h".to_string()));
//     block_on(ipfs.set_config(&config)).unwrap();
//     config
//         .datastore
//         .as_object_mut()
//         .unwrap()
//         .insert("StorageMax".to_string(), Value::String("10GB".to_string()));
//     config
//         .datastore
//         .as_object_mut()
//         .unwrap()
//         .insert("GCPeriod".to_string(), Value::String("1h".to_string()));
//     block_on(ipfs.set_config(&config)).unwrap();
//     assert!(true)
// }
