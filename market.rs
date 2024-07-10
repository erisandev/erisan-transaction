// market.rs
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the Market struct
struct Market {
    order_book: Vec<Order>,
    token_vault: Pubkey,
}

// Define the Order struct
struct Order {
    id: u64,
    side: Side,
    price: u64,
    amount: u64,
}

// Define the Side enum
enum Side {
    Buy,
    Sell,
}

// Implement the Market program
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len()!= 32 {
        msg!("Invalid instruction");
        return Err(ProgramError::InvalidInstructionData);
    }

    let market_account = next_account_info(accounts)?;
    let token_vault_account = next_account_info(accounts)?;
    let user_account = next_account_info(accounts)?;

    // Unpack the instruction data
    let side = Side::from(instruction_data[0]);
    let price = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
    let amount = u64::from_le_bytes(instruction_data[9..17].try_into().unwrap());

    // Process the order
    match side {
        Side::Buy => {
            // Check if the user has enough tokens
            if user_account.lamports() < amount {
                msg!("Insufficient tokens");
                return Err(ProgramError::Custom(1));
            }

            // Find a matching sell order
            let mut matching_order = None;
            for order in &market_account.order_book {
                if order.side == Side::Sell && order.price <= price {
                    matching_order = Some(order);
                    break;
                }
            }

            if let Some(matching_order) = matching_order {
                // Execute the trade
                
                token_vault_account.transfer tokens(amount, user_account, market_account)?;
                market_account.order_book.retain(|order| order.id!= matching_order.id);
                msg!("Trade executed");
            } else {
                msg!("No matching order found");
                return Err(ProgramError::Custom(2));
            }
        }
        Side::Sell => {
            // Add the order to the order book
            market_account.order_book.push(Order {
                id: market_account.order_book.len() as u64,
                side,
                price,
                amount,
            });
            msg!("Order added");
        }
    }

    Ok(())
}
