import { invoke } from "@tauri-apps/api/tauri";
import { Setter, createSignal } from "solid-js";

type LoginProps = {
  setUser: Setter<string>;
};

export function Login(props: LoginProps) {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");

  async function login_user() {
    let token: String = await invoke("login", {
      credentials: { email: email(), pass: password() },
    });
    console.log(token);
  }
  return (
    <div class="flex flex-col items-center justify-center gap-4">
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
      <button
        onclick={() => {
          props.setUser(email());
          login_user();
        }}
      >
        Log in
      </button>
    </div>
  );
}
