/*
    TODO: Most of these integration tests currently rely upon Ipfs. There is an issue open to fix this. See https://github.com/fission-codes/fission-cli/issues/29
*/

const DATA_FOLDER: &'static str = "./src/test/data";

use std::path::Path;

use colored::Colorize;
use futures::executor::block_on;
use serde_json::Value;
use serial_test::serial;

use crate::ipfs::daemon::IpfsDaemon;
use crate::ipfs::Ipfs;
use crate::utils::file_management;

fn run_ipfs_test<T>(test: T) -> ()
where
    T: FnOnce(&IpfsDaemon) -> bool,
{
    let ipfs = IpfsDaemon::default();
    block_on(ipfs.launch()).unwrap();
    let has_passed = test(&ipfs);
    ipfs.shutdown().unwrap();
    assert!(has_passed)
}

fn are_files_uploaded(uploaded_paths: Vec<String>, os_paths: Vec<String>) -> bool {
    let mut is_uploaded = true;
    let drop_point = DATA_FOLDER.split("/").count();
    for os_path in os_paths {
        let fixed_os_path: String = os_path
            .split("/")
            .enumerate()
            .filter_map(|(i, seg)| {
                return if i < drop_point {
                    None
                } else if i == drop_point {
                    Some(seg.to_string())
                } else {
                    Some("/".to_string() + seg)
                };
            })
            .collect();

        let mut is_any_matching = false;
        'any: for uploaded_path in &uploaded_paths {
            if &fixed_os_path == uploaded_path {
                println!("{} == {}", fixed_os_path.green(), uploaded_path.green());
                is_any_matching = true;
                break 'any;
            } else {
                println!("{} == {}", fixed_os_path.red(), uploaded_path.red());
            }
        }
        if !is_any_matching {
            println!("Failed to match {}", fixed_os_path.red());
            is_uploaded = false;
        }
    }
    is_uploaded
}

#[test]
#[serial]
fn can_add_directory() {
    let test_dir = DATA_FOLDER.to_string() + "/more-tests";
    run_ipfs_test(|ipfs| {
        let hashes = block_on(ipfs.add(Path::new(&test_dir))).unwrap();
        println!("{}", "Finished Hashes:".green());
        for (path, hash) in &hashes {
            println!("{}: {}", path.green(), hash.blue())
        }

        let files = file_management::get_files_in(&test_dir).unwrap();

        let uploaded_paths = hashes.into_iter().map(|(path, _)| path).collect();
        let os_paths = files.into_iter().map(|(path, _)| path).collect();
        are_files_uploaded(uploaded_paths, os_paths)
    })
}
#[test]
#[serial]
fn can_add_file() {
    let test_file = DATA_FOLDER.to_string() + "/test.txt";
    run_ipfs_test(|ipfs| {
        let hashes = block_on(ipfs.add(Path::new(&test_file))).unwrap();
        println!("{}", "Finished Hashes:\n".green());
        for (path, hash) in &hashes {
            println!("{}: {}", path.green(), hash.blue())
        }
        let uploaded_paths = hashes.into_iter().map(|(path, _)| path).collect();
        let os_paths = vec![test_file.to_string()];
        are_files_uploaded(uploaded_paths, os_paths)
    })
}

#[test]
#[serial]
fn can_config() {
    let test_prop = "Datastore.StorageMax";
    let test_value = Value::String("11GB".to_string());
    run_ipfs_test(|ipfs| {
        let old_config = block_on(ipfs.get_config(test_prop)).unwrap();
        block_on(ipfs.set_config(test_prop, &test_value)).unwrap();
        let new_config = block_on(ipfs.get_config(test_prop)).unwrap();
        block_on(ipfs.set_config(test_prop, &old_config)).unwrap();
        test_value == new_config
    });
}
#[test]
#[serial]
fn can_connect() {
    let test_peer = "/dns4/production-ipfs-cluster-us-east-1-node2.runfission.com/tcp/4003/wss/p2p/12D3KooWQ2hL9NschcJ1Suqa1TybJc2ZaacqoQMBT3ziFC7Ye2BZ";
    run_ipfs_test(|ipfs| {
        let res = block_on(ipfs.connect_to(test_peer));
        res.is_ok()
    });
}
