// Define the PaymentMethod trait
trait PaymentMethod {
    fn authorize_payment(&self, amount: f64) -> bool;
    fn refund_payment(&self, amount: f64) -> bool;
}

// Implement the CreditCard struct
struct CreditCard {
    card_number: String,
    card_holder: String,
}

impl PaymentMethod for CreditCard {
    fn authorize_payment(&self, amount: f64) -> bool {
        // Simulate payment authorization logic
        println!("Authorizing credit card payment of ${:.2} for {} (Card no. {})", amount, self.card_holder, self.card_number);
        true // Assume authorization is successful
    }

    fn refund_payment(&self, amount: f64) -> bool {
        // Simulate refund logic
        println!("Refunding ${:.2} to {}", amount, self.card_holder);
        true // Assume refund is successful
    }
}

// Implement the PayPal struct
struct PayPal {
    email: String,
}

impl PaymentMethod for PayPal {
    fn authorize_payment(&self, amount: f64) -> bool {
        // Simulate PayPal payment authorization logic
        println!("Redirecting to PayPal for payment of ${:.2} from {}", amount, self.email);
        true // Assume authorization is successful
    }

    fn refund_payment(&self, amount: f64) -> bool {
        // Simulate refund logic
        println!("Refunding ${:.2} to PayPal account {}", amount, self.email);
        true // Assume refund is successful
    }
}

// Implement the BankTransfer struct
struct BankTransfer {
    account_number: String,
}

impl PaymentMethod for BankTransfer {
    fn authorize_payment(&self, amount: f64) -> bool {
        // Simulate bank transfer authorization logic
        println!("Initiating bank transfer of ${:.2} from account {}", amount, self.account_number);
        true // Assume authorization is successful
    }

    fn refund_payment(&self, amount: f64) -> bool {
        // Simulate refund logic
        println!("Refunding ${:.2} to bank account {}", amount, self.account_number);
        true // Assume refund is successful
    }
}

// Main function to demonstrate the usage
fn main() {
    let credit_card = CreditCard {
        card_number: String::from("1234-5678-9012-3456"),
        card_holder: String::from("John Doe"),
    };

    let paypal = PayPal {
        email: String::from("john.doe@example.com"),
    };

    let bank_transfer = BankTransfer {
        account_number: String::from("987654321"),
    };

    // Authorize payments
    credit_card.authorize_payment(100.0);
    paypal.authorize_payment(50.0);
    bank_transfer.authorize_payment(200.0);

    // Refund payments
    credit_card.refund_payment(30.0);
    paypal.refund_payment(20.0);
    bank_transfer.refund_payment(100.0);
}