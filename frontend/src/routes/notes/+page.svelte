<script lang="ts">
    import {getAuth, type User} from "firebase/auth";
    import {getApp} from "firebase/app";
    import {collection, getFirestore, query, where, getDocs, updateDoc, deleteDoc, addDoc, QueryDocumentSnapshot} from "firebase/firestore";
    import {AuthenticatedUser} from "../../stores/user";
    import {goto} from "$app/navigation";
    import {onDestroy, onMount} from "svelte";
    import PlayerFactory from "youtube-player";
    import {get, writable} from "svelte/store";
    import type {YouTubePlayer} from "youtube-player/dist/types";
    import {Marked} from 'marked';
    import DOMPurify from "dompurify";
    import hljs from 'highlight.js';
    import 'highlight.js/styles/github-dark.css';
    import {markedHighlight} from "marked-highlight";
    import markedKatex from "marked-katex-extension";
    import {AI_SERVER} from "../../env";

    const marked = new Marked(
        markedHighlight({
            langPrefix: 'hljs language-',
            highlight(code, lang) {
                const language = hljs.getLanguage(lang) ? lang : 'plaintext';
                return hljs.highlight(code, { language }).value;
            }
        }),
        markedKatex({
            throwOnError: false,
        }),
    );

    let unsubscribe_from_authenticateduser = () => {};
    let unsubscribe_from_current_notes = () => {};
    let notes_collection_ref;
    let notes_query;
    let notes: QueryDocumentSnapshot<any, any>[] = [];
    let current_note_document = writable();
    let current_note = writable();
    let current_note_data = writable<any[] | null>(null);
    let player: YouTubePlayer;
    let edit_mode = false;
    let has_changed = true;
    let display_save_dialog = false;
    let delete_note_index = -1;
    let saving_note = false;
    let create_note = false;
    let creating_note = false;
    let delete_current_notes = false;
    let deleting_current_notes = false;
    let explain_prompt = false;
    let explaining_prompt = false;
    let note_explanation_data = "";
    let explanation = "";

    onMount(async () => {
        unsubscribe_from_authenticateduser = AuthenticatedUser.subscribe(async (user_state) => {
            if (!user_state) {
                await goto('/', {replaceState: true});
                return;
            }
            await updateNotesList();
        });

        unsubscribe_from_current_notes = current_note.subscribe((note) => {
            edit_mode = false;
            if (note) {
                if (player) {
                    let player_container = document.querySelector('.yt-player-container');
                    player_container?.firstChild?.remove();

                    let player_element = document.createElement("div");
                    player_element.id = "yt-player";

                    player_container?.appendChild(player_element);
                }
                player = PlayerFactory("yt-player", {
                    width: "100%",
                    height: "700px",
                    playerVars: {
                        origin: window.origin,
                    }
                });

                player.loadVideoByUrl(note.video);
                player.stopVideo();
                $current_note_data = JSON.parse(note.data);
            }
        });
    });

    async function updateNotesList() {
        notes_collection_ref = collection(getFirestore(getApp()), "notes");
        notes_query = query(notes_collection_ref, where("owner", "==", $AuthenticatedUser?.uid));
        notes = (await getDocs(notes_query)).docs;
        if (notes.length > 0) {
            $current_note_document = notes[0];
            $current_note = $current_note_document.data();
        }
    }

    function secondsToTime(seconds: number) {
        return `${Math.floor(seconds / 60)}:${Math.floor(seconds % 60)}`
    }

    function toggleEditMode() {
        if (!edit_mode) edit_mode = true;
        else if (has_changed) display_save_dialog = true;
    }

    async function addNoteForCurrentTimestamp() {
        let current_note_data_copy = get(current_note_data);
        current_note_data_copy?.push({
            timestamp: await player.getCurrentTime(),
            markdown: ""
        });

        current_note_data.set(current_note_data_copy);
    }

    function deleteNote() {
        let current_note_data_copy = get(current_note_data);
        current_note_data_copy?.splice(delete_note_index, 1);

        current_note_data.set(current_note_data_copy);
        delete_note_index = -1;
    }

    function updateNote(index: number, markdown: string) {
        $current_note_data[index].markdown = markdown;
    }

    async function createNote() {
        const note_title = document.querySelector('#note-title')?.value;
        const note_yt_vid = document.querySelector('#note-yt-link')?.value;

        if (note_title === "" || note_yt_vid === "") {
            alert("Put valid data in note title and note video link textfield");
            return;
        }

        creating_note = true;
        try {
            await addDoc(collection(getFirestore(getApp()), "notes"), {
                title: note_title,
                video: note_yt_vid.replace("watch?v=", "embed/"),
                data: "[]",
                owner: $AuthenticatedUser?.uid,
            });
        } catch(e) {
            alert(`Failed to create note: ${e}`);
        }
        creating_note = false;

        await updateNotesList();
        create_note = false;
    }

    async function deleteCurrentNotes() {
        deleting_current_notes = true;
        try {
            await deleteDoc($current_note_document?.ref);
        } catch(e) {
            alert(`Failed to delete note: ${e}`);
        }

        deleting_current_notes = false;
        delete_current_notes = false;
        await updateNotesList();
    }

    async function explainNote() {
        const user_prompt = document.querySelector('#explanation-prompt')?.value;
        const final_prompt = `Following is a note written using markdown and katex(wrapped in [START_NOTE] and [END_NOTE]: \
        [START_NOTE] \n\
        ${note_explanation_data} \n\
        [END_NOTE] \n\
\n\
        Wrap katex expressions in single dollar signs. Do not wrap answer with [START_NOTE] and [END_NOTE].
        Give detailed, step by step and correct answer of the following prompt based on the note: \n\
        ${user_prompt} \n\
        `;

        explaining_prompt = true;
        let resp = {};
        try {
            const request_headers = new Headers();
            request_headers.set("Content-type", "application/json");

            const req = await fetch(AI_SERVER, {
                method: "POST",
                headers: request_headers,
                body: JSON.stringify({
                    prompt: final_prompt,
                })
            });
            resp = await req.json();
        } catch(e) {
            alert(`Failed to get explanation: ${e}`);
            explaining_prompt = false;
            explain_prompt = false;
            return;
        }

        explaining_prompt = false;
        explain_prompt = false;
        explanation = DOMPurify.sanitize(await marked.parse(resp.candidates[0].output));
    }

    async function saveChanges() {
        $current_note_data?.sort((l, r) => {
            if (l.timestamp > r.timestamp) return 1;
            else if (l.timestamp < r.timestamp) return -1;
            return 0;
        });
        $current_note.data = JSON.stringify($current_note_data);

        saving_note = true;
        try {
            await updateDoc($current_note_document.ref, "data", JSON.stringify($current_note_data));
        } catch(e) {
            alert(`Failed to save note: ${e}`);
            saving_note = false;
            display_save_dialog = false;
            return;
        }
        saving_note = false;

        edit_mode = false;
        display_save_dialog = false;
        await updateNotesList();
    }

    onDestroy(() => {
        unsubscribe_from_authenticateduser();
        unsubscribe_from_current_notes();
    });
</script>

<svelte:head>
    <title>Smart Totoro - Notes</title>
</svelte:head>

<div class="flex min-h-screen min-w-full">
    {#if display_save_dialog}
        <div class="flex items-center justify-center h-screen w-full absolute top-0 left-0" style="background: rgba(0, 0, 0, 0.3)">
            <div class="bg-stone-100 w-fit rounded-md">
                {#if saving_note}
                    <h5 class="p-5">Saving...</h5>
                {:else}
                    <h5 class="p-5">Are you sure want to save changes?</h5>
                    <div class="p-5 pt-0 flex justify-center gap-2">
                        <button class="p-2 border-[1px] border-stone-500" on:click={() => saveChanges()}>Yes</button>
                        <button class="p-2 border-[1px] border-stone-500" on:click={() => { display_save_dialog = false; edit_mode = false; $current_note_data = JSON.parse($current_note.data) }}>No</button>
                        <button class="p-2 border-[1px] border-stone-500" on:click={() => display_save_dialog = false}>Cancel</button>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
    {#if delete_note_index !== -1}
        <div class="flex items-center justify-center h-screen w-full absolute top-0 left-0" style="background: rgba(0, 0, 0, 0.3)">
            <div class="bg-stone-100 w-fit rounded-md">
                <h5 class="p-5">Are you sure want to delete this note?</h5>
                <div class="p-5 pt-0 flex justify-center gap-2">
                    <button class="p-2 border-[1px] border-stone-500" on:click={() => deleteNote()}>Yes</button>
                    <button class="p-2 border-[1px] border-stone-500" on:click={() => delete_note_index = -1}>No</button>
                </div>
            </div>
        </div>
    {/if}
    {#if delete_current_notes}
        <div class="flex items-center justify-center h-screen w-full absolute top-0 left-0" style="background: rgba(0, 0, 0, 0.3)">
            <div class="bg-stone-100 w-fit rounded-md">
                {#if deleting_current_notes}
                    <h5 class="p-5">Deleting...</h5>
                {:else}
                    <h5 class="p-5">Are you sure want to delete current notes?</h5>
                    <div class="p-5 pt-0 flex justify-center gap-2">
                        <button class="p-2 border-[1px] border-stone-500" on:click={() => deleteCurrentNotes()}>Yes</button>
                        <button class="p-2 border-[1px] border-stone-500" on:click={() => delete_current_notes = false}>No</button>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
    {#if create_note}
        <div class="flex items-center justify-center h-screen w-full absolute top-0 left-0" style="background: rgba(0, 0, 0, 0.3)">
            <div class="bg-stone-100 w-fit rounded-md">
                {#if creating_note}
                    <h5 class="p-5">Creating note...</h5>
                {:else}
                    <h5 class="p-5">Enter the title and the Youtube embed link for the note: </h5>
                    <input class="p-5 text-xl bg-transparent border-stone-500 m-5 border-[1px]" id="note-title" type="text" placeholder="Note title" />
                    <input class="p-5 text-xl bg-transparent border-stone-500 m-5 border-[1px]" id="note-yt-link" type="text" placeholder="Youtube embed link">
                    <div class="p-5 pt-0 flex justify-center gap-2">
                        <button class="p-2 border-[1px] border-stone-500" on:click={async () => await createNote()}>Yes</button>
                        <button class="p-2 border-[1px] border-stone-500" on:click={() => create_note = false}>No</button>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
    {#if explain_prompt}
        <div class="flex items-center justify-center h-screen w-full absolute top-0 left-0" style="background: rgba(0, 0, 0, 0.3)">
            <div class="bg-stone-100 w-fit rounded-md">
                {#if explaining_prompt}
                    <h5 class="p-5">Thinking...</h5>
                {:else}
                    <h5 class="p-5">Enter your prompt: </h5>
                    <input class="p-5 text-xl bg-transparent border-stone-500 m-5 border-[1px]" id="explanation-prompt" type="text" placeholder="Prompt" />
                    <div class="p-5 pt-0 flex justify-center gap-2">
                        <button class="p-2 border-[1px] border-stone-500" on:click={async () => await explainNote()}>Yes</button>
                        <button class="p-2 border-[1px] border-stone-500" on:click={() => explain_prompt = false}>No</button>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
    {#if explanation !== ""}
        <div class="flex items-center justify-center h-screen w-full absolute top-0 left-0" style="background: rgba(0, 0, 0, 0.3)">
            <div class="bg-stone-100 w-fit rounded-md">
                <h5 class="p-5">Explanation</h5>
                <div class="max-w-[700px] max-h-[900px] overflow-scroll m-5 p-5 border-stone-500 border-[1px]">
                    {@html explanation}
                </div>
                <div class="p-5 pt-0 flex justify-center gap-2">
                    <button class="p-2 border-[1px] border-stone-500" on:click={() => explanation = ""}>Close</button>
                </div>
            </div>
        </div>
    {/if}
    <div class="flex flex-col p-5 justify-between min-w-[300px] border-r-[1px] border-r-stone-500">
        <div class="grid gap-5">
            {#each notes as note}
                <button class="p-2 border-[1px] border-stone-500" on:click={() => { $current_note_document = note; $current_note = $current_note_document.data() }}>{note.data().title}</button>
            {/each}
        </div>
        <div class="grid gap-5">
            <button class="p-2 border-[1px] border-stone-500" on:click={() => create_note = true}>Add note</button>
            <button class="p-2 border-[1px] border-stone-500" on:click={async () => await getAuth(getApp()).signOut()}>Log Out</button>
        </div>
    </div>
    <div class="p-20 flex-1 max-h-screen overflow-y-scroll">
        <div class="yt-player-container flex items-center justify-center">
            <div id="yt-player"></div>
        </div>
        {#if $current_note}
            <h2>{$current_note.title}</h2>
            <button class="p-2 border-[1px] border-stone-500" on:click={toggleEditMode}>
                {#if edit_mode}
                    Save Or Exit From Edit Mode
                {:else}
                    Enter Edit Mode
                {/if}
            </button>
            <button class="text-stone-50 p-2 border-[1px] border-stone-500 border-t-0 border-r-0 bg-red-600" on:click={() => delete_current_notes = true}>Delete</button>
            {#if edit_mode}
                <button class="p-2 border-[1px] border-stone-500" on:click={addNoteForCurrentTimestamp}>
                    Add note for current timestamp
                </button>
            {/if}
            <div class="pt-5 flex flex-col gap-5">
                {#each $current_note_data as data, idx}
                    <div class="border-[1px] border-stone-500">
                        <div class="flex justify-end">
                            <button class="p-2 border-[1px] border-stone-500 border-t-0 border-r-0" on:click={async () => { await player.seekTo(data.timestamp, true); await player.playVideo() } }>{secondsToTime(data.timestamp)}</button>
                            <button class="p-2 border-[1px] border-stone-500 border-t-0 border-r-0" on:click={async () => { note_explanation_data = data?.markdown; explain_prompt = true }}>Explain</button>
                            <button class="text-stone-50 p-2 border-[1px] border-stone-500 border-t-0 border-r-0 bg-red-600" on:click={() => delete_note_index = idx}>Delete</button>
                        </div>
                        <div class="p-5">
                            {#if !edit_mode}
                                <div class="note-content p-5">
                                    {@html DOMPurify.sanitize(marked.parse(data?.markdown))}
                                </div>
                            {:else}
                                <textarea class="bg-transparent w-full border-[1px] border-stone-500 min-h-[300px] outline-none p-5 text-xl font-mono" on:change={(e) => updateNote(idx, e.target?.value)}>{data.markdown}</textarea>
                            {/if}
                        </div>
                    </div>
                {/each}
            </div>
        {:else}
            <h2 class="text-center">Create or click on one of the notes to see a note</h2>
        {/if}
    </div>
</div>