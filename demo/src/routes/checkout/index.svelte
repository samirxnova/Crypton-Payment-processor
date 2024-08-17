<script lang="ts">
	import { selectedAccount, connected, chainData } from "svelte-web3";
	import Web3Modal from "$lib/Web3Modal.svelte";

	let totalAmount = 642.60;
</script>

<div class="bg-white">
	<!-- Background color split screen for large screens -->
	<div
		class="hidden lg:block fixed top-0 left-0 w-1/2 h-full bg-white"
		aria-hidden="true"
	/>
	<div
		class="hidden lg:block fixed top-0 right-0 w-1/2 h-full bg-indigo-900"
		aria-hidden="true"
	/>

	<div
		class="relative grid grid-cols-1 gap-x-16 max-w-7xl mx-auto lg:px-8 lg:grid-cols-2 lg:pt-16"
	>
		<h1 class="sr-only">Checkout</h1>

		<section
			aria-labelledby="summary-heading"
			class="bg-indigo-900 text-indigo-300 py-12 md:px-10 lg:max-w-lg lg:w-full lg:mx-auto lg:px-0 lg:pt-0 lg:pb-24 lg:bg-transparent lg:row-start-1 lg:col-start-2"
		>
			<div class="max-w-2xl mx-auto px-4 lg:max-w-none lg:px-0">
				<h2 id="summary-heading" class="sr-only">Order summary</h2>

				<dl>
					<dt class="text-sm font-medium">Amount due</dt>
					<dd class="mt-1 text-3xl font-extrabold text-white">${totalAmount}</dd>
				</dl>

				<ul
					class="text-sm font-medium divide-y divide-white divide-opacity-10"
				>
					<li class="flex items-start py-6 space-x-4">
						<img
							src="https://tailwindui.com/img/ecommerce-images/checkout-page-07-product-01.jpg"
							alt="Front of zip tote bag with white canvas, white handles, and black drawstring top."
							class="flex-none w-20 h-20 rounded-md object-center object-cover"
						/>
						<div class="flex-auto space-y-1">
							<h3 class="text-white">High Wall Tote</h3>
							<p>White and black</p>
							<p>15L</p>
						</div>
						<p class="flex-none text-base font-medium text-white">$570.00</p>
					</li>

					<!-- More products... -->
				</ul>

				<dl
					class="text-sm font-medium space-y-6 border-t border-white border-opacity-10 pt-6"
				>
					<div class="flex items-center justify-between">
						<dt>Subtotal</dt>
						<dd>$570.00</dd>
					</div>

					<div class="flex items-center justify-between">
						<dt>Shipping</dt>
						<dd>$25.00</dd>
					</div>

					<div class="flex items-center justify-between">
						<dt>Taxes</dt>
						<dd>$47.60</dd>
					</div>

					<div
						class="flex items-center justify-between border-t border-white border-opacity-10 text-white pt-6"
					>
						<dt class="text-base">Total</dt>
						<dd class="text-base">$642.60</dd>
					</div>
				</dl>
			</div>
		</section>

		<section
			aria-labelledby="payment-and-shipping-heading"
			class="py-16 lg:max-w-lg lg:w-full lg:mx-auto lg:pt-0 lg:pb-24 lg:row-start-1 lg:col-start-1"
		>
			<h2 id="payment-and-shipping-heading" class="sr-only">
				Payment and shipping details
			</h2>

			<form>
				<div class="max-w-2xl mx-auto px-4 lg:max-w-none lg:px-0">
					<div>
						<h3 id="contact-info-heading" class="text-lg font-medium text-gray-900">
							Contact information
						</h3>

						<div class="mt-6">
							<label for="email-address" class="block text-sm font-medium text-gray-700"
								>Email address</label
							>
							<div class="mt-1">
								<input
									type="email"
									id="email-address"
									name="email-address"
									autocomplete="email"
									class="block w-full border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
								/>
							</div>
						</div>
					</div>

					<div class="mt-10">
						<h3 class="text-lg font-medium text-gray-900">Shipping address</h3>

						<div class="mt-6 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-3">
							<div class="sm:col-span-3">
								<label for="address" class="block text-sm font-medium text-gray-700"
									>Address</label
								>
								<div class="mt-1">
									<input
										type="text"
										id="address"
										name="address"
										autocomplete="street-address"
										class="block w-full border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
									/>
								</div>
							</div>

							<div>
								<label for="city" class="block text-sm font-medium text-gray-700">City</label>
								<div class="mt-1">
									<input
										type="text"
										id="city"
										name="city"
										autocomplete="address-level2"
										class="block w-full border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
									/>
								</div>
							</div>

							<div>
								<label for="region" class="block text-sm font-medium text-gray-700"
									>State / Province</label
								>
								<div class="mt-1">
									<input
										type="text"
										id="region"
										name="region"
										autocomplete="address-level1"
										class="block w-full border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
									/>
								</div>
							</div>

							<div>
								<label for="postal-code" class="block text-sm font-medium text-gray-700"
									>Postal code</label
								>
								<div class="mt-1">
									<input
										type="text"
										id="postal-code"
										name="postal-code"
										autocomplete="postal-code"
										class="block w-full border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
									/>
								</div>
							</div>
						</div>
					</div>

					<div class="mt-10">
						<h3 class="text-lg font-medium text-gray-900">Billing information</h3>

						<div class="mt-6 flex items-center">
							<input
								id="same-as-shipping"
								name="same-as-shipping"
								type="checkbox"
								checked
								class="h-4 w-4 border-gray-300 rounded text-indigo-600 focus:ring-indigo-500"
							/>
							<div class="ml-2">
								<label for="same-as-shipping" class="text-sm font-medium text-gray-900"
									>Same as shipping information</label
								>
							</div>
						</div>
					</div>

					<div class="mt-10">
						<h3 class="text-lg font-medium text-gray-900 mb-4">Payment</h3>
						<Web3Modal {totalAmount}/>
					</div>

				</div>
			</form>
		</section>
	</div>
</div>
