import { For, Show, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { Login } from "./components/login";

type VecRun = Array<{ distance: number; time: string }>;

function App() {
  const [user, setUser] = createSignal<string>("");
  const [distance, setDistance] = createSignal(0);
  const [time, setTime] = createSignal("");
  const [runs, setRuns] = createSignal<VecRun>([]);
  const [weeklyRuns, setWeeklyRuns] = createSignal<VecRun>([]);
  async function saveRun() {
    await invoke("save_run", { distance: distance(), time: time() });
    setDistance(0);
    setTime("");
    getRuns();
    getWeeklyRuns();
  }
  async function getRuns() {
    let runs: VecRun = await invoke("get_runs", {});
    setRuns(runs);
  }

  async function getWeeklyRuns() {
    let runs: VecRun = await invoke("get_weekly_runs", {});
    setWeeklyRuns(runs);
  }

  return (
    <div class="h-screen w-screen flex flex-col justify-center items-center gap-2">
      <Show when={user()} fallback={<Login setUser={setUser} />}>
        <div>{user()}</div>
      </Show>
      <label for="dist">Distance</label>
      <input
        type="numeric"
        name="dist"
        placeholder="0"
        class="text-black px-2 py-4 rounded-xl"
        value={distance()}
        oninput={(e) => setDistance(+e.currentTarget.value)}
      />
      <label for="time">Time</label>
      <input
        type="text"
        name="time"
        placeholder="hh:mm:ss"
        class="text-black px-2 py-4 rounded-xl"
        value={time()}
        oninput={(e) => setTime(e.currentTarget.value)}
      />
      <button
        class="mt-2 px-10 py-2 rounded-xl bg-white text-black"
        onclick={() => saveRun()}
      >
        Save Run
      </button>
      <For each={runs()}>{(run) => <RunInfo {...run} />}</For>
      <For each={weeklyRuns()}>{(run) => <RunInfo {...run} />}</For>
    </div>
  );
}

type RunProps = {
  distance: number;
  time: string;
};

const RunInfo = (props: RunProps) => {
  return (
    <div class="flex items-center justify-center gap-2">
      <div>Distance: {props.distance}</div>
      <div>Time: {props.time}</div>
    </div>
  );
};

export default App;
