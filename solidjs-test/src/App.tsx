import type { Component } from 'solid-js';
import { createSignal } from "solid-js";

import logo from './logo.svg';
import styles from './App.module.css';

import MyComponent from "./MyComponent.tsx";
import AutoCounter from "./AutoCounter.tsx";

const App: Component = () => {
    const [name, setName] = createSignal(new Date().toString());

    setInterval(() => {
        setName(new Date().toString());
        console.log("interval");
    }, 1000);
  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          class={styles.link}
          href="https://github.com/solidjs/solid"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn SolidJS
        </a>
        <AutoCounter />
      </header>
    </div>
  );
};

export default App;
