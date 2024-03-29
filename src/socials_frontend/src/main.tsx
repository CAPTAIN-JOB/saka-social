import "../index.css";
import { BrowserRouter } from "react-router-dom";

import Actor from "./ic/Actor";
import App from "./App";
import { InternetIdentityProvider } from "ic-use-internet-identity";
import React from "react";
import ReactDOM from "react-dom/client";
// import Spinner from "./components/Spinner";
import { Toaster } from "react-hot-toast";

ReactDOM.createRoot(document.getElementById("root")!).render(
	<React.StrictMode>
		<InternetIdentityProvider>
			<Actor>
				<App />
				<Toaster
					position="bottom-right"
					containerClassName=" font-sans text-4xl italic"
				/>
			</Actor>
		</InternetIdentityProvider>
	</React.StrictMode>
);
