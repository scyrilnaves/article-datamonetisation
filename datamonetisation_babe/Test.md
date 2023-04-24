const HTTP_REMOTE_REQUEST_ASSET: &str = "http://vehicleapi.unice.cust.tasfrance.com/assettoken";
	const HTTP_REMOTE_REQUEST_ASSET_VALID: &str =
		"http://vehicleapi.unice.cust.tasfrance.com/assettoken";
	const HTTP_REMOTE_REQUEST_ASSET_SERVICE: &str =
		"http://radarapi.unice.cust.tasfrance.com/assetservicetoken";
	const HTTP_REMOTE_REQUEST_ASSET_SERVICE_VALID: &str =
		"http://radarapi.unice.cust.tasfrance.com/assetservicetoken";

		docker run --rm -it --name polkadot-ui -e WS_URL=ws://substrate-ws.unice.cust.tasfrance.com -p 80:80 jacogr/polkadot-js-apps:latest