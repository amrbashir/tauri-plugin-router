import { invoke } from "tauri-plugin-router";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let asyncGreetInputEl: HTMLInputElement | null;
let asyncGreetMsgEl: HTMLElement | null;
let asyncGreetBtnEl: HTMLElement | null;

async function greet() {
  if (!greetMsgEl || !greetInputEl) return;
  const greeting = await invoke("greet", greetInputEl.value);
  greetMsgEl.textContent = greeting as string;
}

async function asyncGreet() {
  if (!asyncGreetMsgEl || !asyncGreetInputEl) return;
  asyncGreetBtnEl?.classList.add("loading");
  const greeting = await invoke("async_greet", asyncGreetInputEl.value);
  asyncGreetBtnEl?.classList.remove("loading");
  asyncGreetMsgEl.textContent = greeting as string;
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  asyncGreetInputEl = document.querySelector("#async-greet-input");
  asyncGreetMsgEl = document.querySelector("#async-greet-msg");
  asyncGreetBtnEl = document.querySelector("#async-greet-btn");

  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  document
    .querySelector("#async-greet-form")
    ?.addEventListener("submit", (e) => {
      e.preventDefault();
      asyncGreet();
    });
});
