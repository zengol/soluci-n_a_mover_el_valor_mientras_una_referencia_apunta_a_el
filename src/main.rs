#[derive(Debug)]
struct Account {
    id: u32,
    name: String,
}

impl Account {
    fn new(id: u32, name: String) -> Self {
        Account { id, name }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: Vec::new() }
    }
}

fn print_account(accounts: &[&Account]) {
    println!("{:#?}", accounts);
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("me"));

    // Crear una referencia a account sin mover la propiedad
    let account_ref = &[&account];
   
    // Imprimir usando account_ref
    print_account(account_ref);

    // Ahora podemos mover account al vector de bank
    bank.accounts.push(account);
   
    println!("{:#?}", bank);
}
