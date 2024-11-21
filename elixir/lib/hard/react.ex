defmodule React do
  use GenServer

  @opaque cells :: pid

  @type cell :: {:input, String.t(), any} | {:output, String.t(), [String.t()], fun()}

  @doc """
  Start a reactive system
  """
  @spec new(cells :: [cell]) :: {:ok, pid}
  def new(cells) do
    state =
      cells
      |> Enum.reduce(%{data: %{}, computed: %{}, callbacks: %{}}, fn
        {:input, key, value}, acc ->
          data = Map.put(acc.data, key, value)
          %{acc | data: data}

        {:output, key, inputs, fun}, acc ->
          input_values = Enum.map(inputs, &acc.data[&1])

          data = Map.put(acc.data, key, apply(fun, input_values))
          computed = Map.put(acc.computed, key, {inputs, fun})

          %{acc | data: data, computed: computed}
      end)

    GenServer.start_link(__MODULE__, state)
  end

  @doc """
  Return the value of an input or output cell
  """
  @spec get_value(cells :: pid, cell_name :: String.t()) :: any()
  def get_value(cells, cell_name) do
    GenServer.call(cells, {:get_value, cell_name})
  end

  @doc """
  Set the value of an input cell
  """
  @spec set_value(cells :: pid, cell_name :: String.t(), value :: any) :: :ok
  def set_value(cells, cell_name, value) do
    GenServer.cast(cells, {:set_value, cell_name, value})
  end

  @doc """
  Add a callback to an output cell
  """
  @spec add_callback(
          cells :: pid,
          cell_name :: String.t(),
          callback_name :: String.t(),
          callback :: fun()
        ) :: :ok
  def add_callback(cells, cell_name, callback_name, callback) do
    GenServer.cast(cells, {:set_callback, cell_name, callback_name, callback})
  end

  @doc """
  Remove a callback from an output cell
  """
  @spec remove_callback(cells :: pid, cell_name :: String.t(), callback_name :: String.t()) :: :ok
  def remove_callback(cells, cell_name, callback_name) do
    GenServer.cast(cells, {:remove_callback, cell_name, callback_name})
  end

  @impl true
  def init(state) do
    {:ok, state}
  end

  @impl true
  def handle_cast({:set_callback, key, callback, fun}, %{callbacks: callbacks} = state) do
    callbacks =
      Map.update(callbacks, key, [{callback, fun}], fn callbacks ->
        [{callback, fun} | callbacks]
      end)

    {:noreply, %{state | callbacks: callbacks}}
  end

  @impl true
  def handle_cast({:remove_callback, key, callback}, %{callbacks: callbacks} = state) do
    cell_callbacks =
      Enum.reject(callbacks[key], fn {stored_callback, _} -> stored_callback == callback end)

    callbacks = Map.put(callbacks, key, cell_callbacks)

    {:noreply, %{state | callbacks: callbacks}}
  end

  @impl true
  def handle_cast({:set_value, key, value}, %{data: data, computed: computed} = state) do
    new_data =
      data
      |> Map.put(key, value)
      |> compute([key], computed)

    send_callbacks(data, new_data, state.callbacks)

    {:noreply, %{state | data: new_data, computed: computed}}
  end

  defp compute(values, [], _computed), do: values

  defp compute(values, input_keys, computed) do
    invalid_computed_items =
      Enum.filter(computed, fn {_computed_key, {inputs, _fun}} ->
        Enum.any?(input_keys, &(&1 in inputs))
      end)

    new_values =
      Enum.reduce(invalid_computed_items, values, fn {key, {inputs, fun}}, acc ->
        input_values = Enum.map(inputs, &acc[&1])

        computed_value = apply(fun, input_values)

        Map.put(acc, key, computed_value)
      end)

    new_input_keys = for {key, _} <- invalid_computed_items, key not in input_keys, do: key

    compute(new_values, new_input_keys, computed)
  end

  defp send_callbacks(data, new_data, callbacks) do
    for {key, value} <- new_data,
        data[key] != value,
        callback <- callbacks[key] || [],
        callback do
      {name, fun} = callback
      fun.(name, value)
    end
  end

  @impl true
  def handle_call({:get_value, key}, _from, %{data: data} = state) do
    {:reply, data[key], state}
  end
end
