/// Representa una cuenta bancaria con un saldo entero.
pub struct Account(i32);

impl Account {
    /// Crea una cuenta con el saldo inicial indicado.
    pub fn new(saldo_inicial: i32) -> Self {
        Self(saldo_inicial)
    }

    /// Deposita un monto en la cuenta.
    pub fn deposit(&mut self, amount: i32) {
        println!("op: deposit {}, available funds: {:?}", amount, self.0);
        self.0 += amount;
    }

    /// Retira un monto de la cuenta si hay fondos suficientes.
    pub fn withdraw(&mut self, amount: i32) {
        println!("op: withdraw {}, available funds: {}", amount, self.0);
        if self.0 >= amount {
            self.0 -= amount;
        } else {
            println!("Error: Insufficient funds.");
        }
    }

    /// Devuelve el saldo actual de la cuenta.
    pub fn balance(&self) -> i32 {
        self.0
    }
}
