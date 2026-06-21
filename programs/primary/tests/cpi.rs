use litesvm::LiteSVM;
use solana_address::Address;
use solana_instruction::{account_meta::AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;
use std::path::Path;

fn add_program(svm: &mut LiteSVM, program_id: &Address, name: &str) {
    let so_dir = std::env::var("SBF_OUT_DIR")
        .expect("SBF_OUT_DIR must point to the directory containing the built .so files");
    let path = Path::new(&so_dir).join(format!("{name}.so"));
    svm.add_program_from_file(*program_id, &path)
        .unwrap_or_else(|e| panic!("failed to load {}: {e:?}", path.display()));
}

#[test]
fn make_a_cpi() {
    let primary_id = Address::new_unique();
    let cpi_target_id = Address::new_unique();

    // `new_debuggable(true)` installs the register-tracing callback that honors
    // `SBF_TRACE_DIR` (where traces are written) and `SBF_DEBUG_PORT` (where the
    // VM blocks waiting for a debugger). That callback is what Gimlet attaches to.
    let mut svm = LiteSVM::new_debuggable(true);
    add_program(&mut svm, &primary_id, "primary");
    add_program(&mut svm, &cpi_target_id, "cpi_target");

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let instruction = Instruction {
        program_id: primary_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(cpi_target_id, false),
        ],
        data: cpi_target_id.to_bytes().to_vec(),
    };

    let tx = Transaction::new(
        &[&payer],
        Message::new(&[instruction], Some(&payer.pubkey())),
        svm.latest_blockhash(),
    );

    let meta = svm
        .send_transaction(tx)
        .expect("primary -> cpi_target CPI should succeed");

    for line in &meta.logs {
        println!("{line}");
    }
}
