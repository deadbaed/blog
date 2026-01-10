{
  headless ? false,
}:

(import ./. { inherit headless; }).productionShell
