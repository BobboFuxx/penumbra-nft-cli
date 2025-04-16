use clap::{Parser, Subcommand};
use penumbra_nft::{
    mint::mint_nft,
    transfer::transfer_nft,
    staking::{stake_nft, unstake_nft},
    airdrop::airdrop_nft,
    view::reveal_nft,
    ibc::{export_nft_for_ibc, import_nft_from_ibc},
    types::NFTMetadata,
    state::NFTState,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Mint {
        owner: String,
        name: String,
        description: String,
        image_cid: String,
        attributes: String,
    },
    Transfer {
        id: String,
        to: String,
    },
    Stake {
        id: String,
    },
    Unstake {
        id: String,
    },
    Airdrop {
        id: String,
        recipients: Vec<String>,
    },
    View {
        id: String,
    },
    IbcExport {
        id: String,
    },
    IbcImport {
        serialized: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut state = NFTState::new();

    match cli.command {
        Commands::Mint { owner, name, description, image_cid, attributes } => {
            let metadata = NFTMetadata {
                name,
                description,
                image_cid,
                attributes,
                shielded: true,
            };
            let id = mint_nft(&mut state, owner, metadata, Some(5));
            println!("Minted NFT ID: {}", id);
        }

        Commands::Transfer { id, to } => {
            match transfer_nft(&mut state, &id, &to) {
                Ok(_) => println!("Transferred"),
                Err(e) => eprintln!("{}", e),
            }
        }

        Commands::Stake { id } => {
            match stake_nft(&mut state, &id) {
                Ok(_) => println!("Staked"),
                Err(e) => eprintln!("{}", e),
            }
        }

        Commands::Unstake { id } => {
            match unstake_nft(&mut state, &id) {
                Ok(_) => println!("Unstaked"),
                Err(e) => eprintln!("{}", e),
            }
        }

        Commands::Airdrop { id, recipients } => {
            match airdrop_nft(&mut state, &id, recipients) {
                Ok(_) => println!("Airdropped"),
                Err(e) => eprintln!("{}", e),
            }
        }

        Commands::View { id } => {
            if let Some(nft) = reveal_nft(&state, &id, None) {
                println!("{}", serde_json::to_string_pretty(&nft).unwrap());
            } else {
                println!("NFT not found");
            }
        }

        Commands::IbcExport { id } => {
            if let Some(nft) = state.get_nft(&id) {
                println!("{}", export_nft_for_ibc(nft));
            } else {
                println!("NFT not found");
            }
        }

        Commands::IbcImport { serialized } => {
            let nft = import_nft_from_ibc(&serialized);
            println!("Imported NFT ID: {}", nft.id);
        }
    }
}
