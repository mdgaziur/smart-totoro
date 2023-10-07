<script lang="ts">
    import {getApp} from "firebase/app"
    import {getAuth, signInWithPopup, GoogleAuthProvider} from "firebase/auth";
    import {AuthenticatedUser} from "../../stores/user";
    import {goto} from "$app/navigation";
    import {onDestroy} from "svelte";

    const unsubscribe = AuthenticatedUser.subscribe((user) => {
        if (user) {
            goto('/notes', { replaceState: true })
        }
    });

    function login() {
        signInWithPopup(getAuth(getApp()), new GoogleAuthProvider())
            .then((result) => {
                AuthenticatedUser.set(result.user);
            }).catch(() => {
                alert("Failed to log in with Google")
        })
    }

    onDestroy(() => {
        unsubscribe();
    });
</script>

<svelte:head>
    <title>Smart Totoro - Login</title>
</svelte:head>

<section class="bg-secondary text-center">
    <h3>Login</h3>
    <button class="p-5 bg-accent text-secondary rounded-xl" on:click={login}>Sign in with Google</button>
</section>