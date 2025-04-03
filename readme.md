# TOTP Sample Library

Esta biblioteca implementa uma versão simplificada e de exemplo, do algoritmo TOTP, a partir de estudos sobre a [RFC 6238](https://datatracker.ietf.org/doc/html/rfc6238). O _build_ da biblioteca gera um _target_ em WebAssembly, para uso e experimento na [aplicação de demonstração](https://github.com/herberthleao/totp-webapp).

## Instruções

Se ainda não tiver o `wasm-pack` instalado, execute:

```sh
cargo install wasm-pack
```

O seguinte comando irá gerar o _target_ wasm:

```sh
wasm-pack build --target web
```

Após isto, basta importar o _target_ dentro do projeto web e utilizar a função `generate_code()`.

## Licença

Este projeto é de código aberto e está licenciado sob a [Licença MIT](license.md).
