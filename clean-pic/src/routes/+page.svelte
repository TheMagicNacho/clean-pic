<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {open} from "@tauri-apps/plugin-dialog";
    import {openPath} from "@tauri-apps/plugin-opener";
    import {fly} from "svelte/transition";
    import KawaiBear from "$lib/components/KawaiBear.svelte";

    let folderPath = $state("");
    let numberOfFiles = $state(-1);
    let thinking = $state(false);
    let cleanPath = $state("");
    let saveDirectory = $state("OwO-Clean");
    let isDrawerOpen = $state(false);

    let activeView: "scrub" | "inspect" = $state("scrub");

    // Inspect view state
    let imagePath = $state("");
    let imageMetadata = $state<Record<string, string> | null>(null);
    let inspecting = $state(false);

    // Bear state management
    type BearState = "waiting" | "talking" | "looking";
    let bearState: BearState = $state("waiting");
    let bearMessage = $state("");

    function setBearState(state: BearState, message: string = "", duration: number = 3000) {
        bearState = state;
        bearMessage = message;
        if (state !== "waiting" && duration > 0) {
            setTimeout(() => {
                bearState = "waiting";
                bearMessage = "";
            }, duration);
        }
    }

    // Button hover descriptions
    const buttonDescriptions: Record<string, string> = {
        "select-folder": "I'll scrub all the images in the directory you choose, then save the cleaned images in a folder for you! 📁",
        "scrub": "Let's clean those images! I'll remove all metadata and make them squeaky clean! 🧹✨",
        "reset": "This will clear everything and start fresh! 🔄",
        "select-image": "Pick an image and I'll show you all the hidden metadata inside! 🔍",
        "scrub-view": "Switch to scrub mode to clean metadata from your images! 🧼",
        "inspect-view": "Switch to inspect mode to peek at image metadata! 👀",
    };

    function onButtonHover(buttonId: string) {
        const description = buttonDescriptions[buttonId];
        if (description) {
            bearState = "talking";
            bearMessage = description;
        }
    }

    function onButtonLeave() {
        bearState = "waiting";
        bearMessage = "";
    }

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
        imagePath = "";
        imageMetadata = null;
    }

    async function scrubDirectory(event: Event) {
        event.preventDefault();
        thinking = true;
        setBearState("talking", "Scrubbing your images! ✨", 0);
        console.log("Scrubbing folder:", folderPath);
        const result = String(
            await invoke("scrub_images", {
                path: folderPath,
                saveDirectory: saveDirectory,
            }),
        );
        console.log("Scrub result:", result);
        thinking = false;
        setBearState("talking", "All done! Squeaky clean! 🧼", 4000);
        await openPath(result);
        cleanPath = result;
    }

    async function selectFolder(event: Event) {
        event.preventDefault();
        thinking = true;
        setBearState("looking", "Looking for images... 🔍", 0);
        const selected = await open({
            directory: true,
            multiple: false,
            title: "Select a folder",
        });
        if (typeof selected === "string") {
            folderPath = selected;
            console.log("Selected folder:", folderPath);
            numberOfFiles = await invoke("count_images", {path: folderPath});
            if (numberOfFiles > 0) {
                setBearState("talking", `Found ${numberOfFiles} images! 📸`, 3000);
            } else {
                setBearState("talking", "No images found here 😅", 3000);
            }
        } else {
            console.log("No folder selected");
            setBearState("waiting", "");
        }
        thinking = false;
    }

    function createImageMetadataObject(rawData: [string, string][]) {
        let output: Record<string, string> = {};

        for (let dataPair of rawData) {
            console.log(`dataPair: ${dataPair}`);
            const [key, value] = dataPair;
            output[key] = value;
        }
        return output;
    }

    async function selectImage(event: Event) {
        event.preventDefault();
        inspecting = true;
        imageMetadata = null;
        setBearState("looking", "Inspecting image... 🧐", 0);
        const selected = await open({
            directory: false,
            multiple: false,
            title: "Select an image to inspect",
        });
        if (typeof selected === "string") {
            imagePath = selected;
            console.log("Selected image:", imagePath);
            // TODO: invoke backend to get image metadata
            const metadata = await invoke("inspect_image", {path: imagePath});
            console.log(`metadata: ${metadata}`);
            imageMetadata = createImageMetadataObject(metadata as [string, string][]);
            setBearState("talking", "Here's what I found! 📋", 3000);
        } else {
            console.log("No image selected");
            setBearState("waiting", "");
        }
        inspecting = false;
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
            <hr/>
            <label for="reset-state">Reset Application State</label>
            <form id="reset-state" onsubmit={resetState}>
                <button type="submit">Reset</button>
            </form>
        </div>
    </div>
{/if}

<main class="container">
    <h1>0w0 scrubby buddy</h1>

    <div class="view-switcher">
        <button
                class:active={activeView === "scrub"}
                onclick={() => (activeView = "scrub")}
                onmouseenter={() => onButtonHover("scrub-view")}
                onmouseleave={onButtonLeave}>Scrub
        </button
        >
        <button
                class:active={activeView === "inspect"}
                onclick={() => (activeView = "inspect")}
                onmouseenter={() => onButtonHover("inspect-view")}
                onmouseleave={onButtonLeave}>Inspect
        </button
        >
    </div>

    <!--    SCRUBBING SECTION -->
    {#if activeView === "scrub"}
        <div transition:fly={{ x: -200, duration: 300 }}>
            <form class="row" onsubmit={selectFolder}>
                <button
                        type="submit"
                        onmouseenter={() => onButtonHover("select-folder")}
                        onmouseleave={onButtonLeave}>Select Folder
                </button>
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
                        <button
                                type="submit"
                                onmouseenter={() => onButtonHover("scrub")}
                                onmouseleave={onButtonLeave}>Scrub Me Daddy
                        </button>
                    </form>
                {/if}
            {/if}
            {#if cleanPath}
                <p>I'm all squeaky clean: {cleanPath}</p>
                <form class="row" onsubmit={resetState}>
                    <button
                            type="submit"
                            onmouseenter={() => onButtonHover("reset")}
                            onmouseleave={onButtonLeave}>Reset
                    </button>
                </form>
            {/if}
        </div>
    {/if}

    <!--INSPECTION SECTION-->
    {#if activeView === "inspect"}
        <div transition:fly={{ x: 200, duration: 300 }}>
            <form class="row" onsubmit={selectImage}>
                <button
                        type="submit"
                        onmouseenter={() => onButtonHover("select-image")}
                        onmouseleave={onButtonLeave}>Select Image
                </button>
            </form>

            {#if inspecting}
                <p>Inspecting...</p>
            {/if}

            {#if imagePath && imageMetadata}
                <p>Showing metadata for: {imagePath}</p>
                <table class="metadata-table">
                    <thead>
                    <tr>
                        <th>Key</th>
                        <th>Value</th>
                    </tr>
                    </thead>
                    <tbody>
                    {#each Object.entries(imageMetadata) as [key, value]}
                        <tr>
                            <td>{key}</td>
                            <td>{value}</td>
                        </tr>
                    {/each}
                    </tbody>
                </table>
            {/if}
        </div>
    {/if}
    {#if bearMessage && bearState === "talking"}
        <div class="speech-bubble" transition:fly={{ y: 10, duration: 200 }}>
            <p>{bearMessage}</p>
            <div class="bubble-tail"></div>
        </div>
    {/if}

    <KawaiBear message={bearMessage} state={bearState}/>
</main>

<style>
    /** Speech bubble styles **/
    .speech-bubble {
        background: #fff;
        border-radius: 12px;
        padding: 10px 14px;
        max-width: 180px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
        position: absolute;
        bottom: 150px;
        order: 2;
    }

    .speech-bubble p {
        margin: 0;
        font-size: 12px;
        color: #333;
        line-height: 1.4;
    }

    .bubble-tail {
        position: absolute;
        bottom: -10px;
        left: 20px;
        width: 0;
        height: 0;
        border-left: 8px solid transparent;
        border-right: 8px solid transparent;
        border-top: 10px solid #fff;
    }

    /*.logo.vite:hover {*/
    /*    filter: drop-shadow(0 0 2em #747bff);*/
    /*}*/

    /*.logo.svelte-kit:hover {*/
    /*    filter: drop-shadow(0 0 2em #ff3e00);*/
    /*}*/

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
        /*padding-top: 10vh;*/
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

    .view-switcher {
        display: flex;
        justify-content: center;
        margin-bottom: 2rem;
        gap: 0.5rem;
    }

    .view-switcher button {
        background-color: transparent;
        border: 1px solid #ccc;
    }

    .view-switcher button.active {
        border-color: #396cd8;
        background-color: #e8e8e8;
    }

    .metadata-table {
        margin: 2rem auto;
        border-collapse: collapse;
        width: 80%;
        max-width: 600px;
    }

    .metadata-table th,
    .metadata-table td {
        border: 1px solid #ddd;
        padding: 8px;
        text-align: left;
    }

    .metadata-table th {
        background-color: #f2f2f2;
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

        .view-switcher button {
            border-color: #555;
        }

        .view-switcher button.active {
            border-color: #24c8db;
            background-color: #0f0f0f69;
        }

        .metadata-table th,
        .metadata-table td {
            border: 1px solid #444;
        }

        .metadata-table th {
            background-color: #3a3a3a;
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
