<script lang="ts">
	import {
		defaultEvmStores,
		web3,
		selectedAccount,
		connected,
		chainData,
	} from "svelte-web3";
	import Web3Modal from "web3modal";
	import Decimal from "decimal.js";
	import { goto } from "$app/navigation";

	import { variables } from "$lib/variables";
	import { chains } from "../../../data/chains";
	// import CoinbaseWalletSDK from '@coinbase/wallet-sdk'
	const WalletConnectProvider = window.WalletConnectProvider.default;
	// import WalletConnectProvider from "@walletconnect/web3-provider";
	const disable = () => defaultEvmStores.disconnect();
	let web3Modal;
	let symbolPrice = new Decimal(1);
	export let totalAmount: number;
	const connectWallet = async () => {
		if (web3Modal) web3Modal.clearCachedProvider();
		web3Modal = new Web3Modal({
			cacheProvider: false,
			providerOptions: {
				walletconnect: {
					package: WalletConnectProvider,
					options: { infuraId: variables.infuraId },
				},
				// coinbasewallet: {
				//   package: CoinbaseWalletSDK,
				//   options: {
				//     appName: "crypton",
				//     infuraId: "7b305244a8dc444f9cdd411c1a0b10cf"
				//   }
				// },
			},
			disableInjectedProvider: false,
		});
		const provider = await web3Modal.connect();
		defaultEvmStores.setProvider(provider);
	};

	let chainDropdownMenuOpen = false;
	function toggleChainDropdownMenu() {
		chainDropdownMenuOpen = !chainDropdownMenuOpen;
	}

	function getChain(desiredChainId: number): any {
		return (
			chains.filter(({ chainId }) => desiredChainId == chainId)[0] || chains[0]
		);
	}
	$: fetch(
		`https://api.covalenthq.com/v1/pricing/tickers/?quote-currency=USD&format=JSON&tickers=${
			$chainData?.nativeCurrency?.symbol || "ETH"
		}&key=${variables.covalentApiKey}` 
	)
		.then((response) => response.json())
		.then((json) => {
			const { data, error } = json;
			if (error) {
				console.log(json.error_code)
				console.log(json.error_message)
			} else {
				const { items } = data;
				if (items[0]) {
					symbolPrice = new Decimal(items[0].quote_rate);
				} else {
					symbolPrice = new Decimal(1)
				}
			}
		});
	let transactions: string[];
  $: $connected && $web3.eth.currentProvider.on('message', function(message: string) {
    console.log(message)  
    transactions = [...transactions, message]
  });

	async function switchNetwork(chainId: number) {
		if ($chainData.chainId !== chainId) {
			try {
				await $web3.eth.currentProvider.request({
					method: "wallet_switchEthereumChain",
					params: [{ chainId: $web3.utils.toHex(chainId) }],
				});
			} catch (err) {
				console.log(err);
				const chain = getChain(chainId);

				// This error code indicates that the chain has not been added to MetaMask
				if (err.code === 4902) {
					await $web3.eth.currentProvider.request({
						method: "wallet_addEthereumChain",
						params: [
							{
								chainName: chain.name,
								chainId: $web3.utils.toHex(chainId),
								nativeCurrency: chain.nativeCurrency,
								rpcUrls: chain.rpc,
							},
						],
					});
				}
			}
		}
		chainDropdownMenuOpen = false;
	}

	const pay = async () => {
		try {
			const gasPriceNum = await $web3.eth.getGasPrice()
			const gasPrice = $web3.utils.toHex(gasPriceNum)
			const tx_id = await $web3.eth.currentProvider.request({
				method: "eth_sendTransaction",
				params: [{
					from: $selectedAccount,
					to: $selectedAccount,
					value: $web3.utils.toHex(gasPriceNum * 1000),
					gasPrice
			  }],
			});
			goto(`/thankyou?tx_id=${tx_id}`)
		} catch (err) {
			console.log(err);
		}
	};
</script>

{#if $web3.version}
	{#if $connected}
		<div
			class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0 mb-4"
		>
			<!-- Chain dropdown -->
			<div class="relative">
				<div>
					<div class="flex">
						<p class="my-auto mr-6">Payment method:</p>
						<button
							type="button"
							class="inline-flex items-center px-6 py-3 border border-transparent shadow-sm text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
							on:click={toggleChainDropdownMenu}
						>
							{$chainData?.name} with {$chainData.nativeCurrency.symbol}
							<img
								class="h-8 w-8 rounded-full mx-4"
								src={getChain($chainData?.chainId).assetSrc || ""}
								alt=""
							/>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-6 w-6"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
								stroke-width="2"
							>
								<path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
							</svg>
						</button>

						<!--
            Dropdown menu, show/hide based on menu state.

            Entering: "transition ease-out duration-100"
              From: "transform opacity-0 scale-95"
              To: "transform opacity-100 scale-100"
            Leaving: "transition ease-in duration-75"
              From: "transform opacity-100 scale-100"
              To: "transform opacity-0 scale-95"
          -->
						<div
							class="{chainDropdownMenuOpen
								? ''
								: 'hidden'} origin-top-right absolute right-0 mt-12 w-48 rounded-md shadow-lg py-1 bg-white ring-1 ring-black ring-opacity-5 focus:outline-none z-50"
							role="menu"
							aria-orientation="vertical"
							aria-labelledby="user-menu-button"
							tabindex="-1"
						>
							{#each chains as { assetSrc, name, chainId }}
								<button
									class="block px-4 py-2 text-sm text-gray-700"
									role="menuitem"
									tabindex="-1"
									id="user-menu-item-0"
									on:click|preventDefault={() => switchNetwork(chainId)}
								>
									<div class="flex">
										<img class="h-8 w-8 rounded-full" src={assetSrc || ""} alt="" />
										<p class="m-auto">{name}</p>
									</div>
								</button>
							{/each}
						</div>
					</div>
					<div class="flex mt-4">
						<p class="my-auto mr-6">Balance:</p>
						<div
							class="inline-flex items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-indigo-600 bg-white hover:bg-gray-50"
						>
							{#if $selectedAccount}
								{#await $web3.eth.getBalance($selectedAccount)}
									{"pending"}
								{:then value}
									{$web3.utils.fromWei(value)}
								{/await}
								{$chainData.nativeCurrency.symbol}
							{/if}
							<img
								class="h-8 w-8 rounded-full mx-4"
								src={getChain($chainData?.chainId).assetSrc || ""}
								alt=""
							/>
						</div>
					</div>
					<div class="flex mt-4">
						<p class="my-auto mr-6">Amount due:</p>
						<div
							class="inline-flex items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-indigo-600 bg-white hover:bg-gray-50"
						>
							{new Decimal(totalAmount).dividedBy(symbolPrice).toFixed(4)}
							{$chainData.nativeCurrency.symbol}
							<img
								class="h-8 w-8 rounded-full mx-4"
								src={getChain($chainData?.chainId).assetSrc || ""}
								alt=""
							/>
						</div>
					</div>
					<div class="flex mt-4">
						<p class="my-auto mr-6">Network fee:</p>
						<div
							class="inline-flex items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-indigo-600 bg-white hover:bg-gray-50"
						>
							{#if $selectedAccount}
								{#await $web3.eth.getGasPrice()}
									{"pending"}
								{:then value}
									{$web3.utils.fromWei(value, 'gwei')}
								{/await}
								"gwei (10e-15)"
							{/if}
							<img
								class="h-8 w-8 rounded-full mx-4"
								src={getChain($chainData?.chainId).assetSrc || ""}
								alt=""
							/>
						</div>
					</div>
				</div>
			</div>
		</div>

		<button
			class="bg-indigo-600 border border-transparent rounded-md shadow-sm py-2 px-4 text-sm font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-50 focus:ring-indigo-500"
			on:click|preventDefault={pay}>Pay now</button
		>
		<button
			class="inline-flex items-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-indigo-600 bg-white hover:bg-gray-50"
			on:click={disable}>Disconnect wallet</button
		>
	{:else}
		<button
			class="bg-indigo-600 border border-transparent rounded-md shadow-sm py-2 px-4 text-sm font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-50 focus:ring-indigo-500"
			on:click|preventDefault={connectWallet}>Connect your wallet</button
		>
	{/if}
{/if}
