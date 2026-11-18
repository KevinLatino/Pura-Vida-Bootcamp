// ═══════════════════════════════════════════════════════════════════════════
// CONTADOR
// ¿Qué es? El ejemplo de principio a fin de la clase: cada usuario tiene su
// propio contador, y solo él puede sumarle. Junta el constructor,
// require_auth, el patrón DataKey, el storage y el TTL.
// ═══════════════════════════════════════════════════════════════════════════
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey { Admin, Count(Address) }

#[contract]
pub struct Contador;

#[contractimpl]
impl Contador {
    pub fn __constructor(env: Env, admin: Address) {   // corre al desplegar
        env.storage().instance().set(&DataKey::Admin, &admin);
    }
    pub fn increment(env: Env, user: Address, by: u32) -> u32 {
        user.require_auth();                            // ¿firmó user?
        let key = DataKey::Count(user.clone());         // una clave por usuario
        let s = env.storage().persistent();
        let total = s.get(&key).unwrap_or(0u32) + by;   // 0 la primera vez
        s.set(&key, &total);
        s.extend_ttl(&key, 17_280, 518_400);            // si quedan menos de 1 día, a 30
        total
    }
    pub fn count(env: Env, user: Address) -> u32 {      // solo lee: no pide firma
        env.storage().persistent().get(&DataKey::Count(user)).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
