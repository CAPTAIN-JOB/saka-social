import "./index.css";

import Actor from "./ic/Actor";
import App from "./App";
import { InternetIdentityProvider } from "ic-use-internet-identity";
import React from "react";
import ReactDOM from "react-dom/client";

ReactDOM.createRoot(document.getElementById("root")!).render(
	<React.StrictMode>
		<InternetIdentityProvider>
			<Actor>
				<App />
			</Actor>
		</InternetIdentityProvider>
	</React.StrictMode>
);
