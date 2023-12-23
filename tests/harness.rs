use fuels::{prelude::*, types::ContractId, tx::Receipt};

abigen!(Script(
    name = "MyScript",
    abi = "./out/debug/log_script-abi.json"
));

#[tokio::test]
async fn can_get_contract_id() {
    let wallet = launch_provider_and_get_wallet().await.unwrap();
    let bin_path = "./out/debug/log_script.bin";
    let script_instance = MyScript::new(wallet.clone(), bin_path);
 
    let result = script_instance.main().call().await.unwrap();
    // println!("{:?}", result);

    for item in result.receipts.iter() {
        match item {
            Receipt::LogData{ data, ra, .. } => {
                let data_str: String = data.clone().unwrap().iter().map(|b| format!("{:02x}", b)).collect();
                println!("Data: {} (len {})", data_str, ra);
            },
            _ => {},
        }
    }

    // let tx = wallet.provider().unwrap().get_transaction_by_id(&result.tx_id.unwrap()).await.unwrap().unwrap();
    // println!("{:?}", tx);
}
