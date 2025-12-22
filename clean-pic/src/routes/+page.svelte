<script lang="ts">
  import {invoke} from "@tauri-apps/api/core";
  import {open} from "@tauri-apps/plugin-dialog";
  import {openPath} from "@tauri-apps/plugin-opener";
  import {fly} from "svelte/transition";

  let folderPath = $state("");
    let numberOfFiles = $state(-1);
    let thinking = $state(false);
    let cleanPath = $state("");
    let saveDirectory = $state("OwO-Clean");
    let isDrawerOpen = $state(false);

    // function toggleDrawer() {
    //     isDrawerOpen = !isDrawerOpen;
    // }

    function openDrawer() {
        isDrawerOpen = true;
    }

    function closeDrawer() {
        isDrawerOpen = false;
    }

    function resetState(event: Event) {
        event.preventDefault();
        folderPath = "";
        numberOfFiles = -1;
        cleanPath = "";
    }

    async function scrubDirectory(event: Event) {
        event.preventDefault();
        thinking = true;
        console.log("Scrubbing folder:", folderPath);
        const result = String(
            await invoke("scrub_images", {
                path: folderPath,
                saveDirectory: saveDirectory,
            }),
        );
        console.log("Scrub result:", result);
        thinking = false;
        await openPath(result);
        cleanPath = result;
    }

    async function selectFolder(event: Event) {
        event.preventDefault();
        thinking = true;
        const selected = await open({
            directory: true,
            multiple: false,
            title: "Select a folder",
        });
        if (typeof selected === "string") {
            folderPath = selected;
            console.log("Selected folder:", folderPath);
            numberOfFiles = await invoke("count_images", {path: folderPath});
        } else {
            console.log("No folder selected");
        }
        thinking = false;
    }
</script>

{#if !isDrawerOpen}
    <div
            class="hamburger-menu"
            onclick={openDrawer}
            onkeydown={() => {}}
            role="button"
            tabindex="0"
    >
        <div></div>
        <div></div>
        <div></div>
    </div>
{/if}

{#if isDrawerOpen}
    <div
            class="drawer-container"
            role="dialog"
            tabindex="0"
            onclick={closeDrawer}
            onkeydown={() => {}}
    >
        <div
                class="drawer"
                role="dialog"
                tabindex="0"
                onkeydown={() => {}}
                onclick={(event) => event.stopPropagation()}
                transition:fly={{ x: 250, duration: 300 }}
        >
            <label for="save-dir">Save Directory Name</label>
            <input id="save-dir" type="text" bind:value={saveDirectory}/>
            <hr>
            <label for="reset-state">Reset Application State</label>
            <form id="reset-state" onsubmit={resetState}>
                <button type="submit">Reset</button>
            </form>
        </div>
    </div>
{/if}

<main class="container">
    <h1>0w0 scrubby buddy</h1>

    <form class="row" onsubmit={selectFolder}>
        <button type="submit">Select Folder</button>
    </form>

    {#if folderPath}
        {#if numberOfFiles === -1}
            <p>Counting images...</p>
        {/if}
        {#if numberOfFiles === 0}
            <p>No images found in the selected folder.</p>
        {/if}

        {#if numberOfFiles > 0}
            <p>Number of images found: {numberOfFiles}</p>
            <form class="row" onsubmit={scrubDirectory}>
                <button type="submit">Scrub Me Daddy</button>
            </form>
        {/if}
    {/if}
    {#if cleanPath}
        <p>I'm all squeaky clean: {cleanPath}</p>
        <form class="row" onsubmit={resetState}>
            <button type="submit">Reset</button>
        </form>
    {/if}
</main>

<style>
    .logo.vite:hover {
        filter: drop-shadow(0 0 2em #747bff);
    }

    .logo.svelte-kit:hover {
        filter: drop-shadow(0 0 2em #ff3e00);
    }

    .hamburger-menu {
        position: absolute;
        top: 1.5rem;
        right: 1.5rem;
        display: flex;
        flex-direction: column;
        justify-content: space-around;
        width: 2rem;
        height: 2rem;
        background: transparent;
        border: none;
        cursor: pointer;
        padding: 0;
        z-index: 1001;
    }

    .hamburger-menu div {
        width: 2rem;
        height: 0.25rem;
        background: #0f0f0f;
        border-radius: 10px;
        transition: all 0.3s linear;
        position: relative;
        transform-origin: 1px;
    }

    .drawer-container {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        z-index: 1000;
    }

    .drawer {
        position: fixed;
        top: 0;
        right: 0;
        height: 100%;
        width: 250px;
        background: #f6f6f6;
        box-shadow: -2px 0 5px rgba(0, 0, 0, 0.5);
        padding: 20px;
        z-index: 1001;
        display: flex;
        /*justify-content: center;*/
        align-items: center;
        flex-direction: column;
        gap: 1rem;
    }

    @media (prefers-color-scheme: dark) {
        .hamburger-menu div {
            background: #f6f6f6;
        }

        .drawer {
            background: #2f2f2f;
        }
    }

    :root {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: #0f0f0f;
        background-color: #f6f6f6;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    .container {
        margin: 0;
        padding-top: 10vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }

    .logo {
        height: 6em;
        padding: 1.5em;
        will-change: filter;
        transition: 0.75s;
    }

    .logo.tauri:hover {
        filter: drop-shadow(0 0 2em #24c8db);
    }

    .row {
        display: flex;
        justify-content: center;
    }

    a {
        font-weight: 500;
        color: #646cff;
        text-decoration: inherit;
    }

    a:hover {
        color: #535bf2;
    }

    h1 {
        text-align: center;
    }

    input,
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #0f0f0f;
        background-color: #ffffff;
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }

    button {
        cursor: pointer;
    }

    button:hover {
        border-color: #396cd8;
    }

    button:active {
        border-color: #396cd8;
        background-color: #e8e8e8;
    }

    input,
    button {
        outline: none;
    }

    #greet-input {
        margin-right: 5px;
    }

    @media (prefers-color-scheme: dark) {
        :root {
            color: #f6f6f6;
            background-color: #2f2f2f;
        }

        a:hover {
            color: #24c8db;
        }

        input,
        button {
            color: #ffffff;
            background-color: #0f0f0f98;
        }

        button:active {
            background-color: #0f0f0f69;
        }
    }
</style>
