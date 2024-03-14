import { useEffect, useState } from "react";
import "../index.css";

import { LoginButton } from "./components/LoginButton";
import Principal from "./components/Principal";
import { useBackend } from "./ic/Actor";
import { useInternetIdentity } from "ic-use-internet-identity";

function App() {
	const { identity } = useInternetIdentity();
	const { actor: backend } = useBackend();
	const [principal, setPrincipal] = useState<string>();

	// Clear the principal when the identity is cleared
	useEffect(() => {
		if (!identity) setPrincipal(undefined);
	}, [identity]);

	// Get the principal from the backend when an identity is available
	useEffect(() => {
		if (identity && backend && !principal) {
			backend.whoami().then((p) => setPrincipal(p));
		}
	}, [backend, identity, principal]);

	return (
		<div className=" min-h-screen flex  flrx row justify-center items-center p-24 text-sm italic md:items-start md:gap-10 md:text-sm">
			<div className="text-center">
				{identity ? "You are logged in." : "You are not logged in."}
			</div>
			<LoginButton />
			<Principal principal={principal} />
		</div>
	);
}

export default App;
