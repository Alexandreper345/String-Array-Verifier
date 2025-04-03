# 🦀 Palindrome & Array Utilities in Rust

Este projeto contém funções escritas em **Rust** para verificar se uma string é um palíndromo e para operar em arrays de números inteiros.

---

## 📌 Índice
- [Objetivo do Projeto](#-objetivo-do-projeto)
- [Tecnologias Utilizadas](#-tecnologias-utilizadas)
- [Funcionalidades](#-funcionalidades)
- [Instalação e Configuração](#-instalacao-e-configuracao)
- [Executando os Testes](#-executando-os-testes)
- [Contato](#-contato)

---

## 🎯 Objetivo do Projeto

O objetivo deste projeto é implementar e testar funções que:
- Verificam se uma string é um palíndromo.
- Verificam se um array contém um determinado valor.
- Identificam pares de um valor dentro de um array.

---

## 🛠 Tecnologias Utilizadas

- **Linguagem:** Rust
- **Gerenciador de Dependências:** Cargo

---

## ✨ Funcionalidades

- `is_a_palindrome(value: &str) -> bool` → Verifica se uma string é um palíndromo.
- `contains_target_at(value: [i8; 5], target: i8) -> bool` → Verifica se um número específico está presente no array.
- `contains_pars_at(value: [i8; 5], target: i8) -> bool` → Verifica se há pares do número-alvo no array.

---

## 🛠 Instalação e Configuração

1. Instale o **Rust** e o **Cargo** se ainda não tiver:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone o repositório:
   ```bash
   git clone https://github.com/seu-usuario/seu-repositorio.git
   cd seu-repositorio
   ```
3. Compile o código:
   ```bash
   cargo build
   ```

---

## ▶️ Executando os Testes

Para rodar os testes automatizados, use:
```bash
cargo test
```
Isso executará todos os testes dentro do módulo `tests`.

---

## 📩 Contato

Se tiver dúvidas ou sugestões, entre em contato:
📧 **E-mail:** [asilvaperoba@gmail.com](mailto:asilvaperoba@gmail.com)

