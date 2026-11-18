// Se corren con:  cargo test
use super::*;
use soroban_sdk::testutils::storage::Persistent as _;
use soroban_sdk::testutils::Address as _;

// El camino feliz, paso por paso: cada llamada suma y devuelve el total.
#[test]
fn incrementa() {
    let env = Env::default();                           // 1. blockchain de juguete
    let admin = Address::generate(&env);                // 2. usuarios de prueba
    let ana = Address::generate(&env);
    let id = env.register(Contador, (&admin,));         // 3. desplegar, con el admin para el constructor
    let client = ContadorClient::new(&env, &id);        // 4. cliente generado por #[contractimpl]
    env.mock_all_auths();                               // 5. simular que todos firman

    assert_eq!(client.increment(&ana, &5), 5);          // 6. llamar y verificar
    assert_eq!(client.increment(&ana, &2), 7);
}

// Despliega el contador y devuelve un usuario de prueba.
fn setup() -> (Env, ContadorClient<'static>, Address) {
    let env = Env::default();
    let admin = Address::generate(&env);
    let id = env.register(Contador, (&admin,));
    let client = ContadorClient::new(&env, &id);
    let ana = Address::generate(&env);
    (env, client, ana)
}

// count lee lo que quedó guardado.
#[test]
fn lee_lo_guardado() {
    let (env, client, ana) = setup();
    env.mock_all_auths();
    assert_eq!(client.count(&ana), 0);
    client.increment(&ana, &5);
    client.increment(&ana, &2);
    assert_eq!(client.count(&ana), 7);
}

// Sin mock_all_auths nadie firmó: require_auth hace fallar la llamada.
#[test]
#[should_panic]
fn sin_firma() {
    let (_env, client, ana) = setup();
    client.increment(&ana, &1);
}

// env.auths() dice qué firmas pidió la última llamada: la de ana.
#[test]
fn quien_firmo() {
    let (env, client, ana) = setup();
    env.mock_all_auths();
    client.increment(&ana, &1);
    assert_eq!(env.auths()[0].0, ana);
}

// increment deja la entrada de ana con 518 400 ledgers de vida, unos 30 días.
#[test]
fn renueva_el_ttl() {
    let (env, client, ana) = setup();
    env.mock_all_auths();
    client.increment(&ana, &1);
    let ttl = env.as_contract(&client.address, || {
        env.storage().persistent().get_ttl(&DataKey::Count(ana.clone()))
    });
    assert_eq!(ttl, 518_400);
}

// Una suma que no cabe en u32 falla, en vez de dar la vuelta a un número chico.
// En los tests Rust siempre revisa los desbordes; overflow-checks = true en
// Cargo.toml hace lo mismo en el Wasm que desplegás.
#[test]
#[should_panic]
fn desborde_falla() {
    let (env, client, ana) = setup();
    env.mock_all_auths();
    client.increment(&ana, &u32::MAX);
    client.increment(&ana, &1);
}

// Cada usuario tiene su propia clave en el storage.
#[test]
fn cada_usuario_tiene_su_contador() {
    let (env, client, ana) = setup();
    env.mock_all_auths();
    let luis = Address::generate(&env);
    client.increment(&ana, &5);
    assert_eq!(client.increment(&luis, &1), 1);
    assert_eq!(client.count(&ana), 5);
}

// count solo lee, así que no pide firma.
#[test]
fn count_no_pide_firma() {
    let (_env, client, ana) = setup();
    assert_eq!(client.count(&ana), 0);
}
