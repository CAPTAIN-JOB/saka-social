import { useInternetIdentity } from "ic-use-internet-identity";
import { twMerge } from "tailwind-merge";
export function LoginButton() {
	const { isLoggingIn, login, clear, identity } = useInternetIdentity();

	// If the user is logged in, clear the identity. Otherwise, log in.
	function handleClick() {
		if (identity) {
			clear();
		} else {
			login();
		}
	}

	let className =
		"font-bold font-sans text-black bg-blue-500 rounded cursor-pointer  px-4 py-4 hover:bg-blue-700 disabled:bg-blue-500/20 disabled:hover:bg-blue-500/20";
	className = isLoggingIn
		? twMerge(className, "cursor-wait")
		: twMerge(className, "cursor-pointer");
	const text = () => {
		if (identity) {
			return "Logout";
		} else if (isLoggingIn) {
			return <>Logging in</>;
		}
		return "Login";
	};

	return (
		<button onClick={handleClick} className={className} disabled={isLoggingIn}>
			{text()}
		</button>
	);
}
