use serde::{ Deserialize};

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
struct Config {
        global: Option<GlobalConfig>,
        proxy: Option<ProxyConfig>,
        web:Option<WebConfig>,
        db_node_list:Option<Vec<DBNodeConfig>>,
        db_cluster_list:Option<Vec<DBClusterConfig>>,
        db_shard_schema_list:Option<Vec<DBShardSchemaConfig>>,
}

#[derive(Debug, Deserialize)]
struct GlobalConfig {
    log_path: Option<String>,
    log_level: Option<String>,
    log_slow_query_time: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct ProxyConfig {
    listen_addr: Option<String>,
    charset: Option<String>,
    proxy_users: Option<Vec<ProxyUser>>,
}

#[derive(Debug, Deserialize)]
struct ProxyUser {
    user: Option<String>,
    pwd: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WebConfig {
    listen_addr: Option<String>,
    web_user: Option<String>,
    web_pwd: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DBNodeConfig {
    id: Option<String>,
    listen_addr: Option<String>,
    user: Option<String>,
    pwd: Option<String>,
    time_to_no_alive: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct DBClusterConfig {
    id: Option<String>,
    master_node_id: Option<String>,
    slave_node_id_list: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct DBShardSchemaConfig {
        owner: Option<String>,
        db: Option<String>,
        table: Option<String>,
        shard_key: Option<String>,
        db_cluster_id_list: Option<Vec<String>>,
        default_write_to: Option<String>,
        shard_type:Option<String>,
        each_cluster_table_split_count: Option<Vec<u16>>,
        day_range:Option<Vec<String>>,
}

fn main() {
    let toml_str = r#"
    #Sparrow Mysql/MariaDB Shard proxy config.
    #example config.

    [global]
    log_path = "/home/xxx/log/"
    #log level: debug/info/warn/error, default error.
    log_level = "debug"
    #only log query which take time > log_slow_query_time ms.
    log_slow_query_time = 100

    [proxy]
    listen_addr = "0.0.0.0:9696"
    charset = "gbk"
    #proxy user auth.
    proxy_users = [

        {user = "root", pwd = "root"},
        {user = "sparrow",pwd = "sparrow"}

    ]

    [web]
    listen_addr = "0.0.0.0:9797"
    web_user = "admin"
    web_pwd = "admin"


    #db  instance list.
    [[db_node_list]]
    id = "mysql_1"
    listen_addr = "0.0.0.0:9797"
    user = "root"
    pwd = "root"
    time_to_no_alive = 100 #ms
    #max_conns_limit = 100
    #---------

    [[db_node_list]]
    id = "mysql_2"
    listen_addr = "0.0.0.0:9798"
    user = "root"
    pwd = "root"
    time_to_no_alive = 100 #ms
    #max_conns_limit = 100
    #---------

    [[db_node_list]]
    id = "mysql_3"
    listen_addr = "0.0.0.0:9799"
    user = "root"
    pwd = "root"
    time_to_no_alive = 100 #ms
    #max_conns_limit = 100
    #---------

    [[db_node_list]]
    id = "mysql_4"
    listen_addr = "0.0.0.0:9897"
    user = "root"
    pwd = "root"
    time_to_no_alive = 100 #ms
    #max_conns_limit = 100
    #---------

    [[db_node_list]]
    id = "mysql_5"
    listen_addr = "0.0.0.0:9997"
    user = "root"
    pwd = "root"
    time_to_no_alive = 100 #ms
    #max_conns_limit = 100
    #---------

    [[db_node_list]]
    id = "mysql_6"
    listen_addr = "0.0.0.0:9899"
    user = "root"
    pwd = "root"
    time_to_no_alive = 100 #ms
    #max_conns_limit = 100
    #---------

    # define mysql cluster list.
    [[db_cluster_list]]
    id = "cluster_1"
    master_node_id = "mysql_1"
    slave_node_id_list= ["mysql_2", "mysql_3"]

    [[db_cluster_list]]
    id = "cluster_2"
    master_node_id = "mysql_4"
    slave_node_id_list  = ["mysql_5", "mysql_6"]

    #define db shard router info for each proxy user.
    #the "owner" which  is a proxy user  name.
    [[db_shard_schema_list]]
    owner = "root"
    db = "test_db"
    table = "test_table"
    shard_key = "id"
    db_cluster_id_list = ["cluster_1", "cluster_2"]
    default_write_to  = "cluster_1"
    each_cluster_table_split_count = [ 2,7]
    shard_type = "hash"

    #---
     [[db_shard_schema_list]]
    owner = "root"
    db = "sparrow"
    table = "date_day_table"
    shard_key = "date_day"
    db_cluster_id_list = ["cluster_1", "cluster_2"]
    default_write_to = "cluster_1"
    shard_type = "date_day"
     # which means left close and right  open range , such as: [ 20191102, 20201102)
    day_range = [ "2019-11-02", "2020-11-02", "2021-11-02", "2022-11-02"]
    #---
    [[db_shard_schema_list]]
    owner = "root"
    db = "ordinal_db"
    table = "test_table"
    shard_key = "id"
    db_cluster_id_list = ["cluster_1", "cluster_2"]
    default_write_to = "cluster_1"
    each_cluster_table_split_count = [ 2,7]
    shard_type = "ordinal_number "

    "#;

    let decoded: Config = toml::de::from_str(toml_str).unwrap();
    println!("{:#?}", decoded);
}
