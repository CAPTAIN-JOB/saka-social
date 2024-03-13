import { useInternetIdentity } from "ic-use-internet-identity";

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
		"flex px-5 font-bold text-white bg-blue-500 rounded cursor-pointer h-9 md:h-16 hover:bg-blue-700 disabled:bg-blue-500/20 disabled:hover:bg-blue-500/20";

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
