# Contador

El ejemplo de principio a fin de la clase: **cada usuario tiene su propio contador, y solo él puede sumarle.** Es un contrato chiquito que junta casi todo lo que vimos.

| Función | Qué hace | ¿Pide firma? |
|---|---|---|
| `__constructor(admin)` | Guarda al admin. Corre una sola vez, al desplegar | — |
| `increment(user, by)` | Le suma `by` al contador de `user` y devuelve el total | Sí, la de `user` |
| `count(user)` | Devuelve el total de `user`, o 0 si nunca sumó | No |

## Qué vas a ver en el código

- **[`src/lib.rs`](src/lib.rs)**:
  - **`DataKey`**: una clave única (`Admin`) y una clave por usuario (`Count(Address)`).
  - **`require_auth()`**: `increment` exige la firma de `user`, porque es su contador.
  - **Storage**: `instance` para el admin y `persistent` para cada contador.
  - **`extend_ttl`**: si a la entrada le queda menos de un día de vida, se renueva a unos 30 días.
- **[`Cargo.toml`](Cargo.toml)**: `overflow-checks = true`. Si una suma se desborda, el contrato falla en vez de dar un número equivocado.
- **[`src/test.rs`](src/test.rs)**: un test por cada cosa que el contrato promete.

El admin se guarda, pero ninguna función lo usa todavía. Es el punto de partida si querés agregar una función que solo él pueda llamar.

## Probarlo

Desde esta carpeta:

```bash
cargo test
```

## Desplegarlo en testnet

Necesitás el entorno de [`setup`](../../setup/README.md). Desde esta carpeta:

```bash
# 1. una identidad con XLM de prueba
stellar keys generate alice --network testnet --fund

# 2. compilar a Wasm
stellar contract build

# 3. desplegar: lo que va después de -- es para el constructor
stellar contract deploy --wasm target/wasm32v1-none/release/contador.wasm \
  --source alice --network testnet -- --admin alice
```

El último comando devuelve la dirección `C…` del contrato. Usala en lugar de `C…`:

```bash
# 4. sumar 5 al contador de alice: devuelve 5
stellar contract invoke --id C… --source alice --network testnet \
  -- increment --user alice --by 5

# 5. leer el total: devuelve 5
stellar contract invoke --id C… --source alice --network testnet \
  -- count --user alice
```

Si Friendbot falla por saturación en el paso 1, reintentá más tarde con `stellar keys fund alice --network testnet`.

Para ver el contrato, buscá su dirección `C…` en [stellar.expert](https://stellar.expert/explorer/testnet).
