<script lang="ts">
    import "../app.scss";

    // Import the functions you need from the SDKs you need
    import {getApp, initializeApp} from "firebase/app";
    import {getAuth} from "firebase/auth";
    import {AuthenticatedUser} from "../stores/user";
    // TODO: Add SDKs for Firebase products that you want to use
    // https://firebase.google.com/docs/web/setup#available-libraries

    // Your web app's Firebase configuration
    export const firebaseConfig = {
        apiKey: "AIzaSyD3CJuC7-TFQ7yaB3N540Sqajuk6T6tEFk",
        authDomain: "smart-totoro.firebaseapp.com",
        projectId: "smart-totoro",
        storageBucket: "smart-totoro.appspot.com",
        messagingSenderId: "163836300242",
        appId: "1:163836300242:web:3edfc19937a27fd53d160b"
    };

    initializeApp(firebaseConfig);
    getAuth(getApp()).onAuthStateChanged((user) => {
        AuthenticatedUser.set(user);
    });
</script>

<slot />