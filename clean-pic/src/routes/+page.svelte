<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {open} from "@tauri-apps/plugin-dialog";


    // let name = $state("");
    // let greetMsg = $state("");
    // async function greet(event: Event) {
    //   event.preventDefault();
    //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //   greetMsg = await invoke("greet", { name });
    // }

    let folderPath = $state("");
    let numberOfFiles = $state(-1);

    async function scrubDirectory(event: Event) {
        event.preventDefault();
        console.log("Scrubbing folder:", folderPath);
        const result = await invoke("scrub_images", {path: folderPath});
        console.log("Scrub result:", result);
    }


    async function selectFolder(event: Event) {
        event.preventDefault();
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
    }
</script>

<main class="container">
    <h1>0w0 scrubby buddy</h1>

    <form class="row" onsubmit={selectFolder}>
        <button type="submit">Select Folder</button>
    </form>


    {#if folderPath}
        {#if numberOfFiles === -1 }
            <p>Counting images...</p>
        {/if}
        {#if numberOfFiles === 0 }
            <p>No images found in the selected folder.</p>
        {/if}

        {#if numberOfFiles > 0}
            <p>Number of images found: {numberOfFiles}</p>
            <form class="row" onsubmit={scrubDirectory}>
                <button type="submit">Scrub Me Daddy</button>
            </form>
        {/if}
    {/if}


</main>

<style>
    .logo.vite:hover {
        filter: drop-shadow(0 0 2em #747bff);
    }

    .logo.svelte-kit:hover {
        filter: drop-shadow(0 0 2em #ff3e00);
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
