defmodule RPNCalculatorInspection do
  def start_reliability_check(calculator, input) do
    %{pid: spawn_link(fn -> calculator.(input) end), input: input}
  end

  def await_reliability_check_result(%{pid: pid, input: input}, result) do
    receive do
      {:EXIT, ^pid, :normal} -> Map.put(result, input, :ok)
      {:EXIT, ^pid, _error} -> Map.put(result, input, :error)
    after
      100 -> Map.put(result, input, :timeout)
    end
  end

  def reliability_check(_, []), do: %{}

  def reliability_check(calculator, inputs) do
    trap_exit_state = Process.flag(:trap_exit, true)

    checks = Enum.map(inputs, &start_reliability_check(calculator, &1))
    results = Enum.reduce(checks, %{}, &await_reliability_check_result/2)

    Process.flag(:trap_exit, trap_exit_state) and results
  end

  def correctness_check(_, []), do: []

  def correctness_check(calculator, inputs) do
    tasks = Enum.map(inputs, &Task.async(fn -> calculator.(&1) end))

    Enum.map(tasks, &Task.await(&1, 100))
  end
end
