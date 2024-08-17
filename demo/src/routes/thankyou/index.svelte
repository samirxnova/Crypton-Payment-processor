<script lang="ts">
	import { web3, connected } from "svelte-web3";
	import { page } from "$app/stores";

	const tx_id = $page.url.searchParams.get("tx_id");
	let state = "pending";

	$: checkIntervalId =
		$connected &&
		setInterval(() => {
			if (state === "complete") {
				clearInterval(checkIntervalId);
			} else {
				loadTx(tx_id);
			}
		}, 3000);

	async function loadTx(tx_id: string) {
		try {
			const {transactionIndex} = await $web3.eth.getTransaction(tx_id);
			if (transactionIndex) {
				state = "complete";
			}
		} catch (err) {
			console.error("Caught error when loading transaction" + err);
		}
	}
</script>

<div class="bg-white">
	<div class="max-w-7xl mx-auto py-16 px-4 sm:py-24 sm:px-6 lg:px-8">
		<div class="text-center">
			<h2 class="text-base font-semibold text-indigo-600 tracking-wide uppercase">
				Thank you
			</h2>
			<p
				class="mt-1 text-4xl font-extrabold text-gray-900 sm:text-5xl sm:tracking-tight lg:text-6xl"
			>
				Your order is {state}.
			</p>
			{#if state === "pending"}
				<p class="max-w-xl mt-5 mx-auto text-xl text-gray-500">
					your order is processing, we will notify you when complete!
				</p>
			{:else}
				<p class="max-w-xl mt-5 mx-auto text-xl text-gray-500">
					Your transaction is complete, expect your orders within a few days!
				</p>
			{/if}
		</div>
	</div>
</div>
