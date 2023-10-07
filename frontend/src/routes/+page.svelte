<script lang="ts">
    import {AuthenticatedUser} from "../stores/user";
    import {goto} from "$app/navigation";
    import {onDestroy} from "svelte";

    const unsubscribe = AuthenticatedUser.subscribe((user) => {
        if (user) {
            goto('/notes', { replaceState: true })
        }
    });

    onDestroy(() => {
        unsubscribe();
    });
</script>

<svelte:head>
    <title>Smart Totoro</title>
</svelte:head>

<section class="relative p-28 min-h-screen flex flex-col gap-10 justify-center">
    <h1>
        <span class="name">Smart Totoro</span> says <i>Konnichiwa</i> ٩(＾◡＾)۶
    </h1>
    <h3>Your personal AI assisted study notebook</h3>
    <a class="w-fit bg-accent text-background text-xl pl-7 pr-7 rounded-xl pt-5 pb-5" href="/login">Get started</a>
    <img class="absolute h-fit right-0 top-80 w-[600px]" alt="Totoro" src="totoro2.png" />
</section>

<section class="flex justify-between gap-5 flex-col sm:flex-row">
    <h2>Tired of looking back and forth between <u>notes</u> and <u>lectures</u>?</h2>
    <p class="text-2xl font-semibold">
        Fear not! <span class="name">Smart Totoro</span> is here to help you. With its feature rich experience, taking
        notes,
        understanding them, referring to lectures are as easier as <i><b>frying an egg</b></i>(no joke!).
    </p>
</section>

<section class="relative flex justify-between gap-5 flex-col sm:flex-row">
    <h2>Don't believe us?</h2>
    <div class="flex flex-col gap-10">
        <h4>Here are the steps to fry an egg compared to steps to take note of a video:</h4>

        <div class="flex gap-10 align-middle bg-secondary rounded-md p-5">
            <h2 class="text-accent">1</h2>
            <h5 class="flex flex-col justify-center">
                <span>Frying egg: Get a frying pan</span>
                <span><span class="name">Smart Totoro</span>: Insert the YouTube link of the lecture</span>
            </h5>
        </div>

        <div class="flex gap-10 align-middle bg-secondary rounded-md p-5">
            <h2 class="text-accent">2</h2>
            <h5 class="flex flex-col justify-center">
                <span>Frying egg: Heat up the stove and wait until the frying pan's hot enough</span>
                <span><span class="name">Smart Totoro</span>: Go to the part for which you want to add notes</span>
            </h5>
        </div>

        <div class="flex gap-10 align-middle bg-secondary rounded-md p-5">
            <h2 class="text-accent">3</h2>
            <h5 class="flex flex-col justify-center">
                <span>
                    Frying egg: Crack an egg, drop its content onto the frying pan and wait until it turns semi white
                </span>
                <span><span class="name">Smart Totoro</span>: Right click and click on "Add notes"</span>
            </h5>
        </div>

        <div class="flex gap-10 align-middle bg-secondary rounded-md p-5">
            <h2 class="text-accent">4</h2>
            <h5 class="flex flex-col justify-center">
                <span>Frying egg: Add salt and pepper</span>
                <span><span class="name">Smart Totoro</span>: Write down your notes</span>
            </h5>
        </div>

        <div class="flex gap-10 align-middle bg-secondary rounded-md p-5">
            <h2 class="text-accent">5</h2>
            <h5 class="flex flex-col justify-center">
                <span>Frying egg: You're done!</span>
                <span><span class="name">Smart Totoro</span>: You're done!</span>
            </h5>
        </div>
    </div>
    <img class="absolute left-20 top-72 w-[600px]" alt="Totoro" src="totoro1.png" />
</section>

<section class="flex flex-col gap-10">
    <h2 class="text-center">Features</h2>
    <div class="grid gap-5 grid-cols-5">
        <div class="flex flex-col items-center justify-center bg-[#60b5c2] col-span-3 rounded-xl p-20">
            <h3>Smart AI Assistance</h3>
            <p class="text-xl text-center text-text font-semibold">Can't understand part of your note? Select that portion, right click and click "Explain"!</p>
        </div>
        <div class="flex flex-col items-center justify-center bg-[#394063] col-span-2 rounded-xl p-20">
            <h3 class="text-stone-100">Markdown Support</h3>
            <p class="text-xl text-center text-stone-100 font-semibold">Easily format your notes with Markdown syntax</p>
        </div>
        <div class="flex flex-col items-center justify-center bg-[#b97b77] col-span-2 rounded-xl p-20">
            <h3>KaTeX Support</h3>
            <p class="text-xl text-center text-text font-semibold">Write mathematical expressions easily with KaTeX</p>
        </div>
        <div class="flex flex-col items-center justify-center bg-[#5a7285] col-span-3 rounded-xl p-20">
            <h3 class="text-center text-stone-100">Notes For Specific Portion Of Lecture</h3>
            <p class="text-xl text-center text-stone-100 font-semibold">Easily keep track of the part of the lecture from which a part of note comes from</p>
        </div>
    </div>
</section>

<section class="text-center flex flex-col gap-10">
    <h2>What are you waiting for?</h2>
    <div>
        <a class="bg-accent text-background text-xl pl-7 pr-7 rounded-xl pt-5 pb-5" href="/login">Log In or Register</a>
    </div>
</section>

<section>
    <div class="relative bg-stone-900 rounded-xl p-10">
        <h3><span class="name">Smart Totoro</span></h3>
        <a class="text-xl text-stone-500 font-semibold underline" href="https://github.com/mdgaziur/smart-totoro">Source Code</a>
        <h6 class="text-center text-stone-500">&copy; Copyright 2023 - Team Segmentation Fault: 11</h6>
        <img class="bottom-0 right-10 absolute w-[300px]" alt="Totoro peeking cute >w<" src="totoro3.png" />
    </div>
</section>