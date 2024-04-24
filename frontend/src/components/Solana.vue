<template>
    <div class="w-full max-w-full px-2 py-2 sm:px-0"></div>
</template>

<script setup type="ts">
import * as web3 from "@solana/web3.js";
// create an empty transaction
const transaction = new web3.Transaction();
// add a hello world program instruction to the transaction
transaction.add(
    new web3.TransactionInstruction({
        keys: [],
        programId: new web3.PublicKey(
            "8PEg6uEYMhC4gyVJkre8MYZmvyzbitnRX1M8369a4g25",
        ),
    }),
);
</script>
