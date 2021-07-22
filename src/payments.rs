use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Honestly I know very little about how payment flows actually work, so this is the weakest part
// of the assignment for me. After seeing Fisher go through this, I could probably research and pick
// this up if given a few days.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentService {
    // Different payment systems use different tokens to track transactions
    PayPal(Uuid),
    Stripe(String),
    Bitcoin(usize),
    Magic,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentError {
    PaymentDNE,
    PaypalIsDown,
    TheWizardIsBeingLazy,
    DaveSpilledCoffeeOnAllTheAWSServers,
    Other(String),
}

/*
 * Assumptions:
 * 1. The frontend has already done the transaction flow, I'm just checking if its gone through.
 * 2. The frontend is smart enough to retry if this returns false.
 * 3. The functions that I call here are smart.
 * Questions:
 * 1. Who is responcible if a transaction fails?
 * If I had more time:
 * 1. I would make this more asyncronus. Some payment stuff takes a while
 *       I wouldn't want to block threads waiting for paypal
 * 2. Go find someone to review this code.
 */

/*
 * The payment flow that I thought of is:
 * 1. The Frontend tells the backend "The payment is done, go check in with the service provider"
 * 2. The Backend starts a an asyncronios check with the service provider, and
        replies with "OK, call me back in a minute after I can chat with the service provider"
 * 3. The Frontend calls back with the ticket info
 * 4. Backend Checks if the async call ever finished and gives a 👍 or 👎
*/

// This function would start up an async verification task and hand it off
// to a tokio executor or something
pub fn has_payment_been_processed(payment: &PaymentService) -> Result<(), PaymentError> {
    match payment {
        PaymentService::PayPal(id) => Err(PaymentError::PaypalIsDown), // did_paypal_have_the_transaction()
        PaymentService::Bitcoin(block_number) => {
            // check_the_chain(block)
            Err(PaymentError::Other("BTC Crashed Again".to_string()))
        }
        PaymentService::Magic => match rand::random::<bool>() {
            true => Ok(()),
            false => Err(PaymentError::TheWizardIsBeingLazy),
        },
        _ => Err(PaymentError::PaymentDNE),
    }
}

pub fn start_payment_processing(payment: PaymentService) {
    match payment {
        PaymentService::PayPal(id) => (), // check_in_with_paypal(id)
        PaymentService::Bitcoin(block_number) => (), // update_block_chain()
        PaymentService::Magic => (),      // Find a wizard
        _ => (),
    };
}
