import { invoke } from "@tauri-apps/api/tauri";
import { Setter, Show, createSignal } from "solid-js";

type LoginProps = {
  setUser: Setter<string>;
};

export function Login(props: LoginProps) {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [newUser, setNewUser] = createSignal(false);

  async function loginUser() {
    let token: String = await invoke("login", {
      credentials: { email: email(), pass: password() },
    });
    console.log(token);
  }
  async function signUpUser() {
    let token: String = await invoke("signup", {
      credentials: { email: email(), pass: password() },
    });
    console.log(token);
  }
  return (
    <div class="flex flex-col items-center justify-center gap-2">
      <input
        placeholder="Email"
        value={email()}
        oninput={(e) => setEmail(e.currentTarget.value)}
        class="rounded-xl px-2 py-4 text-black"
      />
      <input
        placeholder="Password"
        value={password()}
        oninput={(e) => setPassword(e.currentTarget.value)}
        class="rounded-xl px-2 py-4 text-black"
      />
      <Show
        when={!newUser()}
        fallback={
          <div class="flex flex-col items-center justify-center gap-2">
            <button
              onclick={() => {
                props.setUser(email());
                signUpUser();
              }}
            >
              Sign Up
            </button>
            <button
              onclick={() => {
                setNewUser(false);
              }}
            >
              Back to Login
            </button>
          </div>
        }
      >
        <button
          onclick={() => {
            props.setUser(email());
            loginUser();
          }}
        >
          Log In
        </button>
        <button
          onclick={() => {
            setNewUser(true);
          }}
        >
          Create an Account
        </button>
      </Show>
    </div>
  );
}
